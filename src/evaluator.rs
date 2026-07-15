//! The `evaluator` module is responsible for executing the AST.
//! It walks the AST, evaluates expressions, and executes statements to produce results.
//! It also manages the runtime environment and handles runtime errors.

use crate::{
    ast::{Expr, Stmt},
    callable::{Arity, Callable, ID_FUNC, PRINT_FUNC, TYPE_FUNC, UserDefinedFn},
    environment::Environment,
    error::{ErrorReporter, RuntimeError},
    object::Object,
    token::{Literal, SourceLocation, Token, TokenKind},
};

/// Represents the control flow of the interpreter, used to handle returns from functions
/// and propagate runtime errors.
enum ControlFlow {
    Return { keyword: Token, value: Object },
    Error(RuntimeError),
}

/// The `Evaluator` struct is responsible for interpreting the AST.
/// It traverses the AST, evaluates expressions, and executes statements in a given environment.
pub struct Evaluator<'err> {
    error_reporter: &'err mut ErrorReporter,
}

impl<'err> Evaluator<'err> {
    /// Creates a new `Evaluator` instance.
    ///
    /// # Arguments
    ///
    /// * `error_reporter` - A mutable reference to an `ErrorReporter` for reporting runtime errors.
    pub fn new(error_reporter: &'err mut ErrorReporter) -> Self {
        Evaluator { error_reporter }
    }

    /// Interprets a list of statements in a given environment.
    ///
    /// This is the main entry point for executing the program's AST. It sets up the initial
    /// environment with native functions and then executes each statement sequentially.
    /// If `interactive` mode is true, it will print the result of expression statements.
    ///
    /// # Arguments
    ///
    /// * `statements` - A reference to a vector of `Stmt` to be executed.
    /// * `environment` - A mutable reference to the `Environment` in which to execute the statements.
    /// * `interactive` - A boolean indicating whether the interpreter is running in interactive mode.
    ///
    /// # Returns
    ///
    /// An `Option<Object>` containing the last evaluated value in interactive mode, or `None` otherwise.
    /// If a runtime error occurs, it reports the error and returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use whipsnake::{error::ErrorReporter, lexer::Lexer, parser::Parser, evaluator::Evaluator, environment::Environment, object::Object};
    ///
    /// let mut error_reporter = ErrorReporter::new();
    /// let mut lexer = Lexer::new(&mut error_reporter);
    /// let tokens = lexer.lex("x = 10\nprint(x)");
    /// let mut token_iter = tokens.into_iter().peekable();
    /// let mut parser = Parser::new(&mut error_reporter);
    /// let statements = parser.parse(&mut token_iter);
    ///
    /// let mut evaluator = Evaluator::new(&mut error_reporter);
    /// let mut environment = Environment::new_global();
    /// let result = evaluator.interpret(&statements, &mut environment, true);
    ///
    /// assert_eq!(result, Some(Object::None)); // print returns None
    /// ```
    pub fn interpret(
        &mut self,
        statements: &Vec<Stmt>,
        environment: &mut Environment,
        interactive: bool,
    ) -> Option<Object> {
        environment.define(
            PRINT_FUNC.name.to_string(),
            Object::Function(Callable::Native(PRINT_FUNC)),
        );

        environment.define(
            TYPE_FUNC.name.to_string(),
            Object::Function(Callable::Native(TYPE_FUNC)),
        );

        environment.define(
            ID_FUNC.name.to_string(),
            Object::Function(Callable::Native(ID_FUNC)),
        );

        match self.execute_statements(statements, environment, interactive) {
            Ok(last_value) => Some(last_value),
            Err(c) => {
                let error = match c {
                    ControlFlow::Error(e) => e,
                    ControlFlow::Return { keyword, value: _value } => RuntimeError::RuntimeError(
                        SourceLocation { line: keyword.line },
                        "got return statement outside of function call".to_string(),
                    )
                };

                self.error_reporter.register_runtime_error(error);
                None
            }
        }
    }

    fn execute_statements(
        &mut self,
        statements: &Vec<Stmt>,
        environment: &mut Environment,
        interactive: bool,
    ) -> Result<Object, ControlFlow> {
        let mut last_value = Object::None;
        for statement in statements {
            last_value = self.execute_statement(statement, environment, interactive)?;
        }
        Ok(last_value)
    }

