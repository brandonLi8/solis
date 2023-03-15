// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Tests tokenizing programs with various syntax errors.

use expect_test::expect;
use test_utils::tokenize_error_check;

#[test]
fn test_syntax_error_basic_1() {
    tokenize_error_check(
        "
        let name: [int = 32
        ",
        expect![[r#"
            Error: Syntax Error: Invalid or unexpected token
             --> :2:18
              |
            2 |         let name: [int = 32
              |                   ^
        "#]],
    );
}

#[test]
fn test_syntax_error_basic_2() {
    tokenize_error_check(
        "
        let name: in[t = 32
        ",
        expect![[r#"
            Error: Syntax Error: Invalid or unexpected token
             --> :2:20
              |
            2 |         let name: in[t = 32
              |                     ^
        "#]],
    );
}

#[test]
fn test_syntax_error_whitespace() {
    tokenize_error_check(
        "

           let name:
                  int [= 32

        ",
        expect![[r#"
            Error: Syntax Error: Invalid or unexpected token
             --> :4:22
              |
            4 |                   int [= 32
              |                       ^
        "#]],
    );
}

#[test]
fn test_syntax_error_double_dot() {
    tokenize_error_check(
        "
        2.5.2
        ",
        expect![[r#"
            Error: Syntax Error: Invalid syntax
             --> :2:11
              |
            2 |         2.5.2
              |            ^
        "#]],
    );
}

#[test]
fn test_syntax_error_double_dot_2() {
    tokenize_error_check(
        "
        1..2
        ",
        expect![[r#"
            Error: Syntax Error: Invalid syntax
             --> :2:10
              |
            2 |         1..2
              |           ^
        "#]],
    );
}

#[test]
fn test_syntax_error_double_dot_3() {
    tokenize_error_check(
        "
        .2.
        ",
        expect![[r#"
            Error: Syntax Error: Invalid syntax
             --> :2:10
              |
            2 |         .2.
              |           ^
        "#]],
    );
}
