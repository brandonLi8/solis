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
            Token `Fun` at 9..12
            Token `Id("fib")` at 13..16
            Token `OpenParen` at 16..17
            Token `Id("n")` at 17..18
            Token `Colon` at 18..19
            Token `Id("int")` at 20..23
            Token `CloseParen` at 23..24
            Token `Colon` at 25..26
            Token `Id("int")` at 27..30
            Token `OpenBrace` at 31..32
            Token `If` at 43..45
            Token `Id("n")` at 46..47
            Token `LessThanOrEquals` at 48..50
            Token `Int(1)` at 51..52
            Token `OpenBrace` at 53..54
            Token `Int(1)` at 67..68
            Token `CloseBrace` at 79..80
            Token `Else` at 91..95
            Token `OpenBrace` at 96..97
            Token `Id("fib")` at 110..113
            Token `OpenParen` at 113..114
            Token `Id("n")` at 114..115
            Token `Minus` at 116..117
            Token `Int(1)` at 118..119
            Token `CloseParen` at 119..120
            Token `Plus` at 121..122
            Token `Id("fib")` at 123..126
            Token `OpenParen` at 126..127
            Token `Id("n")` at 127..128
            Token `Minus` at 129..130
            Token `Int(2)` at 131..132
            Token `CloseParen` at 132..133
            Token `CloseBrace` at 144..145
            Token `CloseBrace` at 154..155
            Token `Let` at 165..168
            Token `Id("a")` at 169..170
            Token `Colon` at 170..171
            Token `Id("int")` at 172..175
            Token `Equals` at 176..177
            Token `Int(2)` at 178..179
            Token `Minus` at 180..181
            Token `Id("fib")` at 182..185
            Token `OpenParen` at 185..186
            Token `Int(5)` at 186..187
            Token `CloseParen` at 187..188
            Token `Times` at 189..190
            Token `Int(3)` at 191..192
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
            Token `Fun` at 9..12
            Token `Id("a")` at 13..14
            Token `OpenParen` at 14..15
            Token `Id("b")` at 15..16
            Token `Colon` at 16..17
            Token `Id("int")` at 18..21
            Token `Comma` at 21..22
            Token `Id("c")` at 23..24
            Token `Colon` at 24..25
            Token `Id("int")` at 26..29
            Token `Comma` at 29..30
            Token `Id("d")` at 30..31
            Token `Colon` at 31..32
            Token `Id("int")` at 32..35
            Token `CloseParen` at 35..36
            Token `Colon` at 37..38
            Token `Id("int")` at 39..42
            Token `OpenBrace` at 43..44
            Token `Id("a")` at 55..56
            Token `Plus` at 57..58
            Token `Id("b")` at 59..60
            Token `Plus` at 61..62
            Token `Id("c")` at 63..64
            Token `CloseBrace` at 73..74
            Token `Id("a")` at 84..85
            Token `OpenParen` at 85..86
            Token `Int(1)` at 86..87
            Token `Comma` at 87..88
            Token `Int(2)` at 88..89
            Token `Comma` at 89..90
            Token `Int(3)` at 93..94
            Token `CloseParen` at 94..95
        "#]],
    );
}
