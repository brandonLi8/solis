// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Basic tests for the tokenizer.

use expect_test::expect;
use test_utils::tokenize_check;

#[test]
fn test_empty() {
    tokenize_check("", expect![""]);
}

#[test]
fn test_basic_let() {
    tokenize_check(
        "let name: int = 32",
        expect![[r#"
            Token `Let` at 0..3
            Token `Id("name")` at 4..8
            Token `Colon` at 8..9
            Token `Id("int")` at 10..13
            Token `Equals` at 14..15
            Token `Int(32)` at 16..18
        "#]],
    );

    tokenize_check(
        "let final name: int = -123",
        expect![[r#"
            Token `Let` at 0..3
            Token `Final` at 4..9
            Token `Id("name")` at 10..14
            Token `Colon` at 14..15
            Token `Id("int")` at 16..19
            Token `Equals` at 20..21
            Token `Minus` at 22..23
            Token `Int(123)` at 23..26
        "#]],
    );
}

#[test]
fn test_braces() {
    tokenize_check(
        "(());()(){}",
        expect![[r#"
            Token `OpenParen` at 0..1
            Token `OpenParen` at 1..2
            Token `CloseParen` at 2..3
            Token `CloseParen` at 3..4
            Token `Semi` at 4..5
            Token `OpenParen` at 5..6
            Token `CloseParen` at 6..7
            Token `OpenParen` at 7..8
            Token `CloseParen` at 8..9
            Token `OpenBrace` at 9..10
            Token `CloseBrace` at 10..11
        "#]],
    );
}

#[test]
fn test_if() {
    tokenize_check(
        r"
        if i < 2 {
          let a: int = 2
        }
        else if {
          let b: bool = false
          1 + 2
        }
        else {
          2 + 3
        }
        ",
        expect![[r#"
            Token `If` at 9..11
            Token `Id("i")` at 12..13
            Token `LessThan` at 14..15
            Token `Int(2)` at 16..17
            Token `OpenBrace` at 18..19
            Token `Let` at 30..33
            Token `Id("a")` at 34..35
            Token `Colon` at 35..36
            Token `Id("int")` at 37..40
            Token `Equals` at 41..42
            Token `Int(2)` at 43..44
            Token `CloseBrace` at 53..54
            Token `Else` at 63..67
            Token `If` at 68..70
            Token `OpenBrace` at 71..72
            Token `Let` at 83..86
            Token `Id("b")` at 87..88
            Token `Colon` at 88..89
            Token `Id("bool")` at 90..94
            Token `Equals` at 95..96
            Token `Bool(false)` at 97..102
            Token `Int(1)` at 113..114
            Token `Plus` at 115..116
            Token `Int(2)` at 117..118
            Token `CloseBrace` at 127..128
            Token `Else` at 137..141
            Token `OpenBrace` at 142..143
            Token `Int(2)` at 154..155
            Token `Plus` at 156..157
            Token `Int(3)` at 158..159
            Token `CloseBrace` at 168..169
        "#]],
    );
}
