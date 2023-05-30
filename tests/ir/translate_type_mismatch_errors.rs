// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Tests translator type checking for programs with variable misuse errors.

use expect_test::expect;
use test_utils::translate_error_check;

#[test]
fn test_let_type_reference_mismatch_1() {
    translate_error_check(
        "let a: int = true",
        expect![[r#"
            Error: Mismatched types, expected `int`, but found `bool`
             --> :1:13
              |
            1 | let a: int = true
              |              ^^^^
        "#]],
    );
}

#[test]
fn test_let_type_reference_mismatch_2() {
    translate_error_check(
        "let b: int = 1 + 2 + 3 < 4 * 5",
        expect![[r#"
            Error: Mismatched types, expected `int`, but found `bool`
             --> :1:13
              |
            1 | let b: int = 1 + 2 + 3 < 4 * 5
              |              ^^^^^^^^^^^^^^^^^
        "#]],
    );
}

#[test]
fn test_let_type_reference_mismatch_3() {
    translate_error_check(
        "let b: bool = let a: int = true",
        expect![[r#"
            Error: Mismatched types, expected `int`, but found `bool`
             --> :1:27
              |
            1 | let b: bool = let a: int = true
              |                            ^^^^
        "#]],
    );
}

#[test]
fn test_unary_type_reference_mismatch_1() {
    translate_error_check(
        "let b: bool = !(1 + 2 - 3)",
        expect![[r#"
            Error: Mismatched types. `Not` operator expected `bool`, found `int`
             --> :1:14
              |
            1 | let b: bool = !(1 + 2 - 3)
              |               ^
        "#]],
    );
}

#[test]
fn test_unary_type_reference_mismatch_2() {
    translate_error_check(
        "let b: int = -----false",
        expect![[r#"
            Error: Mismatched types. `Negative` operator expected `int` or `float`, found `bool`
             --> :1:17
              |
            1 | let b: int = -----false
              |                  ^
        "#]],
    );
}

#[test]
fn test_binary_type_reference_mismatch_numerical_1() {
    translate_error_check(
        "let b: int = 1 + 2. - 3 * false",
        expect![[r#"
            Error: Bad operand types for `Times` operator: `int` and `bool`
             --> :1:24
              |
            1 | let b: int = 1 + 2. - 3 * false
              |                         ^
        "#]],
    );
}

#[test]
fn test_binary_type_reference_mismatch_numerical_2() {
    translate_error_check(
        "
        let a: bool = 1 < 2;
        let b: int = 1 + 2 - 3 * a
        ",
        expect![[r#"
            Error: Bad operand types for `Times` operator: `int` and `bool`
             --> :3:31
              |
            3 |         let b: int = 1 + 2 - 3 * a
              |                                ^
        "#]],
    );
}

#[test]
fn test_binary_type_reference_mismatch_numerical_3() {
    translate_error_check(
        "
        let a: bool = 1 < 2;
        let b: int = 1 + 2 - 3 * a
        ",
        expect![[r#"
            Error: Bad operand types for `Times` operator: `int` and `bool`
             --> :3:31
              |
            3 |         let b: int = 1 + 2 - 3 * a
              |                                ^
        "#]],
    );
}

#[test]
fn test_binary_type_reference_mismatch_numerical_4() {
    translate_error_check(
        "
        let a: int = (let c: int = 1 + 2 + 3) + 2
        ",
        expect![[r#"
            Error: Bad operand types for `Plus` operator: `<unit>` and `int`
             --> :2:46
              |
            2 |         let a: int = (let c: int = 1 + 2 + 3) + 2
              |                                               ^
        "#]],
    );
}

#[test]
fn test_binary_type_reference_mismatch_comparison_1() {
    translate_error_check(
        "let b: bool = 1 + 2 - 3 <= false",
        expect![[r#"
            Error: Bad operand types for `LessThanOrEquals` operator: `int` and `bool`
             --> :1:24
              |
            1 | let b: bool = 1 + 2 - 3 <= false
              |                         ^^
        "#]],
    );
}

#[test]
fn test_binary_type_reference_mismatch_comparison_2() {
    translate_error_check(
        "
        let a: bool = 1 < 2;
        let b: int = 2 * 23;
        let c: int = b >= a
        ",
        expect![[r#"
            Error: Bad operand types for `MoreThanOrEquals` operator: `int` and `bool`
             --> :4:23
              |
            4 |         let c: int = b >= a
              |                        ^^
        "#]],
    );
}

#[test]
fn test_binary_type_reference_mismatch_equality_1() {
    translate_error_check(
        "let b: bool = 1 + 2 - 3 == false",
        expect![[r#"
            Error: Mismatched types. `EqualsEquals` cannot be used with `int` and `bool`
             --> :1:24
              |
            1 | let b: bool = 1 + 2 - 3 == false
              |                         ^^
        "#]],
    );
}

#[test]
fn test_binary_type_reference_mismatch_equality_2() {
    translate_error_check(
        "
        let a: bool = 1 < 2;
        let b: int = 2 * 23;
        let c: int = b != a
        ",
        expect![[r#"
            Error: Mismatched types. `NotEquals` cannot be used with `int` and `bool`
             --> :4:23
              |
            4 |         let c: int = b != a
              |                        ^^
        "#]],
    );
}
