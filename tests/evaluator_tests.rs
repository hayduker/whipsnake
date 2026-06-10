use whipsnake::{
    ast::Expr,
    environment::Environment,
    error::ErrorReporter,
    evaluator::Evaluator,
    object::Object,
    token::{Literal, Token, TokenKind},
};

use common::*;

mod common;

macro_rules! test_no_errors {
    ($name:ident, $input:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let mut error_reporter = ErrorReporter::new();
            let interpreter = Evaluator::new(&mut error_reporter);
            let mut environment = Environment::new_global();
            match interpreter.evaluate(&$input, &mut environment) {
                Ok(value) => assert_eq!(value, $expected),
                Err(e) => {
                    eprintln!("RuntimeError: {:?}", e);
                    assert!(false);
                }
            }
        }
    };
}

test_no_errors!(
    evaluate_string_literal,
    Expr::Literal(Literal::String("hey".to_string())),
    Object::String(String::from("hey"))
);

test_no_errors!(
    evaluate_float_literal,
    Expr::Literal(Literal::Float(1.23)),
    Object::Float(1.23)
);

test_no_errors!(
    evaluate_true_literal,
    Expr::Literal(Literal::Bool(true)),
    Object::Bool(true)
);

test_no_errors!(
    evaluate_false_literal,
    Expr::Literal(Literal::Bool(false)),
    Object::Bool(false)
);

test_no_errors!(
    evaluate_none_literal,
    Expr::Literal(Literal::None),
    Object::None
);

test_no_errors!(
    evaluate_unary_plus,
    Expr::Unary {
        operator: tok!(Plus, "+", 1),
        right: expr_float_box(3.4)
    },
    Object::Float(3.4)
);

test_no_errors!(
    evaluate_unary_minus,
    Expr::Unary {
        operator: tok!(Minus, "-", 1),
        right: expr_float_box(3.4)
    },
    Object::Float(-3.4)
);

test_no_errors!(
    evaluate_unary_tilde_0,
    Expr::Unary {
        operator: tok!(Tilde, "~", 1),
        right: expr_int_box(0)
    },
    Object::Int(-1)
);

test_no_errors!(
    evaluate_unary_tilde_1,
    Expr::Unary {
        operator: tok!(Tilde, "~", 1),
        right: expr_int_box(9)
    },
    Object::Int(-10)
);

test_no_errors!(
    evaluate_unary_tilde_2,
    Expr::Unary {
        operator: tok!(Tilde, "~", 1),
        right: expr_int_box(-5)
    },
    Object::Int(4)
);

test_no_errors!(
    evaluate_plus_string,
    Expr::Binary {
        left: expr_string_box("oh"),
        operator: tok!(Plus, "+", 1),
        right: expr_string_box("io")
    },
    Object::String(String::from("ohio"))
);

test_no_errors!(
    evaluate_plus_int,
    Expr::Binary {
        left: expr_int_box(1),
        operator: tok!(Plus, "+", 1),
        right: expr_int_box(4)
    },
    Object::Int(5)
);

test_no_errors!(
    evaluate_plus_float,
    Expr::Binary {
        left: expr_float_box(1.2),
        operator: tok!(Plus, "+", 1),
        right: expr_float_box(4.5)
    },
    Object::Float(5.7)
);

test_no_errors!(
    evaluate_plus_int_float,
    Expr::Binary {
        left: expr_int_box(1),
        operator: tok!(Plus, "+", 1),
        right: expr_float_box(4.5)
    },
    Object::Float(5.5)
);

test_no_errors!(
    evaluate_minus_int,
    Expr::Binary {
        left: expr_int_box(4),
        operator: tok!(Minus, "-", 1),
        right: expr_int_box(2)
    },
    Object::Int(2)
);

test_no_errors!(
    evaluate_minus_float,
    Expr::Binary {
        left: expr_float_box(4.5),
        operator: tok!(Minus, "-", 1),
        right: expr_float_box(1.2)
    },
    Object::Float(3.3)
);

test_no_errors!(
    evaluate_minus_float_int,
    Expr::Binary {
        left: expr_float_box(4.3),
        operator: tok!(Minus, "-", 1),
        right: expr_int_box(2)
    },
    Object::Float(2.3)
);

test_no_errors!(
    evaluate_star_int,
    Expr::Binary {
        left: expr_int_box(4),
        operator: tok!(Star, "*", 1),
        right: expr_int_box(2)
    },
    Object::Int(8)
);

