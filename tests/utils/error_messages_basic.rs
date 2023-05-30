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
fn test_span_multiline() {
    compilation_error_check(
        "0123\n5\n7
        ",
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
