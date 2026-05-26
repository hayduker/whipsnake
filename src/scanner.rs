use std::{collections::VecDeque, iter::Peekable, str::CharIndices};

use crate::{error::ErrorReporter, token::{Literal, Token, TokenKind}};

#[derive(Debug, PartialEq)]
pub enum ScannerError {
    UnexpectedCharacter(usize, char),
    UnterminatedString(usize),
    TooManyIndentations(usize, usize),
    MalformedNumberLiteral(usize),
}

pub struct Scanner<'a, 'b> {
    source: &'a str,
    chars: Peekable<CharIndices<'a>>,
    start: usize,
    current: usize,
    line: usize,
    indent_level: usize,
    token_buffer: VecDeque<Token<'a>>,
    is_done: bool,
    error_reporter: &'b mut ErrorReporter,
}

impl<'a, 'b> Iterator for Scanner<'a, 'b> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.token_buffer.is_empty() {
            return Some(self.token_buffer.pop_front().unwrap());
        }

        if self.is_done {
            return None;
        }

        while !self.is_at_end() {
            self.start = self.current;

            match self.next_token_group() {
                Ok(Some(tokens)) => {
                    self.token_buffer.extend(tokens);
                    if !self.token_buffer.is_empty() {
                        return Some(self.token_buffer.pop_front().unwrap());
                    }
                }
                Ok(None) => { /* non-indentation whitespace or comment */ }
                Err(e) => self.error_reporter.register_error(e),
            }
        }

        self.is_done = true;
        Some(Token {
            kind: TokenKind::Eof,
            lexeme: "",
            literal: Literal::None,
            line: self.line,
        })
    }
}

impl<'a, 'b> Scanner<'a, 'b> {
    pub fn new(source: &'a str, error_reporter: &'b mut ErrorReporter) -> Scanner<'a, 'b> {
        Scanner {
            source,
            chars: source.char_indices().peekable(),
            start: 0,
            current: 0,
            line: 1,
            indent_level: 0,
            token_buffer: VecDeque::new(),
            is_done: false,
            error_reporter,
        }
    }

    fn next_token_group(&mut self) -> Result<Option<Vec<Token<'a>>>, ScannerError> {
        let c = self.advance().unwrap();

        let kind = match c {
            '\n' => {
                self.line += 1;
                // after newlines we need to consider beginning-of-line whitespace
                // since python uses semantic indentation
                return self.scan_indentation();
            }
            // beginning-of-line indentation is consumed with self.scan_indentation
            // in '\n' pattern, so this is whitespace elsewhere in the line
            ' ' | '\t' | '\r' => return Ok(None),
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            ':' => TokenKind::Colon,
            ',' => TokenKind::Comma,
            '.' => TokenKind::Dot,
            '-' => TokenKind::Minus,
            '+' => TokenKind::Plus,
            '/' => TokenKind::Slash,
            '*' => TokenKind::Star,
            '!' => {
                if self.advance_if_match('=') {
                    TokenKind::BangEqual
                } else {
                    return Err(ScannerError::UnexpectedCharacter(self.line, c));
                }
            }
            '=' => {
                if self.advance_if_match('=') {
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
                }
            }
            '<' => {
                if self.advance_if_match('=') {
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                }
            }
            '>' => {
                if self.advance_if_match('=') {
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                }
            }
            '#' => return self.scan_comment(),
            '"' => return self.scan_string_literal(),
            '0'..='9' => return self.scan_number_literal(),
            'a'..='z' | 'A'..='Z' | '_' => return self.scan_indentifier(),
            _ => return Err(ScannerError::UnexpectedCharacter(self.line, c)),
        };

