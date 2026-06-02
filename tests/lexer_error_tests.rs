use whipsnake::error::{CompilerError, ErrorReporter, LexError};
use whipsnake::lexer::Lexer;
use whipsnake::token::SourceLocation;

mod common;

#[test]
fn lex_unexpected_character_error() {
    let mut reporter = ErrorReporter::new();
    let mut lexer = Lexer::new(&mut reporter);
    lexer.lex("(1 + 2)&");

    assert_eq!(reporter.errors.len(), 1);
    assert!(matches!(
        reporter.errors.pop().unwrap(),
        CompilerError::Lex(LexError::UnexpectedCharacter(
            SourceLocation { line: 1 },
            '&'
        ))
    ));
}

#[test]
fn lex_unterminated_string_error() {
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
fn lex_mismatched_indentation_error() {
    let mut reporter = ErrorReporter::new();
    let mut lexer = Lexer::new(&mut reporter);
    lexer.lex("w\n    x\n        y\n      z");

    assert_eq!(reporter.errors.len(), 1);
    assert_eq!(
        reporter.errors.pop().unwrap(),
        CompilerError::Lex(LexError::IndentationError(
            SourceLocation { line: 4 },
            String::from("unindent does not match any outer indentation level.")
        ))
    );
}

#[test]
fn lex_tab_error_0() {
    let mut reporter = ErrorReporter::new();
    let mut lexer = Lexer::new(&mut reporter);
    lexer.lex("w\n\tx\n        y\n\t\tz");

    assert_eq!(reporter.errors.len(), 1);
    assert_eq!(
        reporter.errors.pop().unwrap(),
        CompilerError::Lex(LexError::TabError(
            SourceLocation { line: 3 },
            String::from("mixed spaces and tabs for indentation.")
        ))
    );
}

#[test]
fn lex_tab_error_1() {
    let mut reporter = ErrorReporter::new();
    let mut lexer = Lexer::new(&mut reporter);
    lexer.lex("w\n    x\n\t\ty\n        z");

    assert_eq!(reporter.errors.len(), 1);
    assert_eq!(
        reporter.errors.pop().unwrap(),
        CompilerError::Lex(LexError::TabError(
            SourceLocation { line: 3 },
            String::from("mixed spaces and tabs for indentation.")
        ))
    );
}

#[test]
fn lex_tab_error_2() {
    let mut reporter = ErrorReporter::new();
    let mut lexer = Lexer::new(&mut reporter);
    lexer.lex("w\n\t    ");

    assert_eq!(reporter.errors.len(), 1);
    assert_eq!(
        reporter.errors.pop().unwrap(),
        CompilerError::Lex(LexError::TabError(
            SourceLocation { line: 2 },
            String::from("mixed spaces and tabs for indentation.")
        ))
    );
}

#[test]
fn lex_malformed_number_literal_error() {
    let mut reporter = ErrorReporter::new();
    let mut lexer = Lexer::new(&mut reporter);
    lexer.lex("let x = 123.");

    assert_eq!(reporter.errors.len(), 1);
    assert!(matches!(
        reporter.errors.pop().unwrap(),
        CompilerError::Lex(LexError::MalformedNumberLiteral(SourceLocation { line: 1 }))
    ));
}
