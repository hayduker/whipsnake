use whipsnake::{
    environment::Environment, error::ErrorReporter, evaluator::Evaluator, lexer::Lexer,
    object::Object, parser::Parser,
};

macro_rules! test_case {
    ($name:ident, $input:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let mut reporter = ErrorReporter::new();
            let mut environment = Environment::new_global();

            let mut lexer = Lexer::new(&mut reporter);
            let tokens = lexer.lex($input);

            if reporter.has_errors() {
                reporter.print_errors();
                assert!(false);
            }

            let mut parser = Parser::new(&mut reporter);
            let statements = parser.parse(&mut tokens.into_iter().peekable());

            if reporter.has_errors() {
                reporter.print_errors();
                assert!(false);
            }

            let mut evaluator = Evaluator::new(&mut reporter);
            let value = evaluator.interpret(&statements, &mut environment, true).unwrap();

            if reporter.has_errors() {
                reporter.print_errors();
                assert!(false);
            }

            assert_eq!(value, $expected);
        }
    };
}

// =======================================================
// Literals
// =======================================================

test_case!(
    interpret_string_literal,
    "\"Hello, world!\"",
    Object::String("Hello, world!".to_string())
);

test_case!(interpret_int_literal, "99", Object::Int(99));

test_case!(interpret_float_literal, "1.23", Object::Float(1.23));

test_case!(interpret_true_literal, "True", Object::Bool(true));

test_case!(interpret_false_literal, "False", Object::Bool(false));

// =======================================================
// Single-Quoted Strings
// =======================================================

test_case!(
    single_quoted_basic,
    r#" 'hello world' "#,
    // A standard single-quoted string should evaluate to a string object
    Object::String("hello world".into())
);

test_case!(
    single_quoted_empty,
    r#" '' "#,
    // An empty single-quoted string should be supported seamlessly
    Object::String("".into())
);

test_case!(
    single_quoted_containing_double_quotes,
    r#" 'He said, "Hello there!"' "#,
    // Double quotes inside a single-quoted string should be treated as literal characters
    Object::String("He said, \"Hello there!\"".into())
);

test_case!(
    double_quoted_containing_single_quotes,
    r#" "It's a beautiful day" "#,
    // Conversely, a single quote (apostrophe) inside a double-quoted string is literal
    Object::String("It's a beautiful day".into())
);

test_case!(
    single_quoted_equality,
    r#" 'abc' == "abc" "#,
    // A string defined with single quotes must equal an identical string defined with double quotes
    Object::Bool(true)
);

// test_case!(
//     single_quoted_escape_sequence,
//     r#" 'line one\nline two' "#,
//     // Escape sequences should behave the same way inside single quotes
//     Object::String("line one\nline two".into())
// );

// =======================================================
// Logical NOT Tests (Always returns a Bool)
// =======================================================

