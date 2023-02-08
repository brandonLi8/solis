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

    tokenize_check(
        "(());()()",
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
        "#]],
    );
}
