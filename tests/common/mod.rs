#![allow(dead_code)]

use whipsnake::token::{Token, TokenKind, Literal};

#[macro_export]
macro_rules! tok {
    ($kind:ident, $lexeme:expr, $line:expr) => {
        Token::new(TokenKind::$kind, $lexeme, $line)
    };
}

pub fn tok_float<'src>(value: f64, line: usize) -> Token<'static> {
    let float_string = value.to_string();
    let static_lexeme : &'static str = Box::leak(float_string.into_boxed_str());

    Token::with_literal(
        TokenKind::Number,
        static_lexeme,
        Literal::Float(value),
        line
    )
}

pub fn tok_string<'src>(value: &'static str, line: usize) -> Token<'static> {
    let quoted_string = format!("\"{value}\"");
    let static_lexeme: &'static str = Box::leak(quoted_string.into_boxed_str());
    
    Token::with_literal(
        TokenKind::String,
        static_lexeme,
        Literal::String(value),
        line
    )
}

pub fn tok_true<'src>(line: usize) -> Token<'src> {
    Token::new(
        TokenKind::True,
        "True",
        line
    )
}

pub fn tok_false<'src>(line: usize) -> Token<'src> {
    Token::new(
        TokenKind::False,
        "False",
        line
    )
}

pub fn tok_eof<'src>(line: usize) -> Token<'src> {
    Token::new(
        TokenKind::Eof,
        "",
        line
    )
}