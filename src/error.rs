use crate::lexer::LexerError;

pub struct ErrorReporter {
    pub errors: Vec<LexerError>,
}

impl ErrorReporter {
    pub fn new() -> Self {
        ErrorReporter { errors: Vec::new() }
    }

    pub fn register_error(&mut self, error: LexerError) {
        self.errors.push(error);
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn print_errors(&self) {
        for err in self.errors.as_slice() {
            match err {
                LexerError::UnexpectedCharacter(l, c) => {
                    eprintln!("LexerError: unexpected character {c} at line {l}");
                },
                LexerError::TooManyIndentations(l, n) => {
                    eprintln!(
                        "LexerError: too many indentations at line {l}, got {n} more than previous line"
                    );
                },
                LexerError::UnterminatedString(l) => {
                    eprintln!("LexerError: unterminated string at line {l}");
                },
                LexerError::MalformedNumberLiteral(l) => {
                    eprintln!("LexerError: malformed number literal at line {l}");
                },
            }
        }
    }
}
