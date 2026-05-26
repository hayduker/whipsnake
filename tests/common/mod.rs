// #![allow(dead_code)]

#[macro_export]
macro_rules! tok {
    ($kind:ident, $lexeme:expr, $line:expr) => {
        Token::new(TokenKind::$kind, $lexeme, $line)
    };
}

#[macro_export]
macro_rules! tok_with_literal {
    ($kind:ident, $lexeme:expr, $lit:ident, $line:expr) => {
        Token::with_literal(TokenKind::$kind, $lexeme, Literal::$lit, $line)
    };
}