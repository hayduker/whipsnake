#![allow(dead_code)]

use whipsnake::{
    ast::Expr,
    token::{Literal, Token, TokenKind},
};

#[macro_export]
macro_rules! tok {
    ($kind:ident, $lexeme:expr, $line:expr) => {
        Token::new(TokenKind::$kind, $lexeme, $line)
    };
}

pub fn tok_int(value: i64, line: usize) -> Token {
    let int_string = value.to_string();
    let static_lexeme: &'static str = Box::leak(int_string.into_boxed_str());

    Token::with_literal(TokenKind::Int, static_lexeme, Literal::Int(value), line)
}

pub fn tok_float(value: f64, line: usize) -> Token {
    let float_string = value.to_string();
    let static_lexeme: &'static str = Box::leak(float_string.into_boxed_str());

    Token::with_literal(TokenKind::Float, static_lexeme, Literal::Float(value), line)
}

pub fn tok_string(value: &'static str, line: usize) -> Token {
    let quoted_string = format!("\"{value}\"");
    let static_lexeme: &'static str = Box::leak(quoted_string.into_boxed_str());

    Token::with_literal(
        TokenKind::String,
        static_lexeme,
        Literal::String(value.to_string()),
        line,
    )
}

pub fn tok_sq_string(value: &'static str, line: usize) -> Token {
    let quoted_string = format!("'{value}'");
    let static_lexeme: &'static str = Box::leak(quoted_string.into_boxed_str());

    Token::with_literal(
        TokenKind::String,
        static_lexeme,
        Literal::String(value.to_string()),
        line,
    )
}

pub fn tok_true(line: usize) -> Token {
    Token::new(TokenKind::True, "True", line)
}

pub fn tok_false(line: usize) -> Token {
    Token::new(TokenKind::False, "False", line)
}

pub fn expr_string(value: &str) -> Expr {
    Expr::Literal(Literal::String(value.to_string()))
}

pub fn expr_string_box(value: &str) -> Box<Expr> {
    Box::new(expr_string(value))
}

pub fn expr_int(value: i64) -> Expr {
    Expr::Literal(Literal::Int(value))
}

pub fn expr_int_box(value: i64) -> Box<Expr> {
    Box::new(expr_int(value))
}

pub fn expr_float(value: f64) -> Expr {
    Expr::Literal(Literal::Float(value))
}

pub fn expr_float_box(value: f64) -> Box<Expr> {
    Box::new(expr_float(value))
}

pub fn expr_true() -> Expr {
    Expr::Literal(Literal::Bool(true))
}

pub fn expr_true_box() -> Box<Expr> {
    Box::new(expr_true())
}

pub fn expr_false() -> Expr {
    Expr::Literal(Literal::Bool(false))
}

pub fn expr_false_box() -> Box<Expr> {
    Box::new(expr_false())
}

pub fn expr_none() -> Expr {
    Expr::Literal(Literal::None)
}

pub fn expr_none_box() -> Box<Expr> {
    Box::new(expr_none())
}
