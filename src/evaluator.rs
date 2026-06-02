use crate::{
    ast::{Expr, Stmt},
    environment::Environment,
    error::{ErrorReporter, RuntimeError},
    object::Object,
    token::{Literal, SourceLocation, Token, TokenKind},
};

pub struct Evaluator<'err> {
    error_reporter: &'err mut ErrorReporter,
}

impl<'err> Evaluator<'err> {
    pub fn new(error_reporter: &'err mut ErrorReporter) -> Self {
        Evaluator { error_reporter }
    }

    pub fn interpret(
        &mut self,
        statements: &Vec<Stmt>,
        environment: &mut Environment,
        interactive: bool,
    ) {
        for statement in statements {
            self.execute(statement, environment, interactive);
        }
    }

    pub fn execute(&mut self, statement: &Stmt, environment: &mut Environment, interactive: bool) {
        match statement {
            Stmt::Print(expr) => match self.evaluate(&expr, environment) {
                Ok(value) => println!("{}", value),
                Err(e) => self.error_reporter.register_runtime_error(e),
            },

            Stmt::Expression(expr) => match self.evaluate(&expr, environment) {
                Ok(value) => {
                    if interactive {
                        println!("{}", value)
                    }
                }
                Err(e) => self.error_reporter.register_runtime_error(e),
            },

            Stmt::Block(stmts) => {
                for stmt in stmts {
                    self.execute(stmt, environment, interactive);
                }
            }

            Stmt::Assignment { name, initializer } => {
                match self.evaluate(initializer, environment) {
                    Ok(value) => environment.define(name.lexeme.to_string(), value),
                    Err(e) => self.error_reporter.register_runtime_error(e),
                }
            }

            Stmt::If {
                condition,
                then_body,
                else_body,
            } => {
                match self.if_statement(condition, then_body, else_body, environment, interactive) {
                    Ok(_) => (),
                    Err(e) => self.error_reporter.register_runtime_error(e),
                }
            }
        }
    }

    fn if_statement(
        &mut self,
        condition: &Expr,
        then_body: &Stmt,
        else_body: &Option<Box<Stmt>>,
        environment: &mut Environment,
        interactive: bool,
    ) -> Result<Object, RuntimeError> {
        let condition = self.evaluate(condition, environment)?;

        if condition.is_truthy() {
            self.execute(then_body, environment, interactive);
        } else {
            match else_body {
                Some(else_body) => self.execute(else_body, environment, interactive),
                None => ()
            }
        }

        Ok(Object::None)
    }

    pub fn evaluate(&self, expr: &Expr, environment: &Environment) -> Result<Object, RuntimeError> {
        let value = match expr {
            Expr::Literal(literal) => match literal {
                Literal::Int(int) => Object::Int(*int),
                Literal::Float(float) => Object::Float(*float),
                Literal::String(string) => Object::String(string.to_string()),
                Literal::Bool(b) => Object::Bool(*b),
                Literal::None => Object::None,
            },

            Expr::Grouping(inner_expr) => self.evaluate(inner_expr, environment)?,

            Expr::Unary { operator, right } => {
                match self.evaluate(right, environment) {
                    Ok(right) => {
                        match operator.kind {
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
                            }
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
                        }
                    }
                    Err(e) => return Err(e),
                }
            }

            Expr::Binary {
                left,
                operator,
                right,
            } => {
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
                    TokenKind::Or => if left.is_truthy() { return Ok(left) },
                    TokenKind::And => if !left.is_truthy() { return Ok(left) },
                    _ => panic!("invalid logical operator {:?}", operator),
                }

                return self.evaluate(right, environment);
            }            

            Expr::Variable(token) => match environment.get(token.lexeme) {
                Some(object) => object.clone(),
                None => {
                    return Err(RuntimeError::NameError(
                        SourceLocation { line: token.line },
                        format!("name '{}' is not defined", token.lexeme),
                    ));
                }
            },
        };

        Ok(value)
    }

    fn binary_expr<'src>(
        &self,
        left: &Object,
        operator: &Token<'src>,
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
