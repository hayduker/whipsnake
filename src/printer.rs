use crate::{token::Literal, ast::{Stmt, Expr}};

pub struct PrettyPrinter;

impl PrettyPrinter {
    pub fn print(statements: &Vec<Stmt>) -> String {
        let mut output = String::from("");

        for s in statements {
            output += PrettyPrinter::print_stmt(&s).as_str();
            output += "\n";
        }

        output
    }

    pub fn print_stmt(s: &Stmt) -> String {
        match s {
            Stmt::Print(expr) => PrettyPrinter::parenthesize("print", &[expr]),
            Stmt::Expression(expr) => PrettyPrinter::parenthesize("stmt", &[expr])
        }
    }

    pub fn print_expr(e: &Expr) -> String {
        match e {
            Expr::Literal(Literal::String(s)) => format!("\"{s}\""),
            Expr::Literal(Literal::Float(f)) => format!("{f}").to_string(),
            Expr::Literal(Literal::Bool(true)) => format!("True"),
            Expr::Literal(Literal::Bool(false)) => format!("False"),
            Expr::Literal(Literal::None) => format!("None"),
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
            s.push_str(PrettyPrinter::print_expr(expr).as_str());
        }

        s.push(')');
        s
    }
}