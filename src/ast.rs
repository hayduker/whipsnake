use crate::token::{Token, Literal};

enum Expr {
    Literal(Literal),
    Grouping(Box<Expr>),
    Unary { operator: Token, right: Box<Expr> },
    Binary { left: Box<Expr>, operator: Token, right: Box<Expr> },
}