test_no_errors!(
    evaluate_star_float,
    Expr::Binary {
        left: expr_float_box(4.5),
        operator: tok!(Star, "*", 1),
        right: expr_float_box(2.0)
    },
    Object::Float(9.0)
);

test_no_errors!(
    evaluate_star_int_float,
    Expr::Binary {
        left: expr_int_box(4),
        operator: tok!(Star, "*", 1),
        right: expr_float_box(2.1)
    },
    Object::Float(8.4)
);

test_no_errors!(
    evaluate_slash_int,
    Expr::Binary {
        left: expr_int_box(9),
        operator: tok!(Slash, "/", 1),
        right: expr_int_box(2)
    },
    Object::Float(4.5)
);

test_no_errors!(
    evaluate_slash_float,
    Expr::Binary {
        left: expr_float_box(9.0),
        operator: tok!(Slash, "/", 1),
        right: expr_float_box(2.0)
    },
    Object::Float(4.5)
);

test_no_errors!(
    evaluate_greater,
    Expr::Binary {
        left: expr_float_box(3.4),
        operator: tok!(Greater, ">", 1),
        right: expr_float_box(7.2)
    },
    Object::Bool(false)
);

test_no_errors!(
    evaluate_greater_equal_0,
    Expr::Binary {
        left: expr_float_box(3.1),
        operator: tok!(GreaterEqual, ">=", 1),
        right: expr_float_box(3.1)
    },
    Object::Bool(true)
);

test_no_errors!(
    evaluate_greater_equal_1,
    Expr::Binary {
        left: expr_int_box(6),
        operator: tok!(GreaterEqual, ">=", 1),
        right: expr_int_box(5)
    },
    Object::Bool(true)
);

test_no_errors!(
    evaluate_less,
    Expr::Binary {
        left: expr_float_box(9.1),
        operator: tok!(Less, "<", 1),
        right: expr_float_box(3.1)
    },
    Object::Bool(false)
);

test_no_errors!(
    evaluate_less_equal_0,
    Expr::Binary {
        left: expr_float_box(3.0),
        operator: tok!(LessEqual, "<=", 1),
        right: expr_float_box(3.1)
    },
    Object::Bool(true)
);

test_no_errors!(
    evaluate_less_equal_1,
    Expr::Binary {
        left: expr_int_box(4),
        operator: tok!(LessEqual, "<=", 1),
        right: expr_int_box(4)
    },
    Object::Bool(true)
);

test_no_errors!(
    evaluate_equal_equal_same_type,
    Expr::Binary {
        left: expr_float_box(3.1),
        operator: tok!(EqualEqual, "==", 1),
        right: expr_float_box(3.1)
    },
    Object::Bool(true)
);

test_no_errors!(
    evaluate_equal_equal_different_type,
    Expr::Binary {
        left: expr_float_box(3.0),
        operator: tok!(EqualEqual, "==", 1),
        right: expr_string_box("hi")
    },
    Object::Bool(false)
);

test_no_errors!(
    evaluate_bang_equal_same_type,
    Expr::Binary {
        left: expr_int_box(3),
        operator: tok!(BangEqual, "!=", 1),
        right: expr_int_box(3)
    },
    Object::Bool(false)
);

test_no_errors!(
    evaluate_bang_equal_different_type,
    Expr::Binary {
        left: expr_float_box(3.0),
        operator: tok!(BangEqual, "!=", 1),
        right: expr_string_box("hi")
    },
    Object::Bool(true)
);

test_no_errors!(
    evaluate_not_bool,
    Expr::Unary {
        operator: tok!(Not, "not", 1),
        right: expr_none_box()
    },
    Object::Bool(true)
);

test_no_errors!(
    evaluate_not_truthy_int,
    Expr::Unary {
        operator: tok!(Not, "not", 1),
        right: expr_int_box(1)
    },
    Object::Bool(false)
);

test_no_errors!(
    evaluate_not_falsy_int,
    Expr::Unary {
        operator: tok!(Not, "not", 1),
        right: expr_int_box(0)
    },
    Object::Bool(true)
);

test_no_errors!(
    evaluate_not_truthy_float,
    Expr::Unary {
        operator: tok!(Not, "not", 1),
        right: expr_float_box(1.2)
    },
    Object::Bool(false)
);

