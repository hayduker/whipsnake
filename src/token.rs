#[derive(Debug, PartialEq)]
pub struct SourceLocation {
    pub line: usize,
}

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
    Tilde,

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
    Int,
    Float,

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
    NewLine,

    // makes the parser cleaner
    Eof,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Literal<'src> {
    None,
    String(&'src str),
    Int(i64),
    Float(f64),
    Bool(bool),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Token<'src> {
    pub kind: TokenKind,
    pub lexeme: &'src str,
    pub literal: Literal<'src>,
    pub line: usize,
}

impl<'src> Token<'src> {
    pub fn new(kind: TokenKind, lexeme: &'src str, line: usize) -> Self {
        Self::with_literal(kind, lexeme, Literal::None, line)
    }

    pub fn with_literal(
        kind: TokenKind,
        lexeme: &'src str,
        literal: Literal<'src>,
        line: usize,
    ) -> Token<'src> {
        Self {
            kind,
            lexeme,
            literal,
            line,
        }
    }
}
