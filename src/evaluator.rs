use crate::{
    ast::{Stmt, Expr},
    object::Object,
    token::{Token, TokenKind, Literal, SourceLocation},
    error::{ErrorReporter, RuntimeError},
    environment::Environment,
};

pub struct Evaluator<'err> {
    environment: Environment,
    error_reporter: &'err mut ErrorReporter,
}

impl<'err> Evaluator<'err> {
    pub fn new(error_reporter: &'err mut ErrorReporter) -> Self {
        Evaluator {
            environment: Environment::new(),
            error_reporter
        }
    }

    pub fn interpret(&mut self, statements: &Vec<Stmt>) {
        for statement in statements {
            self.execute(statement);
        }
    }

    pub fn execute(&mut self, statement: &Stmt) {
        match statement {
            Stmt::Print(expr) => {
                match self.evaluate(&expr) {
                    Ok(value) => println!("{}", value),
                    Err(e) => self.error_reporter.register_runtime_error(e),
                }
            },
            
            Stmt::Expression(expr) => {
                match self.evaluate(&expr) {
                    Err(e) => self.error_reporter.register_runtime_error(e),
                    _ => (),
                }
            },
            
            Stmt::Assignment { name, initializer } => {
                match self.evaluate(initializer) {
                    Ok(value) => self.environment.define(name.lexeme.to_string(), value),
                    Err(e) => self.error_reporter.register_runtime_error(e),
                }
            }
            
            _ => self.error_reporter.register_runtime_error(
                RuntimeError::RuntimeError(
                    SourceLocation { line: 0 },
                    format!("don't know how to evaluate statement {:?}", statement)
                )
            )
        }
    }
    
    pub fn evaluate(&self, expr: &Expr) -> Result<Object, RuntimeError> {
        let value = match expr {
            Expr::Literal(literal) => {
                match literal {
                    Literal::Float(float) => Object::Float(*float),
                    Literal::String(string) => Object::String(string.to_string()),
                    Literal::Bool(b) => Object::Bool(*b),
                    Literal::None => Object::None, 
                }
            },
            
            Expr::Grouping(inner_expr) => self.evaluate(inner_expr)?,

            Expr::Unary { operator, right } => {
                match self.evaluate(right) {
                    Ok(right) => {
                        match operator.kind {
                            TokenKind::Minus => {
                                match right {
                                    Object::Float(float) => Object::Float(-float),
                                    _ => return Err(RuntimeError::TypeError(
                                        SourceLocation { line: operator.line },
                                        format!("bad operand type for unary -: '{}'", right.py_type())
                                    ))
                                }
                            },
                            TokenKind::Not => Object::Bool(!right.is_truthy()),
                            _ => return Err(RuntimeError::TypeError(
                                SourceLocation { line: operator.line },
                                format!("invalid unary operator: '{}'", operator.lexeme)
                            ))
                        }
                    },
                    Err(e) => return Err(e)
                }
            },

            Expr::Binary { left, operator, right} => {
                let left = self.evaluate(left)?;
                let right = self.evaluate(right)?;

                match operator.kind {
                    TokenKind::Plus => match (&left, &right) {
                        (Object::Float(fl), Object::Float(fr)) => Object::Float(fl + fr),
                        (Object::String(sl), Object::String(sr)) => Object::String(format!("{}{}", sl, sr)),
                        _ => return Err(RuntimeError::TypeError(
                            SourceLocation { line: operator.line },
                            format!("unsupported operand of type(s) for +: '{}' and '{}'", left.py_type(), right.py_type())
                        ))
                    }
                    TokenKind::EqualEqual => Object::Bool(right == left),
                    TokenKind::BangEqual => Object::Bool(right != left),
                    TokenKind::Minus | TokenKind::Star | TokenKind::Slash |
                    TokenKind::Greater | TokenKind::GreaterEqual | TokenKind::Less | TokenKind::LessEqual => {
                         self.evaluate_binary_float_expr(&left, &operator, &right)?
                    }
                    _ => return Err(RuntimeError::TypeError(
                        SourceLocation { line: operator.line },
                        format!("got an invalid binary operator {:?}", operator.kind)
                    ))
                }
            },

            Expr::Variable(token) => {
                match self.environment.get(token.lexeme) {
                    Some(object) => object.clone(),
                    None => return Err(RuntimeError::NameError(
                        SourceLocation { line: token.line },
                        format!("name '{}' is not defined", token.lexeme)
                    )),
                }
            }

            _ => return Err(RuntimeError::RuntimeError(
                SourceLocation { line: 0 },
                format!("don't know how to evaluate expression {:?}", expr)
            ))
        };

        Ok(value)
    }

    fn evaluate_binary_float_expr<'src>(
        &self,
        left: &Object,
        operator: &Token<'src>,
        right: &Object
    ) -> Result<Object, RuntimeError> {
        if let (Object::Float(fl), Object::Float(fr)) = (left, right) {
            let value = match operator.kind {
                TokenKind::Minus => Object::Float(fl - fr),
                TokenKind::Star => Object::Float(fl * fr),
                TokenKind::Slash => Object::Float(fl / fr),
                TokenKind::Greater => Object::Bool(fl > fr),
                TokenKind::GreaterEqual => Object::Bool(fl >= fr),
                TokenKind::Less => Object::Bool(fl < fr),
                TokenKind::LessEqual => Object::Bool(fl <= fr),
                _ => return Err(RuntimeError::TypeError(
                    SourceLocation { line: operator.line },
                    format!("unsupported operand of type(s) for {operator:?}: 'float' and 'floar'")
                ))
            };

            Ok(value)
        } else {
            Err(RuntimeError::RuntimeError(
                SourceLocation { line: operator.line },
                format!("somehow hit binary float expression with incompatible operator {:?}", operator)
            ))
        }
    }
}