test_no_errors!(
    evaluate_not_falsy_float,
    Expr::Unary {
        operator: tok!(Not, "not", 1),
        right: expr_float_box(0.0)
    },
    Object::Bool(true)
);

test_no_errors!(
    evaluate_and_false_first_int,
    Expr::Logical {
        left: expr_int_box(0),
        operator: tok!(And, "and", 1),
        right: expr_int_box(1)
    },
    Object::Int(0)
);

test_no_errors!(
    evaluate_and_false_second_int,
    Expr::Logical {
        left: expr_int_box(1),
        operator: tok!(And, "and", 1),
        right: expr_int_box(0)
    },
    Object::Int(0)
);

test_no_errors!(
    evaluate_and_true_ints,
    Expr::Logical {
        left: expr_int_box(1),
        operator: tok!(And, "and", 1),
        right: expr_int_box(2)
    },
    Object::Int(2)
);

test_no_errors!(
    evaluate_and_false_first_string,
    Expr::Logical {
        left: expr_string_box(""),
        operator: tok!(And, "and", 1),
        right: expr_string_box("hi")
    },
    Object::String(String::from(""))
);

test_no_errors!(
    evaluate_and_false_second_string,
    Expr::Logical {
        left: expr_string_box("hi"),
        operator: tok!(And, "and", 1),
        right: expr_string_box("")
    },
    Object::String(String::from(""))
);

test_no_errors!(
    evaluate_and_true_strings,
    Expr::Logical {
        left: expr_string_box("hi"),
        operator: tok!(And, "and", 1),
        right: expr_string_box("bye")
    },
    Object::String(String::from("bye"))
);

test_no_errors!(
    evaluate_and_false_first_bool,
    Expr::Logical {
        left: expr_false_box(),
        operator: tok!(And, "and", 1),
        right: expr_true_box()
    },
    Object::Bool(false)
);

test_no_errors!(
    evaluate_and_false_second_bool,
    Expr::Logical {
        left: expr_true_box(),
        operator: tok!(And, "and", 1),
        right: expr_false_box()
    },
    Object::Bool(false)
);

test_no_errors!(
    evaluate_and_true_bools,
    Expr::Logical {
        left: expr_true_box(),
        operator: tok!(And, "and", 1),
        right: expr_true_box()
    },
    Object::Bool(true)
);

test_no_errors!(
    evaluate_or_false_first_int,
    Expr::Logical {
        left: expr_int_box(0),
        operator: tok!(Or, "or", 1),
        right: expr_int_box(1)
    },
    Object::Int(1)
);

test_no_errors!(
    evaluate_or_false_second_int,
    Expr::Logical {
        left: expr_int_box(1),
        operator: tok!(Or, "or", 1),
        right: expr_int_box(0)
    },
    Object::Int(1)
);

test_no_errors!(
    evaluate_or_true_ints,
    Expr::Logical {
        left: expr_int_box(1),
        operator: tok!(Or, "or", 1),
        right: expr_int_box(2)
    },
    Object::Int(1)
);

test_no_errors!(
    evaluate_or_false_first_string,
    Expr::Logical {
        left: expr_string_box(""),
        operator: tok!(Or, "or", 1),
        right: expr_string_box("hi")
    },
    Object::String(String::from("hi"))
);

test_no_errors!(
    evaluate_or_false_second_string,
    Expr::Logical {
        left: expr_string_box("hi"),
        operator: tok!(Or, "or", 1),
        right: expr_string_box("")
    },
    Object::String(String::from("hi"))
);

test_no_errors!(
    evaluate_or_true_strings,
    Expr::Logical {
        left: expr_string_box("hi"),
        operator: tok!(Or, "or", 1),
        right: expr_string_box("bye")
    },
    Object::String(String::from("hi"))
);

test_no_errors!(
    evaluate_or_false_first_bool,
    Expr::Logical {
        left: expr_false_box(),
        operator: tok!(Or, "or", 1),
        right: expr_true_box()
    },
    Object::Bool(true)
);

test_no_errors!(
    evaluate_or_false_second_bool,
    Expr::Logical {
        left: expr_true_box(),
        operator: tok!(Or, "or", 1),
        right: expr_false_box()
    },
    Object::Bool(true)
);

test_no_errors!(
    evaluate_or_true_bools,
    Expr::Logical {
        left: expr_true_box(),
        operator: tok!(Or, "or", 1),
        right: expr_true_box()
    },
    Object::Bool(true)
);