        Ok(Some(vec![Token::new(
            kind,
            self.current_lexeme(),
            self.line,
        )]))
    }

    fn scan_indentation(&mut self) -> Result<Option<Vec<Token<'a>>>, ScannerError> {
        let mut num_spaces: usize = 0;
        while self.advance_if_match(' ') {
            num_spaces += 1
        }
        let level = num_spaces / 4;

        // println!("num_spaces = {num_spaces}, new level = {}, old level = {}", level, self.indent_level);

        let mut generated_tokens = vec![];

        if level == self.indent_level + 1 {
            generated_tokens.push(
                Token::new(
                    TokenKind::Indent,
                    "",
                    self.line
                ));
        } else if level < self.indent_level {
            let num_dedents = self.indent_level - level;
            for _ in 0..num_dedents {
                generated_tokens.push(
                    Token::new(
                        TokenKind::Dedent,
                        "",
                        self.line
                    ));
            }
        } else if level != self.indent_level {
            let how_many = level - self.indent_level;
            return Err(ScannerError::TooManyIndentations(self.line, how_many));
        }
        self.indent_level = level;

        if generated_tokens.is_empty() {
            Ok(None)
        } else {
            Ok(Some(generated_tokens))
        }
    }

    fn scan_comment(&mut self) -> Result<Option<Vec<Token<'a>>>, ScannerError> {
        while self.peek() != Some('\n') && !self.is_at_end() {
            self.advance();
        }
        return Ok(None);
    }

    fn scan_string_literal(&mut self) -> Result<Option<Vec<Token<'a>>>, ScannerError> {
        while self.peek() != Some('"') {
            if self.peek() == Some('\n') || self.is_at_end() {
                return Err(ScannerError::UnterminatedString(self.line));
            }
            self.advance();
        }

        self.advance(); // eat the closing "

        let lexeme = self.current_lexeme();
        let literal = lexeme.get(1..lexeme.len() - 1)
            .expect("String literal should be the same as the lexeme without the quotes on either side");

        return Ok(Some(vec![Token::with_literal(
            TokenKind::String,
            lexeme,
            Literal::String(literal),
            self.line,
        )]));
    }

    fn scan_number_literal(&mut self) -> Result<Option<Vec<Token<'a>>>, ScannerError> {
        while self.peek_is_digit() { self.advance(); }

        println!("got first (and maybe only) digit clump");

        if self.peek() == Some('.') {
            self.advance();

            if !self.peek_is_digit() {
                return Err(ScannerError::MalformedNumberLiteral(self.line));
            }

            while self.peek_is_digit() { self.advance(); }
        }

        let float = self.current_lexeme().parse().expect(
            "Scanner guarantees a well-formed numeric value in earlier part of this method.",
        );

        Ok(Some(vec![Token::with_literal(
            TokenKind::Number,
            self.current_lexeme(),
            Literal::Float(float),
            self.line,
        )]))
    }

    fn peek_is_digit(&mut self) -> bool {
        // println!("peek_is_digit called");
        self.peek().map_or(false, |c| self.is_digit(c))
    }

    // fn peek_next_is_digit(&mut self) -> bool {
    //     self.peek_next().map_or(false, |c| self.is_digit(c))
    // }

    fn scan_indentifier(&mut self) -> Result<Option<Vec<Token<'a>>>, ScannerError> {
        while self.peek().map_or(false, |c| self.is_alpha_numeric(c)) {
            self.advance();
        }

        let kind = self.lookup_keyword(
            &self.current_lexeme(),
        ).unwrap_or(TokenKind::Identifier);

        Ok(Some(vec![Token::new(
            kind,
            self.current_lexeme(),
            self.line
        )]))
    }

    fn is_alpha(&self, c: char) -> bool {
        match c {
            'a'..='z' | 'A'..='Z' | '_' => true,
            _ => false,
        }
    }

    fn is_digit(&self, c: char) -> bool {
        match c {
            '0'..='9' => true,
            _ => false,
        }
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    fn advance(&mut self) -> Option<char> {
        if let Some((idx, c)) = self.chars.next() {
            // update current to the *end* byte of this character
            self.current = idx + c.len_utf8();
            Some(c)
        } else {
            self.current = self.source.len();
            None
        }
    }

    fn advance_if_match(&mut self, expected: char) -> bool {
        if let Some((idx, c)) = self.chars.next_if(|&(_, c)| c == expected) {
            // update current to the *end* byte of this character
            self.current = idx + c.len_utf8();
            true
        } else {
            if self.chars.peek().is_none() {
                self.current = self.source.len();
            }
            false
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().map(|&(_, c)| c)
    }

    fn current_lexeme(&self) -> &'a str {
        &self.source[self.start..self.current]
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn lookup_keyword(&self, identifier: &str) -> Option<TokenKind> {
        match identifier {
            "and"    => Some(TokenKind::And),
            "class"  => Some(TokenKind::Class),
            "def"    => Some(TokenKind::Def),
            "elif"   => Some(TokenKind::Elif),
            "else"   => Some(TokenKind::Else),
            "False"  => Some(TokenKind::False), // really a literal
            "for"    => Some(TokenKind::For),
            "if"     => Some(TokenKind::If),
            "None"   => Some(TokenKind::None), // really a literal
            "not"    => Some(TokenKind::Not),
            "or"     => Some(TokenKind::Or),
            "print"  => Some(TokenKind::Print),
            "return" => Some(TokenKind::Return),
            "super"  => Some(TokenKind::Super),
            "self"   => Some(TokenKind::This),
            "True"   => Some(TokenKind::True), // really a literal
            "while"  => Some(TokenKind::While),
            _        => None,
        }
    }

}
