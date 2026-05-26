use crate::scanner::ScannerError;

pub struct ErrorReporter {
    pub errors: Vec<ScannerError>,
}

impl ErrorReporter {
    pub fn new() -> Self {
        ErrorReporter { errors: Vec::new() }
    }

    pub fn register_error(&mut self, error: ScannerError) {
        self.errors.push(error);
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn print_errors(&self) {
        for err in self.errors.as_slice() {
            match err {
                ScannerError::UnexpectedCharacter(l, c) => {
                    eprintln!("ScannerError: unexpected character {c} at line {l}");
                },
                ScannerError::TooManyIndentations(l, n) => {
                    eprintln!(
                        "ScannerError: too many indentations at line {l}, got {n} more than previous line"
                    );
                },
                ScannerError::UnterminatedString(l) => {
                    eprintln!("ScannerError: unterminated string at line {l}");
                },
                ScannerError::MalformedNumberLiteral(l) => {
                    eprintln!("ScannerError: malformed number literal at line {l}");
                },
            }
        }
    }
}
