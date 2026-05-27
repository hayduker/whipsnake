use whipsnake::scanner::{Scanner, ScannerError};
use whipsnake::token::{Token, TokenKind};
use whipsnake::error::ErrorReporter;

mod common;

use common::*;

#[test]
fn scan_unexpected_character_error() {
    let mut reporter = ErrorReporter::new();
    let _: Vec<Token> = Scanner::new(
        "(1 + 2)&",
        &mut reporter
    ).map(|r| r).collect();

    assert_eq!(reporter.errors.len(), 1);
    assert!(matches!(reporter.errors.pop().unwrap(), ScannerError::UnexpectedCharacter(1, '&')));
}

#[test]
fn scan_unterminated_string_error() {
    let mut reporter = ErrorReporter::new();
    let _: Vec<Token> = Scanner::new(
        "x = 1\ny = \"hello, world!",
        &mut reporter
    ).map(|r| r).collect();

    assert_eq!(reporter.errors.len(), 1);
    assert!(matches!(reporter.errors.pop().unwrap(), ScannerError::UnterminatedString(2)));
}

#[test]
fn scan_too_many_indentations_error() {
    let mut reporter = ErrorReporter::new();
    let _: Vec<Token> = Scanner::new(
        "x\n    y\n            z",
        &mut reporter
    ).map(|r| r).collect();

    assert_eq!(reporter.errors.len(), 1);
    assert!(matches!(reporter.errors.pop().unwrap(), ScannerError::TooManyIndentations(3, 2)));
}

#[test]
fn scan_malformed_number_literal_error() {
    let mut reporter = ErrorReporter::new();
    let _: Vec<Token> = Scanner::new(
        "let x = 123.",
        &mut reporter
    ).map(|r| r).collect();

    assert_eq!(reporter.errors.len(), 1);
    assert!(matches!(reporter.errors.pop().unwrap(), ScannerError::MalformedNumberLiteral(1)));
}