    fn execute_statement(
        &mut self,
        statement: &Stmt,
        environment: &mut Environment,
        interactive: bool,
    ) -> Result<Object, ControlFlow> {
        match statement {
            Stmt::Expression(expr) => match self.evaluate(&expr, environment) {
                Ok(value) => {
                    if interactive {
                        println!("{}", value);
                        return Ok(value);
                    }
                }
                Err(e) => return Err(ControlFlow::Error(e)),
            },

            Stmt::Block(stmts) => {
                self.execute_statements(stmts, environment, interactive)?;
            }

            Stmt::Assignment { name, initializer } => {
                match self.evaluate(initializer, environment) {
                    Ok(value) => environment.define(name.lexeme.to_string(), value),
                    Err(e) => return Err(ControlFlow::Error(e)),
                }
            }

            Stmt::If {
                condition,
                then_body,
                else_body,
            } => {
                let value = self.if_statement(condition, then_body, else_body, environment)?;
                if interactive {
                    return Ok(value);
                }
            },

            Stmt::While { condition, body } => {
                loop {
                    match self.evaluate(condition, environment) {
                        Ok(value) => {
                            if value.is_truthy() {
                                self.execute_statement(body, environment, interactive)?;
                            } else {
                                break;
                            }
                        },
                        Err(e) => return Err(ControlFlow::Error(e)),
                    }
                }
            },

            Stmt::Function { name, params, body } => {
                let name = name.lexeme.clone();

                let user_fn = Object::Function(Callable::UserDefined(UserDefinedFn {
                    name: name.clone(),
                    params: params.clone(),
                    body: body.clone()
                }));

                environment.define(name, user_fn);
            },

            Stmt::Return { keyword, value } => {
                let return_value = if let Some(value_expr) = value {
                    match self.evaluate(value_expr, environment) {
                        Ok(value) => value,
                        Err(e) => return Err(ControlFlow::Error(e))
                    }
                } else {
                    Object::None
                };

                // This looks weird. A return value isn't really an error, but we lump it under Err here
                // so that we can bubble it up to the call function using the ? operator. To keep returns
                // separated from actual errors, we make them different variants of the ControlFlow enum.
                return Err(ControlFlow::Return { keyword: keyword.clone(), value: return_value })
            }
        }

        Ok(Object::None)
    }

    fn if_statement(
        &mut self,
        condition: &Expr,
        then_body: &Stmt,
        else_body: &Option<Box<Stmt>>,
        environment: &mut Environment,
    ) -> Result<Object, ControlFlow> {
        let condition = match self.evaluate(condition, environment) {
            Ok(condition) => condition,
            Err(e) => return Err(ControlFlow::Error(e)),
        };

        if condition.is_truthy() {
            self.execute_statement(then_body, environment, false)?;
        } else if let Some(else_body) = else_body {
            self.execute_statement(else_body, environment, false)?;
        }

        Ok(Object::None)
    }

