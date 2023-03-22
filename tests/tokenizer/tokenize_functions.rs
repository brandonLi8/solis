// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Tests tokenizing function declarations.

use expect_test::expect;
use test_utils::tokenize_check;

#[test]
fn test_fib() {
    tokenize_check(
        "
        fun fib(n: int) : int {
          if n <= 1 {
            1
          }
          else {
            fib(n - 1) + fib(n - 2)
          }
        }

        let a: int = 2 - fib(5) * 3
        ",
        expect![[r#"
            Token { kind: Fun, position: 9..12 }
            Token { kind: Id("fib"), position: 13..16 }
            Token { kind: OpenParen, position: 16..17 }
            Token { kind: Id("n"), position: 17..18 }
            Token { kind: Colon, position: 18..19 }
            Token { kind: Id("int"), position: 20..23 }
            Token { kind: CloseParen, position: 23..24 }
            Token { kind: Colon, position: 25..26 }
            Token { kind: Id("int"), position: 27..30 }
            Token { kind: OpenBrace, position: 31..32 }
            Token { kind: If, position: 43..45 }
            Token { kind: Id("n"), position: 46..47 }
            Token { kind: LessThanOrEquals, position: 48..50 }
            Token { kind: Int(1), position: 51..52 }
            Token { kind: OpenBrace, position: 53..54 }
            Token { kind: Int(1), position: 67..68 }
            Token { kind: CloseBrace, position: 79..80 }
            Token { kind: Else, position: 91..95 }
            Token { kind: OpenBrace, position: 96..97 }
            Token { kind: Id("fib"), position: 110..113 }
            Token { kind: OpenParen, position: 113..114 }
            Token { kind: Id("n"), position: 114..115 }
            Token { kind: Minus, position: 116..117 }
            Token { kind: Int(1), position: 118..119 }
            Token { kind: CloseParen, position: 119..120 }
            Token { kind: Plus, position: 121..122 }
            Token { kind: Id("fib"), position: 123..126 }
            Token { kind: OpenParen, position: 126..127 }
            Token { kind: Id("n"), position: 127..128 }
            Token { kind: Minus, position: 129..130 }
            Token { kind: Int(2), position: 131..132 }
            Token { kind: CloseParen, position: 132..133 }
            Token { kind: CloseBrace, position: 144..145 }
            Token { kind: CloseBrace, position: 154..155 }
            Token { kind: Let, position: 165..168 }
            Token { kind: Id("a"), position: 169..170 }
            Token { kind: Colon, position: 170..171 }
            Token { kind: Id("int"), position: 172..175 }
            Token { kind: Equals, position: 176..177 }
            Token { kind: Int(2), position: 178..179 }
            Token { kind: Minus, position: 180..181 }
            Token { kind: Id("fib"), position: 182..185 }
            Token { kind: OpenParen, position: 185..186 }
            Token { kind: Int(5), position: 186..187 }
            Token { kind: CloseParen, position: 187..188 }
            Token { kind: Times, position: 189..190 }
            Token { kind: Int(3), position: 191..192 }
        "#]],
    );
}

#[test]
fn test_function() {
    tokenize_check(
        "
        fun a(b: int, c: int,d:int) : int {
          a + b + c
        }

        a(1,2,   3)
        ",
        expect![[r#"
            Token { kind: Fun, position: 9..12 }
            Token { kind: Id("a"), position: 13..14 }
            Token { kind: OpenParen, position: 14..15 }
            Token { kind: Id("b"), position: 15..16 }
            Token { kind: Colon, position: 16..17 }
            Token { kind: Id("int"), position: 18..21 }
            Token { kind: Comma, position: 21..22 }
            Token { kind: Id("c"), position: 23..24 }
            Token { kind: Colon, position: 24..25 }
            Token { kind: Id("int"), position: 26..29 }
            Token { kind: Comma, position: 29..30 }
            Token { kind: Id("d"), position: 30..31 }
            Token { kind: Colon, position: 31..32 }
            Token { kind: Id("int"), position: 32..35 }
            Token { kind: CloseParen, position: 35..36 }
            Token { kind: Colon, position: 37..38 }
            Token { kind: Id("int"), position: 39..42 }
            Token { kind: OpenBrace, position: 43..44 }
            Token { kind: Id("a"), position: 55..56 }
            Token { kind: Plus, position: 57..58 }
            Token { kind: Id("b"), position: 59..60 }
            Token { kind: Plus, position: 61..62 }
            Token { kind: Id("c"), position: 63..64 }
            Token { kind: CloseBrace, position: 73..74 }
            Token { kind: Id("a"), position: 84..85 }
            Token { kind: OpenParen, position: 85..86 }
            Token { kind: Int(1), position: 86..87 }
            Token { kind: Comma, position: 87..88 }
            Token { kind: Int(2), position: 88..89 }
            Token { kind: Comma, position: 89..90 }
            Token { kind: Int(3), position: 93..94 }
            Token { kind: CloseParen, position: 94..95 }
        "#]],
    );
}
