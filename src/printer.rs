use crate::{
    ast::{Expr, Stmt},
    token::Literal,
};


enum SExpr {
    Atom(String),
    List(Vec<SExpr>),
}

fn atom(s: &str) -> SExpr {
    SExpr::Atom(s.to_string())
}

fn measure_single_line(expr: &SExpr) -> usize {
    match expr {
        SExpr::Atom(s) => s.len(),
        SExpr::List(children) => {
            if children.is_empty() {
                return 2; // "()"
            }
            
            let children_len: usize = children.iter().map(measure_single_line).sum();
            
            // include '(' and ')' and spaces between children
            2 + children_len + (children.len() - 1)
        }
    }
}

fn format_flat(expr: &SExpr) -> String {
    match expr {
        SExpr::Atom(s) => s.clone(),
        SExpr::List(children) => {
            if children.is_empty() {
                return "()".to_string();
            }
            let child_strings: Vec<String> = children.iter().map(format_flat).collect();
            format!("({})", child_strings.join(" "))
        }
    }
}

fn format_sexpr(expr: &SExpr, indent: usize) -> String {
    match expr {
        SExpr::Atom(s) => s.clone(),

        SExpr::List(children) => {
            if children.is_empty() {
                return "()".to_string();
            }

            if measure_single_line(expr) <= 12 {
                return format_flat(expr);
            }

            let mut result = String::new();
            result.push('(');

            let first_str = format_sexpr(&children[0], indent + 1);
            result.push_str(&first_str);

            if children.len() > 1 {
                result.push(' ');

                let second_indent = indent + 1 + first_str.len() + 1;
                let second_str = format_sexpr(&children[1], second_indent);
                result.push_str(&second_str);

                for child in &children[2..] {
                    result.push('\n');
                    result.push_str(&" ".repeat(second_indent));
                    result.push_str(&format_sexpr(child, second_indent));
                }
            }

            result.push(')');
            result
        }
    }
}

fn convert_stmt(s: &Stmt) -> SExpr {
    match s {
        Stmt::Print(expr) => SExpr::List(vec![atom("print"), convert_expr(expr)]),
        Stmt::Expression(expr) => SExpr::List(vec![atom("stmt"), convert_expr(expr)]),
        Stmt::Assignment { name, initializer } => {
            SExpr::List(vec![atom("="), atom(name.lexeme), convert_expr(initializer)])
        },
        Stmt::Block(stmts) => {
            let mut sexpr = vec![atom("block")];
            sexpr.extend(stmts.iter().map(|stmt| convert_stmt(stmt)));
            SExpr::List(sexpr)
        },
        Stmt::If { condition, then_body, else_body } => {
            let mut sexpr = vec![
                atom("if"),
                convert_expr(condition),
                convert_stmt(then_body),
            ];

            match else_body {
                Some(else_body) => sexpr.push(convert_stmt(else_body)),
                None => ()
            }

            SExpr::List(sexpr)
        }
    }
}

fn convert_expr(e: &Expr) -> SExpr {
    match e {
        Expr::Literal(Literal::String(s)) => atom(format!("\"{s}\"").as_str()),
        Expr::Literal(Literal::Int(i)) => atom(format!("{i}").as_str()),
        Expr::Literal(Literal::Float(f)) => atom(format!("{f}").as_str()),
        Expr::Literal(Literal::Bool(true)) => atom("True"),
        Expr::Literal(Literal::Bool(false)) => atom("False"),
        Expr::Literal(Literal::None) => atom("None"),
        Expr::Grouping(expr) => SExpr::List(vec![atom("group"), convert_expr(expr)]),
        Expr::Unary { operator, right } => {
            SExpr::List(vec![atom(operator.lexeme), convert_expr(right)])
        },
        Expr::Binary { left, operator, right } => {
            SExpr::List(vec![atom(operator.lexeme), convert_expr(left), convert_expr(right)])
        },
        Expr::Variable(token) => atom(format!("{}", token.lexeme).as_str()),
    }
}

pub fn print_ast(stmts: &Vec<Stmt>) -> String {
    let mut result = String::new();
    for stmt in stmts {
        let sexpr = convert_stmt(stmt);
        result += format_sexpr(&sexpr, 0).as_str();
        result.push('\n');
    }

    result
}