    /// Evaluates a given expression within the current environment.
    ///
    /// This function recursively evaluates expressions based on their type (literals, unary,
    /// binary, logical, variable, or function calls) and returns the resulting `Object`.
    /// It handles type checking and runtime errors during evaluation.
    ///
    /// # Arguments
    ///
    /// * `expr` - A reference to the `Expr` to be evaluated.
    /// * `environment` - A reference to the `Environment` containing variable bindings.
    ///
    /// # Returns
    ///
    /// A `Result<Object, RuntimeError>` indicating the evaluated `Object` on success,
    /// or a `RuntimeError` if an error occurs during evaluation.
    ///
    /// # Examples
    ///
    /// ```
    /// use whipsnake::{error::ErrorReporter, ast::{Expr, self}, evaluator::Evaluator, environment::Environment, token::{Token, TokenKind, SourceLocation, Literal}, object::Object};
    ///
    /// let mut error_reporter = ErrorReporter::new();
    /// let mut evaluator = Evaluator::new(&mut error_reporter);
    /// let mut environment = Environment::new_global();
    ///
    /// let expr = Expr::Binary {
    ///     left: Box::new(Expr::Literal(Literal::Int(1))),
    ///     operator: Token::new(TokenKind::Plus, "+", 1),
    ///     right: Box::new(Expr::Literal(Literal::Int(2))),
    /// };
    /// let result = evaluator.evaluate(&expr, &environment).unwrap();
    /// assert_eq!(result, Object::Int(3));
    ///
    /// let expr_float = Expr::Binary {
    ///     left: Box::new(Expr::Literal(Literal::Float(1.5))),
    ///     operator: Token::new(TokenKind::Star, "*", 1),
    ///     right: Box::new(Expr::Literal(Literal::Int(2))),
    /// };
    /// let result_float = evaluator.evaluate(&expr_float, &environment).unwrap();
    /// assert_eq!(result_float, Object::Float(3.0));
    /// ```
    pub fn evaluate(&mut self, expr: &Expr, environment: &Environment) -> Result<Object, RuntimeError> {
        let value = match expr {
            Expr::Literal(literal) => match literal {
                Literal::Int(int) => Object::Int(*int),
                Literal::Float(float) => Object::Float(*float),
                Literal::String(string) => Object::String(string.clone()),
                Literal::Bool(b) => Object::Bool(*b),
                Literal::None => Object::None,
            },

            Expr::Grouping(inner_expr) => self.evaluate(inner_expr, environment)?,

            Expr::Unary { operator, right } => {
                let right = self.evaluate(right, environment)?;
                return self.unary_expr(operator, right);
            }

            Expr::Binary { left, operator, right } => {
                let left = self.evaluate(left, environment)?;
                let right = self.evaluate(right, environment)?;
                return self.binary_expr(&left, operator, &right);
            }

            Expr::Logical {
                left,
                operator,
                right,
            } => {
                let left = self.evaluate(left, environment)?;

                match operator.kind {
                    TokenKind::Or => {
                        if left.is_truthy() {
                            return Ok(left);
                        }
                    }
                    TokenKind::And => {
                        if !left.is_truthy() {
                            return Ok(left);
                        }
                    }
                    _ => panic!("invalid logical operator {:?}", operator),
                }

                return self.evaluate(right, environment);
            }

            Expr::Variable(token) => match environment.get(token.lexeme.as_ref()) {
                Some(object) => object.clone(),
                None => {
                    return Err(RuntimeError::NameError(
                        SourceLocation { line: token.line },
                        format!("name '{}' is not defined", token.lexeme),
                    ));
                }
            },

            Expr::Call {
                callee,
                paren,
                arguments,
            } => {
                let callee = self.evaluate(callee, environment)?;

                let mut arg_objects = vec![];
                for argument in arguments {
                    arg_objects.push(self.evaluate(argument, environment)?);
                }

                return self.call(&callee, paren, arg_objects, environment);
            }
        };

        Ok(value)
    }

    fn call(
        &mut self,
        callee: &Object,
        paren: &Token,
        arguments: Vec<Object>,
        enclosing: &Environment,
    ) -> Result<Object, RuntimeError> {
        if let Object::Function(callable) = callee {
            match callable {
                Callable::UserDefined(user_fn) => {
                    if arguments.len() != user_fn.params.len() {
                        return Err(RuntimeError::RuntimeError(
                            SourceLocation { line: paren.line },
                            format!("Function {} expects {} arguments but got {}", user_fn.name, user_fn.params.len(), arguments.len()),
                        ));
                    }

                    let mut environment = Environment::new_local(enclosing);
                    for (arg, param) in arguments.iter().zip(user_fn.params.clone()) {
                        environment.define(param.lexeme, arg.clone());
                    }

                    match self.execute_statements(&user_fn.body, &mut environment, false) {
                        Ok(_) => Ok(Object::None),
                        Err(ControlFlow::Error(e)) => return Err(e),
                        Err(ControlFlow::Return { keyword: _keyword, value }) => {
                            println!("got return from executing statements within call, value = {}", value);
                            return Ok(value)
                        }
                    }
                },
                Callable::Native(native_fn) => {
                    self.check_arity(arguments.len(), native_fn.arity, native_fn.name, paren)?;
                    (native_fn.body)(arguments)
                },
            }
        } else {
            return Err(RuntimeError::TypeError(
                SourceLocation { line: paren.line },
                format!("'{}' object is not callable", callee.py_type()),
            ));
        }
    }

