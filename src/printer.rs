use crate::{
    ast::{AstNode, Expr, Stmt},
    token::Literal,
};

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
            Stmt::Print(expr) => PrettyPrinter::parenthesize("print", &vec![AstNode::Expr(expr)]),
            Stmt::Expression(expr) => {
                PrettyPrinter::parenthesize("stmt", &vec![AstNode::Expr(expr)])
            }
            Stmt::Assignment { name, initializer } => PrettyPrinter::parenthesize(
                format!("assign {}", name.lexeme).as_str(),
                &vec![AstNode::Expr(initializer)],
            ),
            Stmt::Block(stmts) => {
                PrettyPrinter::parenthesize(
                    "block",
                    &(stmts.iter().map(|stmt| AstNode::Stmt(stmt)).collect())
                )
            },
            Stmt::If {
                condition,
                then_body,
                else_body,
            } => {
                let condition_str = PrettyPrinter::print_expr(condition);
                let then_body_str = PrettyPrinter::print_stmt(then_body);

                let string = format!("(if {condition_str} {then_body_str}");

                match else_body {
                    Some(else_body) => {
                        let else_body_str = PrettyPrinter::print_stmt(else_body);
                        string + format!(" {else_body_str})").as_str()
                    },
                    None => string
                }
            }
        }
    }

    pub fn print_expr(e: &Expr) -> String {
        match e {
            Expr::Literal(Literal::String(s)) => format!("\"{s}\""),
            Expr::Literal(Literal::Int(i)) => format!("{i}").to_string(),
            Expr::Literal(Literal::Float(f)) => format!("{f}").to_string(),
            Expr::Literal(Literal::Bool(true)) => format!("True"),
            Expr::Literal(Literal::Bool(false)) => format!("False"),
            Expr::Literal(Literal::None) => format!("None"),
            Expr::Grouping(expr) => {
                PrettyPrinter::parenthesize("group", &vec![AstNode::Expr(expr)])
            }
            Expr::Unary { operator, right } => {
                PrettyPrinter::parenthesize(operator.lexeme, &vec![AstNode::Expr(right)])
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => PrettyPrinter::parenthesize(
                operator.lexeme,
                &vec![AstNode::Expr(left), AstNode::Expr(right)],
            ),
            Expr::Variable(token) => format!("{}", token.lexeme),
        }
    }

    fn parenthesize(name: &str, nodes: &Vec<AstNode>) -> String {
        let mut s = String::from(format!("({name}"));

        for node in nodes {
            s.push(' ');

            match node {
                AstNode::Expr(expr) => s.push_str(PrettyPrinter::print_expr(expr).as_str()),
                AstNode::Stmt(stmt) => s.push_str(PrettyPrinter::print_stmt(stmt).as_str()),
            }
        }

        s.push(')');
        s
    }
}
