use whipsnake::token::{Literal, Token, TokenKind};
use whipsnake::ast::Expr;
use whipsnake::printer::PrettyPrinter;

mod common;

macro_rules! test_no_errors {
    ($name:ident, $input:expr, $expected:expr) => {
        #[test]
        fn $name() {
            assert_eq!(PrettyPrinter::print_expr(&$input), String::from($expected));
        }
    };
}

test_no_errors!(
    pretty_print_literal_string_expr,
    Expr::Literal(Literal::String("hey")),
    "\"hey\""
);

test_no_errors!(
    pretty_print_literal_float_expr,
    Expr::Literal(Literal::Float(1.2345)),
    "1.2345"
);

test_no_errors!(
    pretty_print_group_float_expr,
    Expr::Grouping(
        Box::new(Expr::Literal(Literal::Float(9.876)))
    ),
    "(group 9.876)"
);

test_no_errors!(
    pretty_print_unary_expr,
    Expr::Unary {
        operator: tok!(Minus, "-", 1),
        right: Box::new(Expr::Literal(Literal::Float(3.14))),
    },
    "(- 3.14)"
);

test_no_errors!(
    pretty_print_binary_expr,
    Expr::Binary {
        left: Box::new(Expr::Literal(Literal::Float(2.0))),
        operator: tok!(Star, "*", 1),
        right: Box::new(Expr::Literal(Literal::Float(5.1))),
    },
    "(* 2 5.1)"
);

test_no_errors!(
    pretty_print_nested_exprs,
    Expr::Binary {
        left: Box::new(Expr::Unary {
            operator: tok!(Minus, "-", 1),
            right: Box::new(Expr::Literal(Literal::Float(123.0))),
        }),
        operator: tok!(Star, "*", 1),
        right: Box::new(Expr::Grouping(
            Box::new(Expr::Literal(Literal::Float(45.67)))
        ))
    },
    "(* (- 123) (group 45.67))"
);