// These test just verify that the test harness infrastructure is working
// as expected, e.g. macros for mock token creation

use whipsnake::token::{Literal, Token, TokenKind};

mod common;

use common::*;

#[test]
fn tok_macro() {
    assert_eq!(
        tok!(And, "and", 42),
        Token::with_literal(TokenKind::And, "and", Literal::None, 42)
    )
}

#[test]
fn tok_float_fn() {
    assert_eq!(
        tok_float(1.234, 1),
        Token::with_literal(TokenKind::Number, "1.234", Literal::Float(1.234), 1)
    )
}

#[test]
fn tok_string_fn() {
    assert_eq!(
        tok_string("hello", 1),
        Token::with_literal(TokenKind::String, "\"hello\"", Literal::String("hello"), 1)
    )
}

#[test]
fn tok_true_fn() {
    assert_eq!(
        tok_true(9),
        Token::new(TokenKind::True, "True", 9)
    )
}

#[test]
fn tok_false_fn() {
    assert_eq!(
        tok_false(12),
        Token::new(TokenKind::False, "False", 12)
    )
}

#[test]
fn tok_eof_macro() {
    assert_eq!(
        tok_eof(33),
        Token::with_literal(TokenKind::Eof, "", Literal::None, 33)
    )
}