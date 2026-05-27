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
                    Literal::Bool(true) => Object::Bool(true),
                    Literal::Bool(false) => Object::Bool(false),
                    Literal::None => Object::None, 
                }
            },
            Expr::Grouping(inner_expr) => {
                self.evaluate(inner_expr)
            },
            Expr::Unary { operator, right } => {
                let right = self.evaluate(right);

                match operator.kind {
                    TokenKind::Minus => {
                        match right {
                            Object::Float(float) => Object::Float(-float),
                            _ => panic!("Can't negate something that isn't a number!")
                        }
                    },
                    TokenKind::Not => {
                        match right {
                            Object::Bool(true) => Object::Bool(false),
                            Object::Bool(false) | Object::None => Object::Bool(true),
                            _ => panic!("Can't perform logical not on something that isn't a Boolean!")
                        }
                    },
                    _ => panic!("Got an invalid unary operator {:?}", operator.kind)
                }
            },
            Expr::Binary { left, operator, right} => {
                let left = self.evaluate(left);
                let right = self.evaluate(right);

                match operator.kind {
                    TokenKind::Plus => {
                        if let (Object::Float(fl), Object::Float(fr)) = (&left, &right) {
                            return Object::Float(fl + fr);
                        } else if let (Object::String(sl), Object::String(sr)) = (&left, &right) {
                            return Object::String(sl.clone() + sr);
                        }
                        else {
                            panic!("Can't add two objects of those types!")
                        }
                    },
                    TokenKind::Minus => {
                        if let (Object::Float(fl), Object::Float(fr)) = (left, right) {
                            return Object::Float(fl - fr);
                        }
                        else {
                            panic!("Can't subtract two objects of those types!")
                        }
                    },
                    TokenKind::Star => {
                        if let (Object::Float(fl), Object::Float(fr)) = (left, right) {
                            return Object::Float(fl * fr);
                        }
                        else {
                            panic!("Can't multiply two objects of those types!")
                        }
                    },
                    TokenKind::Slash => {
                        if let (Object::Float(fl), Object::Float(fr)) = (left, right) {
                            return Object::Float(fl / fr);
                        }
                        else {
                            panic!("Can't divide two objects of those types!")
                        }
                    },
                    _ => panic!("Got an invalid binary operator {:?}", operator.kind)
                }
            },
            _ => panic!("Got an expression that I don't know how to implement yet!")
        }
    }
}