    fn check_arity(
        &self,
        num_args: usize,
        arity: Arity,
        name: &'static str,
        paren: &Token,
    ) -> Result<(), RuntimeError> {
        match arity {
            Arity::Exact(n) => {
                if num_args != n {
                    return Err(RuntimeError::TypeError(
                        SourceLocation { line: paren.line },
                        format!("{}() expected {} arguments but got {}", name, n, num_args),
                    ));
                }
            }
            Arity::Minimum(n) => {
                if num_args < n {
                    return Err(RuntimeError::TypeError(
                        SourceLocation { line: paren.line },
                        format!(
                            "{}() expected at least {} arguments but got {}",
                            name, n, num_args
                        ),
                    ));
                }
            }
        }

        Ok(())
    }

    fn unary_expr(
        &self,
        operator: &Token,
        right: Object,
    ) -> Result<Object, RuntimeError> {
        let result = match operator.kind {

            TokenKind::Plus => {
                // unary + is identity
                match right {
                    Object::Int(_) | Object::Float(_) => right,
                    _ => {
                        return Err(RuntimeError::TypeError(
                            SourceLocation {
                                line: operator.line,
                            },
                            format!(
                                "bad operand type for unary -: '{}'",
                                right.py_type()
                            ),
                        ));
                    }
                }
            },

            TokenKind::Minus => match right {
                Object::Int(int) => Object::Int(-int),
                Object::Float(float) => Object::Float(-float),
                _ => {
                    return Err(RuntimeError::TypeError(
                        SourceLocation {
                            line: operator.line,
                        },
                        format!(
                            "bad operand type for unary -: '{}'",
                            right.py_type()
                        ),
                    ));
                }
            },

            TokenKind::Tilde => {
                // unary ~ is bitwise inversion, which for two's complement integers
                // works out to:  ~x = -(x+1)
                match right {
                    Object::Int(int) => Object::Int(-(int + 1)),
                    _ => {
                        return Err(RuntimeError::TypeError(
                            SourceLocation {
                                line: operator.line,
                            },
                            format!(
                                "bad operand type for unary -: '{}'",
                                right.py_type()
                            ),
                        ));
                    }
                }
            }

            TokenKind::Not => Object::Bool(!right.is_truthy()),
            
            _ => {
                return Err(RuntimeError::TypeError(
                    SourceLocation {
                        line: operator.line,
                    },
                    format!("invalid unary operator: '{}'", operator.lexeme),
                ));
            }
        };

        Ok(result)
    }

