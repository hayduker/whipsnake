use whipsnake::{
    token::{Literal, Token, TokenKind},
    parser::Parser,
    error::ErrorReporter,
    ast::Expr,
};

mod common;

#[test]
fn test_parse_string_literal() {
    let mut reporter = ErrorReporter::new();
    let mut parser = Parser::new(&mut reporter);
    
    let mut tokens = vec![
        Token::with_literal(
            TokenKind::String,
            "\"hi\"",
            Literal::String("hi"),
            1
        )
    ].into_iter().peekable();

    let expr = parser.parse(&mut tokens);

    assert_eq!(
        Expr::Literal(Literal::String("hi")),
        expr
    );
}
