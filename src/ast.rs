use crate::token::{Token, Literal};

pub enum Expr<'a> {
    Literal(Literal<'a>),
    Grouping(Box<Expr<'a>>),
    Unary { operator: Token<'a>, right: Box<Expr<'a>> },
    Binary { left: Box<Expr<'a>>, operator: Token<'a>, right: Box<Expr<'a>> },
}
