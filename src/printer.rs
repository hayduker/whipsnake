use crate::{token::Literal, ast::Expr};

pub struct PrettyPrinter;

impl PrettyPrinter {
    pub fn print(e: &Expr) -> String {
        match e {
            Expr::Literal(Literal::String(s)) => format!("\"{s}\""),
            Expr::Literal(Literal::Float(f)) => format!("{f}").to_string(),
            Expr::Grouping(expr) => PrettyPrinter::parenthesize("group", &[expr]), 
            Expr::Unary { operator, right } => {
                PrettyPrinter::parenthesize(operator.lexeme, &[right])
            },
            Expr::Binary { left, operator, right } => {
                PrettyPrinter::parenthesize(operator.lexeme, &[left, right])
            },
            _ => String::from(""), 
        }
    }

    fn parenthesize(name: &str, exprs: &[&Expr]) -> String {
        let mut s = String::from(format!("({name}"));

        for expr in exprs {
            s.push(' ');
            s.push_str(PrettyPrinter::print(expr).as_str());
        }

        s.push(')');
        s
    }
}