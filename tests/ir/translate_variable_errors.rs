// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Tests translator type checking for programs with variable misuse errors.

use expect_test::expect;
use test_utils::translate_error_check;

#[test]
fn test_undeclared_variable_1() {
    translate_error_check(
        "a",
        expect![[r#"
            Error: Undeclared variable `a`
             --> :1:0
              |
            1 | a
              | ^
        "#]],
    );
}

#[test]
fn test_undeclared_variable_deep_1() {
    translate_error_check(
        "1 + 2 - 3 + 4 * 5 % 6 + 7 + a",
        expect![[r#"
            Error: Undeclared variable `a`
             --> :1:28
              |
            1 | 1 + 2 - 3 + 4 * 5 % 6 + 7 + a
              |                             ^
        "#]],
    );
}

#[test]
fn test_undeclared_variable_deep_2() {
    translate_error_check(
        &("1 + 2\n".repeat(100) + "undeclared_variable"),
        expect![[r#"
            Error: Undeclared variable `undeclared_variable`
             --> :101:0
                |
            101 | undeclared_variable
                | ^^^^^^^^^^^^^^^^^^^
        "#]],
    );
}

#[test]
fn test_undeclared_variable_let_1() {
    translate_error_check(
        "let a: int = b",
        expect![[r#"
            Error: Undeclared variable `b`
             --> :1:13
              |
            1 | let a: int = b
              |              ^
        "#]],
    );
}

#[test]
fn test_undeclared_variable_let_nested() {
    translate_error_check(
        "let a: int = let b: int = c",
        expect![[r#"
            Error: Undeclared variable `c`
             --> :1:26
              |
            1 | let a: int = let b: int = c
              |                           ^
        "#]],
    );
}

#[test]
fn test_undeclared_variable_let_nested_cyclic() {
    translate_error_check(
        "let a: int = let b: int = a",
        expect![[r#"
            Error: Undeclared variable `a`
             --> :1:26
              |
            1 | let a: int = let b: int = a
              |                           ^
        "#]],
    );
}
