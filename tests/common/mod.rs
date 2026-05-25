// #![allow(dead_code)]

#[macro_export]
macro_rules! tok {
    ($kind:ident, $lexeme:expr, $line:expr) => {
        Token::new(TokenKind::$kind, $lexeme, $line)
    };
}