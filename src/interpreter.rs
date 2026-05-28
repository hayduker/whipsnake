use crate::{
    ast::Expr,
    object::Object,
    token::{Token, TokenKind, Literal},
    error::ErrorReporter,
};

#[derive(Debug)]
pub struct RuntimeError<'src> {
    token: Token<'src>,
    message:  String,
}

pub struct Interpreter<'err> {
    error_reporter: &'err mut ErrorReporter,
}

impl<'err> Interpreter<'err> {
    pub fn new(error_reporter: &'err mut ErrorReporter) -> Self {
        Interpreter { error_reporter }
    }

    pub fn interpret(&self, expr: &Expr) {
        match self.evaluate(expr) {
            Ok(value) => println!("{:?}", value),
            Err(e) => () //self.error_reporter.register_error(e),
        }
    }
    
    pub fn evaluate<'src>(&self, expr: &Expr<'src>) -> Result<Object, RuntimeError<'src>> {
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
                                    _ => panic!("Can't negate something that isn't a number!")
                                }
                            },
                            TokenKind::Not => Object::Bool(!right.is_truthy()),
                            _ => panic!("Got an invalid unary operator {:?}", operator.kind)
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
                        _ => return Err(RuntimeError {
                            token: *operator,
                            message: format!("TypeError: unsupported operand type(s) for +: '{:?}' and '{:?}'", left, right)
                        })
                    }
                    TokenKind::EqualEqual => Object::Bool(right == left),
                    TokenKind::BangEqual => Object::Bool(right != left),
                    TokenKind::Minus | TokenKind::Star | TokenKind::Slash |
                    TokenKind::Greater | TokenKind::GreaterEqual | TokenKind::Less | TokenKind::LessEqual => {
                         self.evaluate_binary_float_expr(&left, &operator, &right)?
                    }
                    _ => return Err(RuntimeError {
                        token: *operator,
                        message: format!("Got an invalid binary operator {:?}", operator.kind)
                    })
                }
            },
        };

        Ok(value)
    }

    fn evaluate_binary_float_expr<'src>(
        &self,
        left: &Object,
        operator: &Token<'src>,
        right: &Object
    ) -> Result<Object, RuntimeError<'src>> {
        if let (Object::Float(fl), Object::Float(fr)) = (left, right) {
            let value = match operator.kind {
                TokenKind::Minus => Object::Float(fl - fr),
                TokenKind::Star => Object::Float(fl * fr),
                TokenKind::Slash => Object::Float(fl / fr),
                TokenKind::Greater => Object::Bool(fl > fr),
                TokenKind::GreaterEqual => Object::Bool(fl >= fr),
                TokenKind::Less => Object::Bool(fl < fr),
                TokenKind::LessEqual => Object::Bool(fl <= fr),
                _ => return Err(RuntimeError {
                    token: *operator,
                    message: format!("Can't use operator {operator:?} on two floats!")
                })                    
            };

            Ok(value)
        } else {
            Err(RuntimeError {
                token: *operator,
                message: format!("Somehow hit binary float expression with incompatible operator {:?}", operator)
            })
        }
    }
}