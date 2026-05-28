use std::fmt;

use crate::token::SourceLocation;

// #[derive(Debug, PartialEq)]
pub enum LexError {
    UnexpectedCharacter(SourceLocation,char),
    UnterminatedString(SourceLocation),
    TooManyIndentations(SourceLocation, usize),
    MalformedNumberLiteral(SourceLocation),
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexError::UnexpectedCharacter(l, c) => {
                write!(f, "unexpected character '{}' at line {}", c, l.line)
            },
            LexError::TooManyIndentations(l, n) => {
                write!(f, "too many indentations on line {}, got {} more than previous line", l.line, n)
            },
            LexError::UnterminatedString(l) => {
                write!(f, "unterminated string at line {}", l.line)
            },
            LexError::MalformedNumberLiteral(l) => {
                write!(f, "malformed number literal at line {}", l.line)
            },
        }
    }
}

pub enum CompilerError {
    Lex(LexError),
    // Parse { location: usize, error: ParseError }, 
    // Runtime { lcoation: usize, error: RuntimeError },
}

pub struct ErrorReporter {
    pub errors: Vec<CompilerError>
}

impl ErrorReporter {
    pub fn new() -> Self {
        ErrorReporter { errors: Vec::new() }
    }

    fn register_error(&mut self, error: CompilerError) {
        self.errors.push(error);
    }

    pub fn register_lex_error(&mut self, error: LexError) {
        self.register_error(CompilerError::Lex(error));
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn print_errors(&self) {
        for err in &self.errors {
            match err {
                CompilerError::Lex(error) => eprintln!("LexError: {error}"),
            }
        }
    }
}
