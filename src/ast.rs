use crate::token::{Token, Literal};

pub enum Expr<'src> {
    Literal(Literal<'src>),
    Grouping(Box<Expr<'src>>),
    Unary { operator: Token<'src>, right: Box<Expr<'src>> },
    Binary { left: Box<Expr<'src>>, operator: Token<'src>, right: Box<Expr<'src>> },
}
