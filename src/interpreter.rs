use crate::{
    ast::Expr,
    object::Object,
    token::{TokenKind, Literal}
};

pub struct Interpreter;

impl Interpreter {
    pub fn evaluate(&self, expr: &Expr) -> Object {
        match expr {
            Expr::Literal(literal) => {
                match literal {
                    Literal::Float(float) => Object::Float(*float),
                    Literal::String(string) => Object::String(string.to_string()),
                    Literal::Bool(b) => Object::Bool(*b),
                    Literal::None => Object::None, 
                }
            },
            Expr::Grouping(inner_expr) => self.evaluate(inner_expr),
            Expr::Unary { operator, right } => {
                let right = self.evaluate(right);

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
            Expr::Binary { left, operator, right} => {
                let left = self.evaluate(left);
                let right = self.evaluate(right);

                match operator.kind {
                    TokenKind::Plus => match (&left, &right) {
                        (Object::Float(fl), Object::Float(fr)) => Object::Float(fl + fr),
                        (Object::String(sl), Object::String(sr)) => Object::String(format!("{}{}", sl, sr)),
                        _ => panic!("TypeError: unsupported operand type(s) for +: '{:?}' and '{:?}'", left, right),
                    }
                    TokenKind::EqualEqual => Object::Bool(right == left),
                    TokenKind::BangEqual => Object::Bool(right != left),
                    TokenKind::Minus | TokenKind::Star | TokenKind::Slash |
                    TokenKind::Greater | TokenKind::GreaterEqual | TokenKind::Less | TokenKind::LessEqual => {
                        self.evaluate_binary_float_expr(&left, &operator.kind, &right)
                    }
                    _ => panic!("Got an invalid binary operator {:?}", operator.kind)
                }
            },
            _ => panic!("Got something I don't know how to implement yet!")
        }
    }

    fn evaluate_binary_float_expr(&self, left: &Object, operator: &TokenKind, right: &Object) -> Object {
        if let (Object::Float(fl), Object::Float(fr)) = (left, right) {
            match operator {
                TokenKind::Minus => Object::Float(fl - fr),
                TokenKind::Star => Object::Float(fl * fr),
                TokenKind::Slash => Object::Float(fl / fr),
                TokenKind::Greater => Object::Bool(fl > fr),
                TokenKind::GreaterEqual => Object::Bool(fl >= fr),
                TokenKind::Less => Object::Bool(fl < fr),
                TokenKind::LessEqual => Object::Bool(fl <= fr),
                _ => panic!("Can't use operator {operator:?} on two floats!")
            }
        }
        else {
            panic!("")
        }
    }
}