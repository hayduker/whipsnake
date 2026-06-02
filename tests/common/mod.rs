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

pub fn tok_int<'src>(value: i64, line: usize) -> Token<'static> {
    let int_string = value.to_string();
    let static_lexeme: &'static str = Box::leak(int_string.into_boxed_str());

    Token::with_literal(TokenKind::Int, static_lexeme, Literal::Int(value), line)
}

pub fn tok_float<'src>(value: f64, line: usize) -> Token<'static> {
    let float_string = value.to_string();
    let static_lexeme: &'static str = Box::leak(float_string.into_boxed_str());

    Token::with_literal(TokenKind::Float, static_lexeme, Literal::Float(value), line)
}

pub fn tok_string<'src>(value: &'static str, line: usize) -> Token<'static> {
    let quoted_string = format!("\"{value}\"");
    let static_lexeme: &'static str = Box::leak(quoted_string.into_boxed_str());

    Token::with_literal(
        TokenKind::String,
        static_lexeme,
        Literal::String(value),
        line,
    )
}

pub fn tok_true<'src>(line: usize) -> Token<'src> {
    Token::new(TokenKind::True, "True", line)
}

pub fn tok_false<'src>(line: usize) -> Token<'src> {
    Token::new(TokenKind::False, "False", line)
}

pub fn expr_string<'src>(value: &'src str) -> Expr<'src> {
    Expr::Literal(Literal::String(value))
}

pub fn expr_string_box<'src>(value: &'src str) -> Box<Expr<'src>> {
    Box::new(expr_string(value))
}

pub fn expr_int<'src>(value: i64) -> Expr<'src> {
    Expr::Literal(Literal::Int(value))
}

pub fn expr_int_box<'src>(value: i64) -> Box<Expr<'src>> {
    Box::new(expr_int(value))
}

pub fn expr_float<'src>(value: f64) -> Expr<'src> {
    Expr::Literal(Literal::Float(value))
}

pub fn expr_float_box<'src>(value: f64) -> Box<Expr<'src>> {
    Box::new(expr_float(value))
}

pub fn expr_true<'src>() -> Expr<'src> {
    Expr::Literal(Literal::Bool(true))
}

pub fn expr_true_box<'src>() -> Box<Expr<'src>> {
    Box::new(expr_true())
}

pub fn expr_false<'src>() -> Expr<'src> {
    Expr::Literal(Literal::Bool(false))
}

pub fn expr_false_box<'src>() -> Box<Expr<'src>> {
    Box::new(expr_false())
}

pub fn expr_none<'src>() -> Expr<'src> {
    Expr::Literal(Literal::None)
}

pub fn expr_none_box<'src>() -> Box<Expr<'src>> {
    Box::new(expr_none())
}
