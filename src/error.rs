use std::fmt;

use crate::token::SourceLocation;

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

#[derive(Debug)]
pub enum ParseError {
    ParseError(SourceLocation, String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::ParseError(location, message) => {
                write!(f, "ParseError at line {}: {}", location.line, message)
            },
        }
    }
}

#[derive(Debug)]
pub enum RuntimeError {
    TypeError(SourceLocation, String),
    NameError(SourceLocation, String),
    RuntimeError(SourceLocation, String),
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeError::TypeError(location, message) => {
                write!(f, "TypeError at line {}: {}", location.line, message)
            },
            RuntimeError::NameError(location, message) => {
                write!(f, "NameError at line {}: {}", location.line, message)
            }
            RuntimeError::RuntimeError(location, message) => {
                write!(f, "RuntimeError at line {}: {}", location.line, message)
            },
        }
    }
}

pub enum CompilerError {
    Lex(LexError),
    Parse(ParseError),
    Runtime(RuntimeError),
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

    pub fn register_parse_error(&mut self, error: ParseError) {
        self.register_error(CompilerError::Parse(error));
    }

    pub fn register_runtime_error(&mut self, error: RuntimeError) {
        self.register_error(CompilerError::Runtime(error));
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn print_errors(&self) {
        for err in &self.errors {
            match err {
                CompilerError::Lex(error) => eprintln!("{error}"),
                CompilerError::Parse(error) => eprintln!("{error}"),
                CompilerError::Runtime(error) => eprintln!("{error}"),
            }
        }
    }

    pub fn clear(&mut self) {
        self.errors.clear();
    }
}
