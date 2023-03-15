// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Tests parsing programs with various parsing syntax errors (as opposed to tokenizer errors).

use expect_test::expect;
use test_utils::parse_error_check;

#[test]
fn test_parse_terminal_unexpected_token() {
    parse_error_check(
        "
        let a: int = * * 2
        ",
        expect![[r#"
            Error: Syntax Error: unexpected token
             --> :2:21
              |
            2 |         let a: int = * * 2
              |                      ^
        "#]],
    );
}

#[test]
fn test_parse_terminal_unexpected_end_of_file() {
    parse_error_check(
        "
        let a: int = 2 +
        ",
        expect![[r#"
            Error: Syntax Error: unexpected end of file
             --> :2:23
              |
            2 |         let a: int = 2 +
              |                        ^
        "#]],
    );
}

#[test]
fn test_parse_terminal_unexpected_end_of_file_2() {
    parse_error_check(
        "
        let a: int = +
        ",
        expect![[r#"
            Error: Syntax Error: unexpected end of file
             --> :2:21
              |
            2 |         let a: int = +
              |                      ^
        "#]],
    );
}

#[test]
fn test_parse_factor_consume_token() {
    parse_error_check(
        "
        let a: int = 2 + (2 + 1
        ",
        expect![[r#"
            Error: Syntax Error: unexpected end of file
             --> :2:30
              |
            2 |         let a: int = 2 + (2 + 1
              |                               ^
        "#]],
    );
}

#[test]
fn test_parse_factor_consume_token_end_of_file() {
    parse_error_check(
        "
        let a: int = 2 + (2 + 1 \nlet b: int = 2
        ",
        expect![[r#"
            Error: Syntax Error: expected CloseParen
             --> :2:30
              |
            2 |         let a: int = 2 + (2 + 1 
              |                               ^
        "#]],
    );
}

#[test]
fn test_parse_if_unexpected_end_of_file() {
    parse_error_check(
        "
        if a { 2 + 1 + 3
        ",
        expect![[r#"
            Error: Syntax Error: unexpected end of file
             --> :2:23
              |
            2 |         if a { 2 + 1 + 3
              |                        ^
        "#]],
    );
}

#[test]
fn test_parse_if_no_brace() {
    parse_error_check(
        "
        if a
          1 + 2
        }
        ",
        expect![[r#"
            Error: Syntax Error: expected OpenBrace
             --> :2:11
              |
            2 |         if a
              |            ^
        "#]],
    );
}
