// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Tests tokenizing all literals.

use expect_test::expect;
use test_utils::{tokenize_check, tokenize_error_check};

#[test]
fn test_literals() {
    tokenize_check(
        "32 -123 true false 0 +999999",
        expect![[r#"
            Token `Int(32)` at 0..2
            Token `Minus` at 3..4
            Token `Int(123)` at 4..7
            Token `Bool(true)` at 8..12
            Token `Bool(false)` at 13..18
            Token `Int(0)` at 19..20
            Token `Plus` at 21..22
            Token `Int(999999)` at 22..28
        "#]],
    );
    tokenize_check(
        "truee falsee a999999 varname_true_false_1_end", // Close to being literals
        expect![[r#"
            Token `Id("truee")` at 0..5
            Token `Id("falsee")` at 6..12
            Token `Id("a999999")` at 13..20
            Token `Id("varname_true_false_1_end")` at 21..45
        "#]],
    );
}

#[test]
fn test_literals_floats() {
    tokenize_check(
        "0.0 1.0 3.141592653589793 23. .234234 -----.45 -273.15 +2.3 +.5",
        expect![[r#"
            Token `Float(0.0)` at 0..3
            Token `Float(1.0)` at 4..7
            Token `Float(3.141592653589793)` at 8..25
            Token `Float(23.0)` at 26..29
            Token `Float(0.234234)` at 30..37
            Token `Minus` at 38..39
            Token `Minus` at 39..40
            Token `Minus` at 40..41
            Token `Minus` at 41..42
            Token `Minus` at 42..43
            Token `Float(0.45)` at 43..46
            Token `Minus` at 47..48
            Token `Float(273.15)` at 48..54
            Token `Plus` at 55..56
            Token `Float(2.3)` at 56..59
            Token `Plus` at 60..61
            Token `Float(0.5)` at 61..63
        "#]],
    );
}

// Tests that just dot is not a float. TODO: when dot operator is added, can remove this test
#[test]
fn test_syntax_error_dot() {
    tokenize_error_check(
        ".",
        expect![[r#"
            Error: Syntax Error: Invalid or unexpected token
             --> :1:0
              |
            1 | .
              | ^
        "#]],
    );
}