test_case!(logical_not_boolean, r#"not True"#, Object::Bool(false));

test_case!(logical_not_truthy_int, r#"not 42"#, Object::Bool(false));

test_case!(logical_not_falsy_int, r#"not 0"#, Object::Bool(true));

test_case!(logical_not_truthy_float, r#"not 3.14"#, Object::Bool(false));

test_case!(logical_not_falsy_float, r#"not 0.0"#, Object::Bool(true));

test_case!(
    logical_not_truthy_string,
    r#"not "hello""#,
    Object::Bool(false)
);

test_case!(logical_not_falsy_string, r#"not """#, Object::Bool(true));

test_case!(logical_not_none, r#"not None"#, Object::Bool(true));

// =======================================================
// Logical AND Tests (Short-circuits & Returns Object)
// =======================================================

test_case!(
    logical_and_both_true,
    r#"True and "valid""#,
    Object::String("valid".to_string())
);

test_case!(
    logical_and_first_falsy,
    r#"0 and "ignored""#,
    Object::Int(0)
);

test_case!(
    logical_and_second_falsy,
    r#""hello" and None"#,
    Object::None
);

test_case!(
    logical_and_mixed_floats,
    r#"1.5 and 0.0"#,
    Object::Float(0.0)
);

// =======================================================
// Logical OR Tests (Short-circuits & Returns Object)
// =======================================================

test_case!(
    logical_or_first_truthy,
    r#""fallback" or True"#, // First operand is truthy -> short-circuits and returns it
    Object::String("fallback".to_string())
);

test_case!(
    logical_or_first_falsy,
    r#"None or 4.5"#, // First operand falsy -> evaluates and returns second
    Object::Float(4.5)
);

test_case!(
    logical_or_both_falsy,
    r#""" or 0"#, // First operand falsy -> returns second
    Object::Int(0)
);

// =======================================================
// Precedence & Complex Short-Circuit Chaining
// =======================================================

test_case!(
    precedence_not_before_and,
    r#"not False and "yes""#, // (not False) -> True and "yes" -> "yes"
    Object::String("yes".to_string())
);

test_case!(
    precedence_and_before_or,
    r#""first" or "middle" and False"#,
    // "middle" and False evaluates first -> False
    // "first" or False evaluates second -> "first"
    Object::String("first".to_string())
);

test_case!(
    logical_precedence_with_math,
    r#"not 5 - 5 or 10 * 0"#,
    // Step 1 (Math): 5 - 5 = 0, 10 * 0 = 0
    // Step 2 (Not):  not 0 -> True
    // Step 3 (Or):   True or 0 -> True (short-circuits!)
    Object::Bool(true)
);

test_case!(
    logical_grouping_overrides,
    r#"(None or 0 or "winner") and True"#,
    // None or 0 -> 0
    // 0 or "winner" -> "winner"
    // "winner" and True -> True
    Object::Bool(true)
);

// =======================================================
// Unary Plus (+) Tests
// =======================================================

test_case!(unary_plus_integer, r#"+5"#, Object::Int(5));

test_case!(unary_plus_float, r#"+3.14"#, Object::Float(3.14));

test_case!(unary_plus_negative_integer, r#"+-10"#, Object::Int(-10));

// =======================================================
// Unary Minus (-) Tests
// =======================================================

test_case!(unary_minus_integer, r#"-42"#, Object::Int(-42));

test_case!(unary_minus_float, r#"-2.5"#, Object::Float(-2.5));

test_case!(unary_minus_negate_negative, r#"-(-100)"#, Object::Int(100));

// =======================================================
// Bitwise NOT (~) Tests
// =======================================================

test_case!(bitwise_not_zero, r#"~0"#, Object::Int(-1));

test_case!(bitwise_not_positive_integer, r#"~9"#, Object::Int(-10));

test_case!(bitwise_not_negative_integer, r#"~-5"#, Object::Int(4));

// =======================================================
// Operator Chaining & Combinations
// =======================================================

test_case!(multiple_unary_minus, r#"- - - 5"#, Object::Int(-5));

test_case!(unary_plus_and_minus_mix, r#"+-+-5"#, Object::Int(5));

test_case!(bitwise_not_chained, r#"~~5"#, Object::Int(5));

test_case!(
    minus_and_bitwise_not_combination,
    r#"-~5"#, // ~5 is -6, -(-6) is 6
    Object::Int(6)
);

test_case!(
    bitwise_not_and_minus_combination,
    r#"~-5"#, // -5 is -5, ~-5 is 4
    Object::Int(4)
);

// =======================================================
// Precedence Verification
// =======================================================

test_case!(
    unary_precedence_higher_than_multiplication,
    r#"-5 * 2"#, // Should evaluate as (-5) * 2 = -10, not -(5 * 2)
    Object::Int(-10)
);

test_case!(
    unary_precedence_higher_than_addition,
    r#"10 + -3"#, // Should evaluate as 10 + (-3) = 7
    Object::Int(7)
);

test_case!(
    bitwise_not_precedence_higher_than_binary,
    r#"~5 + 2"#, // ~5 is -6, -6 + 2 = -4
    Object::Int(-4)
);

test_case!(
    complex_unary_and_binary_precedence,
    r#"-~2 * 3 + +4"#,
    // Step 1: ~2 becomes -3
    // Step 2: -(-3) becomes 3, +4 becomes 4
    // Step 3: 3 * 3 becomes 9
    // Step 4: 9 + 4 = 13
    Object::Int(13)
);

// =======================================================
// Addition (+) Tests
// =======================================================

test_case!(binary_add_integers, r#"10 + 20"#, Object::Int(30));

test_case!(binary_add_floats, r#"1.5 + 2.25"#, Object::Float(3.75));

test_case!(
    binary_add_mixed_coercion,
    r#"5 + 2.5"#, // Int + Float -> Float
    Object::Float(7.5)
);

// =======================================================
// Subtraction (-) Tests
// =======================================================

test_case!(binary_sub_integers, r#"50 - 15"#, Object::Int(35));

test_case!(binary_sub_floats, r#"10.5 - 3.25"#, Object::Float(7.25));

test_case!(
    binary_sub_mixed_coercion,
    r#"4.0 - 1"#, // Float - Int -> Float
    Object::Float(3.0)
);

// =======================================================
// Multiplication (*) Tests
// =======================================================

test_case!(binary_mul_integers, r#"6 * 7"#, Object::Int(42));

test_case!(binary_mul_floats, r#"2.5 * 2.0"#, Object::Float(5.0));

test_case!(binary_mul_mixed_coercion, r#"3 * 1.5"#, Object::Float(4.5));

// =======================================================
// Division (/) Tests
// =======================================================

test_case!(
    binary_div_integers,
    r#"10 / 2"#, // Python / operator always results in a float!
    Object::Float(5.0)
);

test_case!(binary_div_floats, r#"7.5 / 2.5"#, Object::Float(3.0));

test_case!(
    binary_div_producing_fraction,
    r#"5 / 2"#,
    Object::Float(2.5)
);

// =======================================================
// Associativity & Left-to-Right Evaluation
// =======================================================

test_case!(
    subtraction_left_associative,
    r#"10 - 4 - 2"#, // Should be (10 - 4) - 2 = 4, NOT 10 - (4 - 2) = 8
    Object::Int(4)
);

test_case!(
    division_left_associative,
    r#"12 / 3 / 2"#, // Should be (12 / 3) / 2 = 2.0
    Object::Float(2.0)
);

// =======================================================
// Precedence (PEMDAS Verification)
// =======================================================

test_case!(
    precedence_mul_before_add,
    r#"2 + 3 * 4"#, // 2 + 12 = 14
    Object::Int(14)
);

test_case!(
    precedence_div_before_sub,
    r#"10 - 6 / 2"#, // 10 - 3.0 = 7.0
    Object::Float(7.0)
);

test_case!(
    precedence_mixed_term_evaluation,
    r#"5 * 4 + 3 * 2"#, // 20 + 6 = 26
    Object::Int(26)
);

test_case!(
    grouping_overrides_precedence,
    r#"(2 + 3) * 4"#, // 5 * 4 = 20
    Object::Int(20)
);

test_case!(
    complex_math_pipeline,
    r#"10 + 20 / (2 * 2) - 1.5"#,
    // Step 1: (2 * 2) -> 4
    // Step 2: 20 / 4 -> 5.0
    // Step 3: 10 + 5.0 -> 15.0
    // Step 4: 15.0 - 1.5 -> 13.5
    Object::Float(13.5)
);

// =======================================================
// Basic Grouping Overrides
// =======================================================

test_case!(
    grouping_forces_add_before_mul,
    r#"(2 + 3) * 4"#, // Standard: 2 + 12 = 14 | Grouped: 5 * 4 = 20
    Object::Int(20)
);

test_case!(
    grouping_forces_sub_before_div,
    r#"(10 - 4) / 2"#, // Standard: 10 - 2.0 = 8.0 | Grouped: 6 / 2 = 3.0
    Object::Float(3.0)
);

test_case!(
    grouping_forces_add_before_sub,
    r#"20 - (5 + 5)"#, // Standard: 20 - 5 + 5 = 20 | Grouped: 20 - 10 = 10
    Object::Int(10)
);

// =======================================================
// Deeply Nested Parentheses
// =======================================================

test_case!(
    nested_parentheses_two_levels,
    r#"2 * ((3 + 4) * 5)"#, // 2 * (7 * 5) -> 2 * 35 = 70
    Object::Int(70)
);

test_case!(
    nested_parentheses_three_levels,
    r#"100 / (((1 + 1) + 1) + 1)"#, // 100 / ((2 + 1) + 1) -> 100 / (3 + 1) -> 100 / 4 = 25.0
    Object::Float(25.0)
);

test_case!(
    adjacent_independent_groups,
    r#"(1 + 2) * (3 + 4)"#, // 3 * 7 = 21
    Object::Int(21)
);

// =======================================================
// Grouping with Unary Operators
// =======================================================

test_case!(
    unary_minus_applied_to_group,
    r#"-(5 + 5)"#, // -(10) = -10
    Object::Int(-10)
);

test_case!(
    bitwise_not_applied_to_group,
    r#"~(2 * 4)"#, // ~(8) = -9
    Object::Int(-9)
);

test_case!(
    unary_minus_inside_and_outside_groups,
    r#"-(-5 * -(2 + 3))"#,
    // Step 1: -(2 + 3) -> -5
    // Step 2: -5 * -5 -> 25
    // Step 3: -(25) -> -25
    Object::Int(-25)
);

// =======================================================
// Complex Precedence Inversion
// =======================================================

test_case!(
    complex_inverted_pemdas,
    r#"((10 + 5) * (6 - 2)) / (2 + 1)"#,
    // Step 1: (10 + 5) -> 15
    // Step 2: (6 - 2)  -> 4
    // Step 3: (2 + 1)  -> 3
    // Step 4: 15 * 4   -> 60
    // Step 5: 60 / 3   -> 20.0
    Object::Float(20.0)
);

test_case!(
    mixed_coercion_inside_parentheses,
    r#"(5 / 2) + (1.5 * (4 - 2))"#,
    // Step 1: (5 / 2)   -> 2.5
    // Step 2: (4 - 2)   -> 2
    // Step 3: 1.5 * 2   -> 3.0
    // Step 4: 2.5 + 3.0 -> 5.5
    Object::Float(5.5)
);

// =======================================================
// Basic Assignment and Retrieval
// =======================================================

test_case!(
    assign_and_read_int,
    r#"x = 42
x"#,
    Object::Int(42)
);

test_case!(
    assign_and_read_float,
    r#"pi = 3.14159
pi"#,
    Object::Float(3.14159)
);

test_case!(
    assign_and_read_string,
    r#"message = "hello interpreter"
message"#,
    Object::String("hello interpreter".to_string())
);

test_case!(
    assign_and_read_bool,
    r#"is_valid = True
is_valid"#,
    Object::Bool(true)
);

test_case!(
    assign_and_read_none,
    r#"nothing = None
nothing"#,
    Object::None
);

// =======================================================
// Re-assignment & Environment Mutation
// =======================================================

test_case!(
    variable_reassignment,
    r#"score = 10
score = 25
score"#,
    Object::Int(25)
);

test_case!(
    variable_reassignment_different_type,
    r#"data = 100
data = "now a string"
data"#,
    Object::String("now a string".to_string())
);

test_case!(
    assign_from_another_variable,
    r#"original = 50
copy = original
copy"#,
    Object::Int(50)
);

// =======================================================
// Complex Expression Assignments
// =======================================================

test_case!(
    assign_math_expression,
    r#"result = (10 + 5) * 2
result"#,
    Object::Int(30)
);

test_case!(
    assign_logical_expression,
    r#"status = "active" or "fallback"
status"#,
    Object::String("active".to_string())
);

test_case!(
    self_referential_assignment,
    r#"counter = 1
counter = counter + 5
counter"#,
    Object::Int(6)
);

test_case!(
    complex_multi_variable_pipeline,
    r#"a = 5
b = 10
c = a * b + -5
c"#,
    Object::Int(45)
);

// =======================================================
// Basic If (No Elif, No Else)
// =======================================================

test_case!(
    if_condition_true,
    r#"x = 0
if True:
    x = 10
x"#,
    Object::Int(10)
);

test_case!(
    if_condition_false,
    r#"x = 0
if False:
    x = 10
x"#,
    Object::Int(0)
);

// =======================================================
// If-Else Permutations
// =======================================================

test_case!(
    if_else_takes_if_branch,
    r#"result = 0
if 5 > 2:
    result = 100
else:
    result = 200
result"#,
    Object::Int(100)
);

test_case!(
    if_else_takes_else_branch,
    r#"result = 0
if 5 < 2:
    result = 100
else:
    result = 200
result"#,
    Object::Int(200)
);

// =======================================================
// If-Elif Permutations (No Else)
// =======================================================

test_case!(
    if_elif_takes_elif,
    r#"status = 0
if False:
    status = 1
elif True:
    status = 2
status"#,
    Object::Int(2)
);

test_case!(
    if_multiple_elif_takes_second,
    r#"selection = "none"
if False:
    selection = "first"
elif 10 == 20:
    selection = "second"
elif 5 == 5:
    selection = "third"
selection"#,
    Object::String("third".to_string())
);

test_case!(
    if_elif_all_false,
    r#"marker = 42
if False:
    marker = 1
elif False:
    marker = 2
marker"#,
    Object::Int(42)
);

// =======================================================
// Full If-Elif-Else Permutations
// =======================================================

test_case!(
    if_elif_else_takes_elif,
    r#"val = 0
if False:
    val = 1
elif True:
    val = 2
else:
    val = 3
val"#,
    Object::Int(2)
);

test_case!(
    if_elif_else_takes_else,
    r#"val = 0
if False:
    val = 1
elif False:
    val = 2
else:
    val = 3
val"#,
    Object::Int(3)
);

// =======================================================
// Scope Leaking & Variable Scope Verification
// =======================================================

test_case!(
    variable_created_inside_if_leaks_out,
    r#"if True:
    new_var = 99
new_var"#, // new_var should be bound in the parent environment
    Object::Int(99)
);

test_case!(
    variable_created_inside_else_leaks_out,
    r#"if False:
    a = 1
else:
    leaked_from_else = "success"
leaked_from_else"#,
    Object::String("success".to_string())
);

test_case!(
    variable_created_inside_elif_leaks_out,
    r#"if False:
    a = 1
elif True:
    leaked_from_elif = 3.14
else:
    b = 2
leaked_from_elif"#,
    Object::Float(3.14)
);

// =======================================================
// Nested Conditional Blocks
// =======================================================

test_case!(
    nested_if_statements,
    r#"out = 0
if True:
    if True:
        out = 77
out"#,
    Object::Int(77)
);

test_case!(
    complex_nested_conditional_pipeline,
    r#"res = "start"
x = 10
if x < 5:
    res = "low"
else:
    if x == 10:
        res = "exact match"
    else:
        res = "high"
res"#,
    Object::String("exact match".to_string())
);

// =======================================================
// Basic While Loops & Accumulators
// =======================================================

test_case!(
    while_loop_basic_increment,
    r#"counter = 0
while counter < 5:
    counter = counter + 1
counter"#,
    Object::Int(5)
);

test_case!(
    while_loop_accumulator,
    r#"total = 0
i = 1
while i <= 4:
    total = total + i
    i = i + 1
total"#, // 1 + 2 + 3 + 4 = 10
    Object::Int(10)
);

// =======================================================
// Condition Boundary Checks
// =======================================================

test_case!(
    while_loop_initially_false,
    r#"flag = False
runs = 0
while flag:
    runs = runs + 1
runs"#, // Condition is false from the start; block should never execute
    Object::Int(0)
);

test_case!(
    while_loop_truthy_coercion,
    r#"countdown = 3
result = ""
while countdown:
    result = "running"
    countdown = countdown - 1
result"#, // Loops while countdown is non-zero (truthy)
    Object::String("running".to_string())
);

// =======================================================
// Variable Scope Leaking
// =======================================================

test_case!(
    variable_created_inside_while_leaks,
    r#"i = 0
while i < 1:
    leaked_val = 555
    i = i + 1
leaked_val"#, // leaked_val must be accessible in the parent scope after loop exit
    Object::Int(555)
);

// =======================================================
// Nested Loops
// =======================================================

test_case!(
    nested_while_loops,
    r#"out = 0
i = 0
while i < 3:
    j = 0
    while j < 2:
        out = out + 1
        j = j + 1
    i = i + 1
out"#, // Outer loops 3 times, inner loops 2 times per outer loop = 6 iterations total
    Object::Int(6)
);

test_case!(
    complex_conditional_loop_pipeline,
    r#"n = 10
evens_count = 0
while n > 0:
    # If statements nested inside a while loop
    if n == 8:
        evens_count = evens_count + 1
    elif n == 4:
        evens_count = evens_count + 1
    
    n = n - 1
evens_count"#,
    Object::Int(2)
);

// =======================================================
// Identity
// =======================================================

test_case!(
    identity_boolean_true,
    r#"id(True) == id(True)"#,
    // True should map to the same deterministic ID every time
    Object::Bool(true)
);

test_case!(
    identity_boolean_false,
    r#"id(False) == id(False)"#,
    // False should map to the same deterministic ID every time
    Object::Bool(true)
);

test_case!(
    identity_none,
    r#"id(None) == id(None)"#,
    // None is a global singleton
    Object::Bool(true)
);

test_case!(
    is_operator_true,
    r#"x = True
y = True
x is y"#,
    // Identical identities mean 'is' must return True
    Object::Bool(true)
);

test_case!(
    is_not_operator_false,
    r#"x = None
y = None
x is not y"#,
    // Since x and y point to the exact same None, 'is not' must be False
    Object::Bool(false)
);

test_case!(
    small_int_cache_interning,
    r#"x = 42
y = 42
x is y"#,
    // 42 falls into Python's -5 to 256 cache range, so they must be the same instance
    Object::Bool(true)
);

test_case!(
    large_int_no_interning,
    r#"x = 500
y = 500
x is y"#,
    // 500 is outside the cache limit, so they must be distinct instances
    Object::Bool(false)
);

test_case!(
    large_int_equality_vs_identity,
    r#"x = 500
y = 500
x == y"#,
    // Distinct instances on the heap, but their actual numeric values are equal
    Object::Bool(true)
);

// test_case!(
//     string_interning_pool,
//     r#"x = "hello"
// y = "hello"
// x is y"#,
//     // Identical short strings are automatically pooled/interned
//     Object::Bool(true)
// );

test_case!(
    comparison_operator_chaining,
    r#"x = 10
y = 20
# Rewrites to: (x < y) and (y == 20) and (20 < 30)
x < y == 20 < 30"#,
    // All conditions in the chain are true
    Object::Bool(true)
);

test_case!(
    comparison_chaining_short_circuit,
    r#"x = 50
y = 20
# Rewrites to: (x < y) and (y == 20)
# Fails instantly at (50 < 20), meaning y is never validated or touched again
x < y == 20"#,
    Object::Bool(false)
);

// =======================================================
// Type
// =======================================================

test_case!(
    type_int,
    r#"type(42) == type(-5)"#,
    // Both are integers, so their types should be identical
    Object::Bool(true)
);

test_case!(
    type_bool_true,
    r#"type(True) == type(False)"#,
    // True and False share the same boolean type class
    Object::Bool(true)
);

test_case!(
    type_none,
    r#"x = None
type(x)"#,
    // Should return your representation of <class 'NoneType'>
    Object::String("<class 'NoneType'>".to_string())
);

test_case!(
    type_string,
    r#"type("hello")"#,
    // Verification for the string type class descriptor
    Object::String("<class 'str'>".to_string())
);

test_case!(
    type_mismatch,
    r#"type(42) == type("42")"#,
    // An integer type is distinctly different from a string type
    Object::Bool(false)
);

// test_case!(
//     type_identity_is_operator,
//     r#"type(100) is type(0)"#,
//     // In Python, type descriptors themselves are singletons,
//     // so checking them with 'is' must return True
//     Object::Bool(true)
// );

test_case!(
    type_expression_evaluation,
    r#"x = 10
y = 20
type(x + y)"#,
    // The expression inside must be fully evaluated to an Int before checking the type
    Object::String("<class 'int'>".to_string())
);

test_case!(
    type_native_function,
    r#"type(print)"#,
    // Verification for how you want to represent built-in functions
    Object::String("<class 'builtin_function_or_method'>".to_string())
);

// =======================================================
// Function Definitions & Basic Calls
// =======================================================

test_case!(
    function_no_args_no_return,
    r#"
def say_hi():
    print("hi")

say_hi() == None"#,
    // Functions without an explicit return statement should implicitly evaluate to None
    Object::Bool(true)
);

test_case!(
    function_single_arg_return,
    r#"
def identity(x):
    return x
    
identity(42) == 42"#,
    Object::Bool(true)
);

test_case!(
    function_multiple_args,
    r#"
def add(a, b, c):
    return a + b + c

add(10, 20, 30)"#,
    Object::Int(60)
);

test_case!(
    function_shadowing_parameters,
    r#"
x = 10
def shadow(x):
    return x
    
shadow(5) == 5 and x == 10"#,
    // Parameters should locally shadow outer variables without overwriting them
    Object::Bool(true)
);

// =======================================================
// Return Statement Control Flow
// =======================================================

test_case!(
    return_early_blocks_execution,
    r#"
def early_exit():
    return "first"
    return "second"
    
early_exit() == "first""#,
    // Execution should drop completely out of the function on the first return hit
    Object::Bool(true)
);

test_case!(
    return_nested_in_if,
    r#"
def max(a, b):
    if a > b:
        return a
    return b
    
max(10, 5) == 10 and max(3, 7) == 7"#,
    // Return nested inside a conditional block should correctly pop the stack
    Object::Bool(true)
);

test_case!(
    return_nested_deep_in_while,
    r#"
def find_threshold():
    i = 1
    while i < 100:
        if i * i > 50:
            return i
        i = i + 1
    return None
    
find_threshold()"#,
    // Return nested inside an if statement, inside a while loop, must cascade un-winding safely
    Object::Int(8)
);

test_case!(
    return_from_deep_block_nesting,
    r#"
def deeply_nested():
    if True:
        while True:
            if True:
                return "escaped!"
    return "failed"
    
deeply_nested()"#,
    // Verifies the ? operator completely bypasses loop mechanics once Return is activated
    Object::String("escaped!".to_string())
);

// =======================================================
// Scope and Recursion
// =======================================================

test_case!(
    function_recursive_fibonacci,
    r#"
def fib(n):
    if n <= 1:
        return n
    return fib(n - 2) + fib(n - 1)
    
fib(15)"#,
    Object::Int(610)
);

test_case!(
    function_recursive_factorial,
    r#"
def factorial(n):
    if n <= 1:
        return 1
    return n * factorial(n - 1)
    
factorial(5)"#,
    Object::Int(120)
);

test_case!(
    function_nested_function_def,
    r#"
def make_printer():
    def printer():
        return "hi!"
    
    return printer

printer = make_printer()
printer()"#,
    Object::String("hi!".to_string())
);

// test_case!(
//     function_closure,
//     r#"
// def make_counter():
//     i = 0
//     def counter():
//         i = i + 1
//         print(i)
    
//     return counter

// counter = make_counter()
// counter()
// counter()"#,
//     Object::String("hi!".to_string())
// );

// =======================================================
// Augmented Assignment Operators
// =======================================================

test_case!(
    augmented_add_int,
    r#"
x = 10
x += 5
x"#, 
    Object::Int(15)
);

test_case!(
    augmented_subtract_int,
    r#"
x = 20
x -= 7
x"#, 
    Object::Int(13)
);

test_case!(
    augmented_multiply_int,
    r#"
x = 6
x *= 4
x"#, 
    Object::Int(24)
);

test_case!(
    augmented_divide_int,
    r#"
x = 20
x /= 4
x"#, 
    // In Python, standard division always returns a float
    Object::Float(5.0)
);

test_case!(
    augmented_add_string_concat,
    r#"
text = "hello"
text += " world"
text"#, 
    Object::String("hello world".to_string())
);

test_case!(
    augmented_multiply_string_repeat,
    r#"
pattern = "Ab"
pattern *= 3
pattern"#, 
    Object::String("AbAbAb".to_string())
);

test_case!(
    augmented_assignment_expression_rvalue,
    r#"
x = 5
y = 2
x += y * 3 + 1  # Equivalent to: x = x + (2 * 3 + 1) -> 5 + 7
x"#, 
    Object::Int(12)
);

test_case!(
    augmented_assignment_in_while_loop,
    r#"
count = 0
total = 0
while count < 4:
    total += count
    count += 1
total"#, // 0 + 1 + 2 + 3 = 6
    Object::Int(6)
);