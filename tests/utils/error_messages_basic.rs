// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Basic tests for the error_messages utility.

use expect_test::expect;
use solis::utils::error_messages::ErrorPosition;
use test_utils::compilation_error_check;

#[test]
fn test_end_of_file() {
    compilation_error_check(
        "some-program",
        ErrorPosition::EndOfFile,
        expect![[r#"
            Error: <error-message>
             --> :1:12
              |
            1 | some-program
              |             ^
        "#]],
    );
}

#[test]
fn test_end_of_file_extra_whitespace() {
    compilation_error_check(
        "some-program



        ",
        ErrorPosition::EndOfFile,
        expect![[r#"
            Error: <error-message>
             --> :1:12
              |
            1 | some-program
              |             ^
        "#]],
    );
}

#[test]
fn test_whitespace_before_basic() {
    compilation_error_check(
        "0  3",
        ErrorPosition::WhitespaceBefore(&(3..4)),
        expect![[r#"
            Error: <error-message>
             --> :1:1
              |
            1 | 0  3
              |  ^
        "#]],
    );
}

#[test]
fn test_whitespace_before_no_whitespace() {
    compilation_error_check(
        "0__2",
        ErrorPosition::WhitespaceBefore(&(3..4)),
        expect![[r#"
            Error: <error-message>
             --> :1:3
              |
            1 | 0__2
              |    ^
        "#]],
    );
}

#[test]
fn test_whitespace_before_is_newline() {
    compilation_error_check(
        "0\n 3",
        ErrorPosition::WhitespaceBefore(&(3..4)),
        expect![[r#"
            Error: <error-message>
             --> :1:1
              |
            1 | 0
              |  ^
        "#]],
    );
}

#[test]
fn test_position_multiline() {
    compilation_error_check(
        "0123\n5\n7",
        ErrorPosition::Span(&(0..8)),
        expect![[r#"
            Error: <error-message>
             --> :1:0
              |
            1 | 0123
              | ^^^^
            2 | 5
              | ^
            3 | 7
              | ^
        "#]],
    );
}

#[test]
fn test_position_multiline_different_line_sizes() {
    compilation_error_check(
        "\n\n\n\n\n\n\n\n0123\n5\n  0 2",
        ErrorPosition::Span(&(8..8 + 13)),
        expect![[r#"
            Error: <error-message>
             --> :9:0
               |
             9 | 0123
               | ^^^^
            10 | 5
               | ^
            11 |   0 2
               |   ^^^
        "#]],
    );
}
