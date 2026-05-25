use whipsnake::token::{Literal, Token, TokenKind};
use whipsnake::ast::{PrettyPrinter, Expr};

#[test]
fn test_pprint_unary_expr() {
    let unary = Expr::Unary {
        operator: Token::new(TokenKind::Minus, "-", 1),
        right: Box::new(Expr::Literal(Literal::Float(3.14))),
    };

    let p = PrettyPrinter;

    assert_eq!(p.pprint_expr(&unary), String::from("(- 3.14)"));
}