use whipsnake::lexer::Lexer;
use whipsnake::token::SourceLocation;
use whipsnake::error::{ErrorReporter, CompilerError, LexError};

mod common;

#[test]
fn scan_unexpected_character_error() {
    let mut reporter = ErrorReporter::new();
    let mut lexer = Lexer::new(&mut reporter);
    lexer.lex("(1 + 2)&");

    assert_eq!(reporter.errors.len(), 1);
    assert!(matches!(
        reporter.errors.pop().unwrap(),
        CompilerError::Lex(LexError::UnexpectedCharacter(SourceLocation { line: 1 }, '&'))
    ));
}

#[test]
fn scan_unterminated_string_error() {
    let mut reporter = ErrorReporter::new();
    let mut lexer = Lexer::new(&mut reporter);
    lexer.lex("x = 1\ny = \"hello, world!");

    assert_eq!(reporter.errors.len(), 1);
    assert!(matches!(
        reporter.errors.pop().unwrap(),
        CompilerError::Lex(LexError::UnterminatedString(SourceLocation { line: 2 }))
    ));
}

#[test]
fn scan_too_many_indentations_error() {
    let mut reporter = ErrorReporter::new();
    let mut lexer = Lexer::new(&mut reporter);
    lexer.lex("x\n    y\n            z");

    assert_eq!(reporter.errors.len(), 1);
    assert!(matches!(
        reporter.errors.pop().unwrap(),
        CompilerError::Lex(LexError::TooManyIndentations(SourceLocation { line: 3 }, 2))
    ));
}

#[test]
fn scan_malformed_number_literal_error() {
    let mut reporter = ErrorReporter::new();
    let mut lexer = Lexer::new(&mut reporter);
    lexer.lex("let x = 123.");

    assert_eq!(reporter.errors.len(), 1);
    assert!(matches!(
        reporter.errors.pop().unwrap(),
        CompilerError::Lex(LexError::MalformedNumberLiteral(SourceLocation { line: 1 }))
    ));
}