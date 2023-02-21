// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Tests tokenizing all literals.

use expect_test::expect;
use test_utils::tokenize_check;

#[test]
fn test_literals() {
    tokenize_check(
        "32 -123 true false 0 +999999",
        expect![[r#"
            Token { kind: Int(32), position: 0..2 }
            Token { kind: Minus, position: 3..4 }
            Token { kind: Int(123), position: 4..7 }
            Token { kind: Bool(true), position: 8..12 }
            Token { kind: Bool(false), position: 13..18 }
            Token { kind: Int(0), position: 19..20 }
            Token { kind: Plus, position: 21..22 }
            Token { kind: Int(999999), position: 22..28 }
        "#]],
    );
    tokenize_check(
        "truee falsee a999999 varname_true_false_1_end", // Close to being literals
        expect![[r#"
            Token { kind: Id("truee"), position: 0..5 }
            Token { kind: Id("falsee"), position: 6..12 }
            Token { kind: Id("a999999"), position: 13..20 }
            Token { kind: Id("varname_true_false_1_end"), position: 21..45 }
        "#]],
    )
}

#[test]
fn test_literals_floats() {
    tokenize_check(
        "0.0 1.0 3.141592653589793 23. .234234 -----.45 -273.15 +2.3 +.5",
        expect![[r#"
            Token { kind: Float(0.0), position: 0..3 }
            Token { kind: Float(1.0), position: 4..7 }
            Token { kind: Float(3.141592653589793), position: 8..25 }
            Token { kind: Float(23.0), position: 26..29 }
            Token { kind: Float(0.234234), position: 30..37 }
            Token { kind: Minus, position: 38..39 }
            Token { kind: Minus, position: 39..40 }
            Token { kind: Minus, position: 40..41 }
            Token { kind: Minus, position: 41..42 }
            Token { kind: Minus, position: 42..43 }
            Token { kind: Float(0.45), position: 43..46 }
            Token { kind: Minus, position: 47..48 }
            Token { kind: Float(273.15), position: 48..54 }
            Token { kind: Plus, position: 55..56 }
            Token { kind: Float(2.3), position: 56..59 }
            Token { kind: Plus, position: 60..61 }
            Token { kind: Float(0.5), position: 61..63 }
        "#]],
    );
}

// Tests that just dot is not a float. TODO: when dot operator is added, can remove this test
#[test]
#[should_panic(expected = "Syntax Error: Invalid or unexpected token at 0..1")]
fn test_syntax_error_dot() {
    tokenize_check(".", expect![]);
}
