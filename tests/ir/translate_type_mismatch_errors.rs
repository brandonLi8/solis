// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Tests translator type checking for programs with variable misuse errors.

use expect_test::expect;
use test_utils::translate_check;

#[test]
#[should_panic(expected = "Mismatched types, expected `bool`, but found `int` at 7..10")]
fn test_let_type_reference_mismatch_1() {
    translate_check("let a: int = true", expect![[]])
}

#[test]
#[should_panic(expected = "Mismatched types, expected `bool`, but found `int` at 7..10")]
fn test_let_type_reference_mismatch_2() {
    translate_check("let b: int = 1 + 2 + 3 < 4 * 5", expect![[]])
}

#[test]
#[should_panic(expected = "Mismatched types, expected `bool`, but found `int` at 21..24")]
fn test_let_type_reference_mismatch_3() {
    translate_check("let b: bool = let a: int = true", expect![[]])
}

#[test]
#[should_panic(expected = "Mismatched types. `Not` operator expected `bool`, found `int` at 14..15")]
fn test_unary_type_reference_mismatch_1() {
    translate_check("let b: bool = !(1 + 2 - 3)", expect![[]])
}

#[test]
#[should_panic(expected = "Mismatched types. `Negative` operator expected `int` or `float`, found `bool` at 17..18")]
fn test_unary_type_reference_mismatch_2() {
    translate_check("let b: int = -----false", expect![[]])
}

#[test]
#[should_panic(expected = "Bad operand types for `Times` operator: `int` and `bool` at 24..25")]
fn test_binary_type_reference_mismatch_numerical_1() {
    translate_check("let b: int = 1 + 2. - 3 * false", expect![[]])
}

#[test]
#[should_panic(expected = "Bad operand types for `Times` operator: `int` and `bool` at 61..62")]
fn test_binary_type_reference_mismatch_numerical_2() {
    translate_check(
        "
        let a: bool = 1 < 2;
        let b: int = 1 + 2 - 3 * a
        ",
        expect![[]],
    )
}

#[test]
#[should_panic(expected = "Bad operand types for `Times` operator: `int` and `bool` at 61..62")]
fn test_binary_type_reference_mismatch_numerical_3() {
    translate_check(
        "
        let a: bool = 1 < 2;
        let b: int = 1 + 2 - 3 * a
        ",
        expect![[]],
    )
}

#[test]
#[should_panic(expected = "Bad operand types for `LessThanOrEquals` operator: `int` and `bool` at 24..26")]
fn test_binary_type_reference_mismatch_comparison_1() {
    translate_check("let b: bool = 1 + 2 - 3 <= false", expect![[]])
}

#[test]
#[should_panic(expected = "Bad operand types for `MoreThanOrEquals` operator: `int` and `bool` at 82..84")]
fn test_binary_type_reference_mismatch_comparison_2() {
    translate_check(
        "
        let a: bool = 1 < 2;
        let b: int = 2 * 23;
        let c: int = b >= a
        ",
        expect![[]],
    )
}

#[test]
#[should_panic(expected = "Mismatched types. `EqualsEquals` cannot be used with `int` and `bool` at 24..26")]
fn test_binary_type_reference_mismatch_equality_1() {
    translate_check("let b: bool = 1 + 2 - 3 == false", expect![[]])
}

#[test]
#[should_panic(expected = "Mismatched types. `NotEquals` cannot be used with `int` and `bool` at 82..84")]
fn test_binary_type_reference_mismatch_equality_2() {
    translate_check(
        "
        let a: bool = 1 < 2;
        let b: int = 2 * 23;
        let c: int = b != a
        ",
        expect![[]],
    )
}
