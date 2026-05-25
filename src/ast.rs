use crate::token::{Token, Literal};

pub enum Expr<'a> {
    Literal(Literal<'a>),
    Grouping(Box<Expr<'a>>),
    Unary { operator: Token<'a>, right: Box<Expr<'a>> },
    Binary { left: Box<Expr<'a>>, operator: Token<'a>, right: Box<Expr<'a>> },
}

pub struct PrettyPrinter;

impl PrettyPrinter {
    pub fn pprint_expr(&self, e: &Expr) -> String {
        match e {
            Expr::Literal(Literal::String(s)) => s.to_string(),
            Expr::Literal(Literal::Float(f)) => format!("{f}").to_string(),
            Expr::Grouping(expr) => self.parenthesize("group", &[expr]), 
            Expr::Unary { operator, right } => {
                self.parenthesize(operator.lexeme, &[right])
            },
            Expr::Binary { left, operator, right } => {
                self.parenthesize(operator.lexeme, &[left, right])
            },
            _ => String::from(""), 
        }
    }

    fn parenthesize(&self, name: &str, exprs: &[&Expr]) -> String {
        let mut s = String::from(format!("({name}"));

        for expr in exprs {
            s.push(' ');
            s.push_str(self.pprint_expr(expr).as_str());
        }

        s.push(')');
        s
    }
}