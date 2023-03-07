// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Tests parsing programs with various parsing syntax errors (as opposed to tokenizer errors).

use expect_test::expect;
use test_utils::parse_check;

#[test]
#[should_panic(expected = "Syntax Error: unexpected token at 13..14")]
fn test_parse_terminal_unexpected_token() {
    parse_check("let a: int = * * 2", expect![[]]);
}

#[test]
#[should_panic(expected = "Syntax Error: unexpected end of file at 15..16")]
fn test_parse_terminal_unexpected_end_of_file() {
    parse_check("let a: int = 2 +", expect![[]]);
}

#[test]
#[should_panic(expected = "Syntax Error: unexpected end of file at 13..14")]
fn test_parse_terminal_unexpected_end_of_file_2() {
    parse_check("let a: int = +", expect![[]]);
}

#[test]
#[should_panic(expected = "Syntax Error: unexpected end of file at 22..23")]
fn test_parse_factor_consume_token() {
    parse_check("let a: int = 2 + (2 + 1", expect![[]]);
}

#[test]
#[should_panic(expected = "Syntax Error: expected CloseParen at 22..23")]
fn test_parse_factor_consume_token_end_of_file() {
    parse_check("let a: int = 2 + (2 + 1 \nlet b: int = 2", expect![[]]);
}

#[test]
#[should_panic(expected = "Syntax Error: unexpected end of file at 15..16")]
fn test_parse_if_unexpected_end_of_file() {
    parse_check("if a { 2 + 1 + 3", expect![[]]);
}

#[test]
#[should_panic(expected = "Syntax Error: expected OpenBrace at 12..13")]
fn test_parse_if_no_brace() {
    parse_check(
        "
        if a
          1 + 2
        }
      ",
        expect![[]],
    );
}
