use std::ops::Deref;

use crate::{ast::{AstNode, Expr, Stmt}, token::Literal};

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
            Stmt::Expression(expr) => PrettyPrinter::parenthesize("stmt", &vec![AstNode::Expr(expr)]),
            Stmt::Assignment { name, initializer } => PrettyPrinter::parenthesize(format!("assign {}", name.lexeme).as_str(), &vec![AstNode::Expr(initializer)]),
            Stmt::If { condition, true_body, false_body } => {               
                let mut true_body_nodes = vec![];
                for stmt in true_body.iter() {
                    true_body_nodes.push(AstNode::Stmt(stmt));
                }

                let mut false_body_nodes = vec![];
                for stmt in false_body.iter() {
                    false_body_nodes.push(AstNode::Stmt(stmt));
                }

                let condition_str = PrettyPrinter::print_expr(condition);
                let true_body_str = PrettyPrinter::parenthesize("block", &true_body_nodes);
                let false_body_str = PrettyPrinter::parenthesize("block", &false_body_nodes);


                format!("(if {condition_str} {true_body_str} {false_body_str})")    
            }
        }
    }

    pub fn print_expr(e: &Expr) -> String {
        match e {
            Expr::Literal(Literal::String(s)) => format!("\"{s}\""),
            Expr::Literal(Literal::Float(f)) => format!("{f}").to_string(),
            Expr::Literal(Literal::Bool(true)) => format!("True"),
            Expr::Literal(Literal::Bool(false)) => format!("False"),
            Expr::Literal(Literal::None) => format!("None"),
            Expr::Grouping(expr) => PrettyPrinter::parenthesize("group", &vec![AstNode::Expr(expr)]), 
            Expr::Unary { operator, right } => {
                PrettyPrinter::parenthesize(operator.lexeme, &vec![AstNode::Expr(right)])
            },
            Expr::Binary { left, operator, right } => {
                PrettyPrinter::parenthesize(operator.lexeme, &vec![AstNode::Expr(left), AstNode::Expr(right)])
            },
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