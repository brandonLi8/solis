// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Tests tokenizing programs with various syntax errors.

use expect_test::expect;
use test_utils::tokenize_check;

#[test]
#[should_panic(expected = "Syntax Error: Invalid or unexpected token at 10..11")]
fn test_syntax_error_basic_1() {
    tokenize_check("let name: [int = 32", expect![]);
}

#[test]
#[should_panic(expected = "Syntax Error: Invalid or unexpected token at 12..13")]
fn test_syntax_error_basic_2() {
    tokenize_check("let name: in[t = 32", expect![]);
}

#[test]
#[should_panic(expected = "Syntax Error: Invalid or unexpected token at 23..24")]
fn test_syntax_error_whitespace() {
    tokenize_check("\n  \nlet name:     \nint [= 32 \n\n  ", expect![]);
}

#[test]
#[should_panic(expected = "Syntax Error: Invalid syntax at 3..4")]
fn test_syntax_error_double_dot() {
    tokenize_check("2.5.2", expect![]);
}

#[test]
#[should_panic(expected = "Syntax Error: Invalid syntax at 2..3")]
fn test_syntax_error_double_dot_2() {
    tokenize_check("1..2", expect![]);
}

#[test]
#[should_panic(expected = "Syntax Error: Invalid syntax at 2..3")]
fn test_syntax_error_double_dot_3() {
    tokenize_check(".2.", expect![]);
}
