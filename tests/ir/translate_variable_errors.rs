// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Tests translator type checking for programs with variable misuse errors.

use expect_test::expect;
use test_utils::translate_check;

#[test]
#[should_panic(expected = "Undeclared variable `a` at 0..1")]
fn test_undeclared_variable_1() {
    translate_check("a", expect![[]])
}

#[test]
#[should_panic(expected = "Undeclared variable `a` at 28..29")]
fn test_undeclared_variable_deep_1() {
    translate_check("1 + 2 - 3 + 4 * 5 % 6 + 7 + a", expect![[]])
}

#[test]
#[should_panic(expected = "Undeclared variable `undeclared_variable` at 600..619")]
fn test_undeclared_variable_deep_2() {
    translate_check(&("1 + 2\n".repeat(100) + "undeclared_variable"), expect![[]])
}

#[test]
#[should_panic(expected = "Undeclared variable `b` at 13..14")]
fn test_undeclared_variable_let_1() {
    translate_check("let a: int = b", expect![[]])
}

#[test]
#[should_panic(expected = "Undeclared variable `c` at 26..27")]
fn test_undeclared_variable_let_nested() {
    translate_check("let a: int = let b: int = c", expect![[]])
}

#[test]
#[should_panic(expected = "Undeclared variable `a` at 26..27")]
fn test_undeclared_variable_let_nested_cyclic() {
    translate_check("let a: int = let b: int = a", expect![[]])
}

#[test]
#[should_panic(expected = "Variable `a` is already declared in this scope at 54..58")]
fn test_redeclared_variable_1() {
    translate_check(
        concat!(
            "let a: int = 2\nlet b: bool = false\n",
            "1 + 2\n1 + 2\n",
            "let a: bool = false"
        ),
        expect![[]],
    )
}

#[test]
#[should_panic(expected = "Variable `a` is already declared in this scope at 7..10")]
fn test_redeclared_variable_let_nested() {
    translate_check("let a: int = let b: int = let a: int = 3", expect![[]])
}
