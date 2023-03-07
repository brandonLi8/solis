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
            Token { kind: Let, position: 0..3 }
            Token { kind: Id("name"), position: 4..8 }
            Token { kind: Colon, position: 8..9 }
            Token { kind: Id("int"), position: 10..13 }
            Token { kind: Equals, position: 14..15 }
            Token { kind: Int(32), position: 16..18 }
        "#]],
    );

    tokenize_check(
        "let final name: int = -123",
        expect![[r#"
            Token { kind: Let, position: 0..3 }
            Token { kind: Final, position: 4..9 }
            Token { kind: Id("name"), position: 10..14 }
            Token { kind: Colon, position: 14..15 }
            Token { kind: Id("int"), position: 16..19 }
            Token { kind: Equals, position: 20..21 }
            Token { kind: Minus, position: 22..23 }
            Token { kind: Int(123), position: 23..26 }
        "#]],
    );
}

#[test]
fn test_braces() {
    tokenize_check(
        "(());()(){}",
        expect![[r#"
            Token { kind: OpenParen, position: 0..1 }
            Token { kind: OpenParen, position: 1..2 }
            Token { kind: CloseParen, position: 2..3 }
            Token { kind: CloseParen, position: 3..4 }
            Token { kind: Semi, position: 4..5 }
            Token { kind: OpenParen, position: 5..6 }
            Token { kind: CloseParen, position: 6..7 }
            Token { kind: OpenParen, position: 7..8 }
            Token { kind: CloseParen, position: 8..9 }
            Token { kind: OpenBrace, position: 9..10 }
            Token { kind: CloseBrace, position: 10..11 }
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
            Token { kind: If, position: 9..11 }
            Token { kind: Id("i"), position: 12..13 }
            Token { kind: LessThan, position: 14..15 }
            Token { kind: Int(2), position: 16..17 }
            Token { kind: OpenBrace, position: 18..19 }
            Token { kind: Let, position: 30..33 }
            Token { kind: Id("a"), position: 34..35 }
            Token { kind: Colon, position: 35..36 }
            Token { kind: Id("int"), position: 37..40 }
            Token { kind: Equals, position: 41..42 }
            Token { kind: Int(2), position: 43..44 }
            Token { kind: CloseBrace, position: 53..54 }
            Token { kind: Else, position: 63..67 }
            Token { kind: If, position: 68..70 }
            Token { kind: OpenBrace, position: 71..72 }
            Token { kind: Let, position: 83..86 }
            Token { kind: Id("b"), position: 87..88 }
            Token { kind: Colon, position: 88..89 }
            Token { kind: Id("bool"), position: 90..94 }
            Token { kind: Equals, position: 95..96 }
            Token { kind: Bool(false), position: 97..102 }
            Token { kind: Int(1), position: 113..114 }
            Token { kind: Plus, position: 115..116 }
            Token { kind: Int(2), position: 117..118 }
            Token { kind: CloseBrace, position: 127..128 }
            Token { kind: Else, position: 137..141 }
            Token { kind: OpenBrace, position: 142..143 }
            Token { kind: Int(2), position: 154..155 }
            Token { kind: Plus, position: 156..157 }
            Token { kind: Int(3), position: 158..159 }
            Token { kind: CloseBrace, position: 168..169 }
        "#]],
    );
}
