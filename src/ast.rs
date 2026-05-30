use crate::token::{Token, Literal};

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt<'src> {
    Expression(Expr<'src>),
    Print(Expr<'src>),
    Assignment { name: Token<'src>, initializer: Expr<'src> },
    If { condition: Expr<'src>, body: Expr<'src> }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr<'src> {
    Literal(Literal<'src>),
    Grouping(Box<Expr<'src>>),
    Unary { operator: Token<'src>, right: Box<Expr<'src>> },
    Binary { left: Box<Expr<'src>>, operator: Token<'src>, right: Box<Expr<'src>> },
    Variable(Token<'src>),
}