    fn binary_expr(
        &self,
        left: &Object,
        operator: &Token,
        right: &Object,
    ) -> Result<Object, RuntimeError> {
        let result = match operator.kind {
            
            TokenKind::Plus => match (&left, &right) {
                (Object::Int(l), Object::Int(r)) => Object::Int(l + r),
                (Object::Float(l), Object::Float(r)) => Object::Float(l + r),
                (Object::Int(l), Object::Float(r)) => Object::Float(*l as f64 + r),
                (Object::Float(l), Object::Int(r)) => Object::Float(l + *r as f64),
                (Object::String(l), Object::String(r)) => Object::String(format!("{}{}", l, r)),
                _ => {
                    return Err(RuntimeError::TypeError(
                        SourceLocation {
                            line: operator.line,
                        },
                        format!(
                            "unsupported operand type(s) for +: '{}' and '{}'",
                            left.py_type(),
                            right.py_type()
                        ),
                    ));
                }
            },

            TokenKind::Minus => match (&left, &right) {
                (Object::Int(l), Object::Int(r)) => Object::Int(l - r),
                (Object::Float(l), Object::Float(r)) => Object::Float(l - r),
                (Object::Int(l), Object::Float(r)) => Object::Float(*l as f64 - r),
                (Object::Float(l), Object::Int(r)) => Object::Float(l - *r as f64),
                _ => {
                    return Err(RuntimeError::TypeError(
                        SourceLocation {
                            line: operator.line,
                        },
                        format!(
                            "unsupported operand type(s) for -: '{}' and '{}'",
                            left.py_type(),
                            right.py_type()
                        ),
                    ));
                }
            },

            TokenKind::Star => match (&left, &right) {
                (Object::Int(l), Object::Int(r)) => Object::Int(l * r),
                (Object::Float(l), Object::Float(r)) => Object::Float(l * r),
                (Object::Int(l), Object::Float(r)) => Object::Float(*l as f64 * r),
                (Object::Float(l), Object::Int(r)) => Object::Float(l * *r as f64),
                (Object::String(l), Object::Int(r)) => {
                    let mut string = String::new();
                    for _ in 0..*r {
                        string.push_str(l);
                    }
                    Object::String(string)
                },
                _ => {
                    return Err(RuntimeError::TypeError(
                        SourceLocation {
                            line: operator.line,
                        },
                        format!(
                            "unsupported operand type(s) for *: '{}' and '{}'",
                            left.py_type(),
                            right.py_type()
                        ),
                    ));
                }
            },

            TokenKind::Slash => match (&left, &right) {
                (Object::Int(l), Object::Int(r)) => Object::Float(*l as f64 / *r as f64),
                (Object::Float(l), Object::Float(r)) => Object::Float(l / r),
                (Object::Int(l), Object::Float(r)) => Object::Float(*l as f64 / r),
                (Object::Float(l), Object::Int(r)) => Object::Float(l / *r as f64),
                _ => {
                    return Err(RuntimeError::TypeError(
                        SourceLocation {
                            line: operator.line,
                        },
                        format!(
                            "unsupported operand type(s) for /: '{}' and '{}'",
                            left.py_type(),
                            right.py_type()
                        ),
                    ));
                }
            },

            TokenKind::Greater => match (&left, &right) {
                (Object::Int(l), Object::Int(r)) => Object::Bool(l > r),
                (Object::Float(l), Object::Float(r)) => Object::Bool(l > r),
                (Object::Int(l), Object::Float(r)) => Object::Bool(*l as f64 > *r),
                (Object::Float(l), Object::Int(r)) => Object::Bool(*l > *r as f64),
                _ => {
                    return Err(RuntimeError::TypeError(
                        SourceLocation {
                            line: operator.line,
                        },
                        format!(
                            "unsupported operand type(s) for >: '{}' and '{}'",
                            left.py_type(),
                            right.py_type()
                        ),
                    ));
                }
            },

            TokenKind::GreaterEqual => match (&left, &right) {
                (Object::Int(l), Object::Int(r)) => Object::Bool(l >= r),
                (Object::Float(l), Object::Float(r)) => Object::Bool(l >= r),
                (Object::Int(l), Object::Float(r)) => Object::Bool(*l as f64 >= *r),
                (Object::Float(l), Object::Int(r)) => Object::Bool(*l >= *r as f64),
                _ => {
                    return Err(RuntimeError::TypeError(
                        SourceLocation {
                            line: operator.line,
                        },
                        format!(
                            "unsupported operand type(s) for >=: '{}' and '{}'",
                            left.py_type(),
                            right.py_type()
                        ),
                    ));
                }
            },

            TokenKind::Less => match (&left, &right) {
                (Object::Int(l), Object::Int(r)) => Object::Bool(l < r),
                (Object::Float(l), Object::Float(r)) => Object::Bool(l < r),
                (Object::Int(l), Object::Float(r)) => Object::Bool((*l as f64) < *r),
                (Object::Float(l), Object::Int(r)) => Object::Bool(*l < *r as f64),
                _ => {
                    return Err(RuntimeError::TypeError(
                        SourceLocation {
                            line: operator.line,
                        },
                        format!(
                            "unsupported operand type(s) for <: '{}' and '{}'",
                            left.py_type(),
                            right.py_type()
                        ),
                    ));
                }
            },

            TokenKind::LessEqual => match (&left, &right) {
                (Object::Int(l), Object::Int(r)) => Object::Bool(l <= r),
                (Object::Float(l), Object::Float(r)) => Object::Bool(l <= r),
                (Object::Int(l), Object::Float(r)) => Object::Bool(*l as f64 <= *r),
                (Object::Float(l), Object::Int(r)) => Object::Bool(*l <= *r as f64),
                _ => {
                    return Err(RuntimeError::TypeError(
                        SourceLocation {
                            line: operator.line,
                        },
                        format!(
                            "unsupported operand type(s) for <=: '{}' and '{}'",
                            left.py_type(),
                            right.py_type()
                        ),
                    ));
                }
            },

            TokenKind::EqualEqual => Object::Bool(right == left),

            TokenKind::BangEqual => Object::Bool(right != left),

            TokenKind::Is => Object::Bool(right.identity() == left.identity()),

            _ => {
                return Err(RuntimeError::RuntimeError(
                    SourceLocation {
                        line: operator.line,
                    },
                    format!(
                        "somehow hit binary float expression with incompatible operator {:?}",
                        operator
                    ),
                ));
            }
        };

        Ok(result)
    }
}
