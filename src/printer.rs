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
        Stmt::Expression(expr) => SExpr::List(vec![atom("stmt"), convert_expr(expr)]),
        Stmt::Assignment { name, initializer } => SExpr::List(vec![
            atom("="),
            atom(name.lexeme.as_str()),
            convert_expr(initializer),
        ]),
        Stmt::Block(stmts) => {
            let mut sexpr = vec![atom("block")];
            sexpr.extend(stmts.iter().map(|stmt| convert_stmt(stmt)));
            SExpr::List(sexpr)
        }
        Stmt::If {
            condition,
            then_body,
            else_body,
        } => {
            let mut sexpr = vec![atom("if"), convert_expr(condition), convert_stmt(then_body)];

            if let Some(else_body) = else_body {
                sexpr.push(convert_stmt(else_body));
            }
            SExpr::List(sexpr)
        }
        Stmt::While { condition, body } => SExpr::List(vec![
            atom("while"),
            convert_expr(condition),
            convert_stmt(body),
        ]),
    }
}

fn convert_expr(e: &Expr) -> SExpr {
    match e {
        Expr::Literal(Literal::String(s)) => atom(&format!("\"{s}\"")),
        Expr::Literal(Literal::Int(i)) => atom(&i.to_string()),
        Expr::Literal(Literal::Float(f)) => atom(&f.to_string()),
        Expr::Literal(Literal::Bool(b)) => atom(if *b { "True" } else { "False" }),
        Expr::Literal(Literal::None) => atom("None"),

        Expr::Grouping(expr) => SExpr::List(vec![atom("group"), convert_expr(expr)]),
        Expr::Unary { operator, right } => {
            SExpr::List(vec![atom(operator.lexeme.as_str()), convert_expr(right)])
        }
        Expr::Binary {
            left,
            operator,
            right,
        } => SExpr::List(vec![
            atom(operator.lexeme.as_str()),
            convert_expr(left),
            convert_expr(right),
        ]),
        Expr::Logical {
            left,
            operator,
            right,
        } => SExpr::List(vec![
            atom(operator.lexeme.as_str()),
            convert_expr(left),
            convert_expr(right),
        ]),
        Expr::Variable(token) => atom(token.lexeme.as_str()),
        Expr::Call {
            callee,
            paren: _paren,
            arguments,
        } => {
            let mut sexpr = vec![atom("call"), convert_expr(callee)];
            for argument in arguments {
                sexpr.push(convert_expr(argument))
            }
            SExpr::List(sexpr)
        }
    }
}

pub fn print_ast(stmts: &Vec<Stmt>) -> String {
    stmts
        .iter()
        .map(|stmt| format_sexpr(&convert_stmt(stmt), 0))
        .collect::<Vec<String>>()
        .join("\n")
}
