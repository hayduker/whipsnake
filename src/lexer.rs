use std::{iter::Peekable, str::CharIndices};

use crate::{
    error::{ErrorReporter, LexError},
    token::{Literal, Token, TokenKind, SourceLocation}
};

pub struct Lexer<'src, 'err> {
    source: &'src str,
    chars: Peekable<CharIndices<'src>>,
    start: usize,
    current: usize,
    line: usize,
    indent_levels: Vec<usize>,
    using_tabs: Option<bool>,
    error_reporter: &'err mut ErrorReporter,
}

impl<'src, 'err> Lexer<'src, 'err> {
    pub fn new(error_reporter: &'err mut ErrorReporter) -> Lexer<'src, 'err> {
        Lexer {
            source: "",
            chars: "".char_indices().peekable(),
            start: 0,
            current: 0,
            line: 1,
            indent_levels: Vec::new(),
            using_tabs: None,
            error_reporter,
        }
    }

    pub fn lex(&mut self, source: &'src str) -> Vec<Token<'src>> {
        self.source = source;
        self.chars = source.char_indices().peekable();

        let mut tokens = Vec::new();

        while !self.is_at_end() {
            self.start = self.current;

            match self.next_token_group() {
                Ok(Some(ts)) => {
                    tokens.extend(ts);
                }
                Ok(None) => { /* non-indentation whitespace or comment */ }
                Err(e) => self.error_reporter.register_lex_error(e),
            }
        }

        if tokens.len() > 0 {
            // Trailing newline and dedents are added in case file ends in
            // the middle of an indented block. This simplifies the parser.

            if tokens[tokens.len() - 1].kind != TokenKind::NewLine {
                tokens.push(Token::new(
                    TokenKind::NewLine,
                    "\n",
                    self.line
                ));

                self.line += 1;
            }

            for _ in self.indent_levels.iter() {
                tokens.push(Token::new(
                    TokenKind::Dedent,
                    "",
                    self.line
                ));
            }
            self.indent_levels.clear();
        }

        tokens.push(Token::new(
            TokenKind::Eof,
            "",
            self.line
        ));

        tokens
    }

    fn next_token_group(&mut self) -> Result<Option<Vec<Token<'src>>>, LexError> {
        let c = self.advance().unwrap();

        let kind = match c {
            '\n' => {
                let mut generated_tokens = vec![
                    Token::new(TokenKind::NewLine, "\n", self.line)
                ];

                self.line += 1;

                // After newlines, whitespace is semantic in Python
                match self.scan_indentation(&mut generated_tokens) {
                    Ok(()) => return Ok(Some(generated_tokens)),
                    Err(e) => return Err(e),
                }
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
                    return Err(LexError::UnexpectedCharacter(SourceLocation { line: self.line }, c));
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
            _ => return Err(LexError::UnexpectedCharacter(SourceLocation { line: self.line }, c)),
        };

        Ok(Some(vec![Token::new(
            kind,
            self.current_lexeme(),
            self.line,
        )]))
    }

    fn scan_indentation(
        &mut self,
        generated_tokens: &mut Vec<Token<'src>>
    ) -> Result<(), LexError> {
        println!("scan_indentation called");

        let mut num_spaces: usize = 0;
        let mut num_tabs: usize = 0;

        while self.peek() == Some(' ') || self.peek() == Some('\t') {
            if self.advance_if_match(' ') {
                num_spaces += 1;
            } else if self.advance_if_match('\t') {
                num_tabs += 1;
            }
        }

        println!("num_spaces = {num_spaces}, num_tabs = {num_tabs}");

        if (num_spaces > 0 && num_tabs > 0) ||
            (self.using_tabs == Some(true) && num_spaces > 0) ||
            (self.using_tabs == Some(false) && num_tabs > 0) {
            return Err(LexError::TabError(
                SourceLocation { line: self.line },
                String::from("mixed spaces and tabs for indentation.")
            ));
        }

        println!("no mixed spaces and tabs, cool");

        if self.using_tabs == None && (num_spaces > 0 || num_tabs > 0) {
            self.using_tabs = Some(num_tabs > 0);
            println!("initializing using_tabs to {:?}", self.using_tabs);
        }

        if self.using_tabs == Some(true) {
            num_spaces = num_tabs * 8;
            println!("got {} tabs which means {} spaces", num_tabs, num_spaces);
        }
        
        println!("scan_indentation line {} got num_spaces = {}", self.line, num_spaces);
        
        let mut last_level = *self.indent_levels.last().unwrap_or(&0);

        println!("  last_level = {last_level}");

        if num_spaces == last_level { println!("  same as last, we're done here"); return Ok(()); }

        if num_spaces > last_level {
            self.indent_levels.push(num_spaces);
            println!("  got more, pushing {}: {:?}", num_spaces, self.indent_levels);
            generated_tokens.push(
                Token::new(
                    TokenKind::Indent,
                    "",
                    self.line
                ));
            return Ok(());
        }
        
        // num_spaces < last_level
        loop {
            self.indent_levels.pop();
            println!("  got less, popping: {:?}", self.indent_levels);
            generated_tokens.push(
                Token::new(
                    TokenKind::Dedent,
                    "",
                    self.line
                ));

            last_level = *self.indent_levels.last().unwrap_or(&0);
            println!("  last level now {}", last_level);
            if last_level == num_spaces {
                // yay, we dedented enough
                println!("  yay, we dedented enough!");
                return Ok(());
            } else if num_spaces > last_level {
                println!("  oh no, we went to far, last_level = {}, num_spaces = {}", last_level, num_spaces);
                // oh no, we went to far
                return Err(LexError::IndentationError(
                    SourceLocation { line: self.line },
                    String::from("unindent does not match any outer indentation level.")
                ));
            }
        }
    }

    fn scan_comment(&mut self) -> Result<Option<Vec<Token<'src>>>, LexError> {
        while self.peek() != Some('\n') && !self.is_at_end() {
            self.advance();
        }
        return Ok(None);
    }

    fn scan_string_literal(&mut self) -> Result<Option<Vec<Token<'src>>>, LexError> {
        while self.peek() != Some('"') {
            if self.peek() == Some('\n') || self.is_at_end() {
                return Err(LexError::UnterminatedString(SourceLocation { line: self.line }));
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

    fn scan_number_literal(&mut self) -> Result<Option<Vec<Token<'src>>>, LexError> {
        while self.peek_is_digit() { self.advance(); }

        if self.peek() == Some('.') {
            self.advance();

            if !self.peek_is_digit() {
                return Err(LexError::MalformedNumberLiteral(SourceLocation { line: self.line }));
            }

            while self.peek_is_digit() { self.advance(); }
        }

        let float = self.current_lexeme().parse().expect(
            "Lexer guarantees a well-formed numeric value in earlier part of this method.",
        );

        Ok(Some(vec![Token::with_literal(
            TokenKind::Number,
            self.current_lexeme(),
            Literal::Float(float),
            self.line,
        )]))
    }

    fn peek_is_digit(&mut self) -> bool {
        self.peek().map_or(false, |c| self.is_digit(c))
    }

    fn scan_indentifier(&mut self) -> Result<Option<Vec<Token<'src>>>, LexError> {
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

    fn current_lexeme(&self) -> &'src str {
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
