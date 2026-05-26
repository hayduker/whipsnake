#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
    // single-character tokens
    LeftParen,
    RightParen,
    Colon,
    Comma,
    Dot,
    Minus,
    Plus,
    Slash,
    Star,

    // one or two character tokens
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // literals
    Identifier,
    String,
    Number,

    // keywords
    And,
    Class,
    Def,
    Elif,
    Else,
    False,
    For,
    If,
    None,
    Not,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    While,

    // python uses whitespace, I guess
    Indent,
    Dedent,

    // makes the parser cleaner
    Eof,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Literal<'a> {
    None,
    String(&'a str),
    Float(f64),
    Bool(bool),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub lexeme: &'a str,
    pub literal: Literal<'a>,
    pub line: usize,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind, lexeme: &'a str, line: usize) -> Self {
        Self::with_literal(kind, lexeme, Literal::None, line)
    }

    pub fn with_literal(
        kind: TokenKind,
        lexeme: &'a str,
        literal: Literal<'a>,
        line: usize,
    ) -> Token<'a> {
        Self {
            kind,
            lexeme,
            literal,
            line,
        }
    }
}
