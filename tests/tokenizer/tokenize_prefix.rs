// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Tests tokenizing prefix expressions.

use expect_test::expect;
use test_utils::tokenize_check;

#[test]
fn test_prefix() {
    tokenize_check(
        "let name: +2 - -3 - - 4.+ !4 !",
        expect![[r#"
            Token `Let` at 0..3
            Token `Id("name")` at 4..8
            Token `Colon` at 8..9
            Token `Plus` at 10..11
            Token `Int(2)` at 11..12
            Token `Minus` at 13..14
            Token `Minus` at 15..16
            Token `Int(3)` at 16..17
            Token `Minus` at 18..19
            Token `Minus` at 20..21
            Token `Float(4.0)` at 22..24
            Token `Plus` at 24..25
            Token `Not` at 26..27
            Token `Int(4)` at 27..28
            Token `Not` at 29..30
        "#]],
    );

    tokenize_check(
        "let a: int = --------------+++++--+- 2.",
        expect![[r#"
            Token `Let` at 0..3
            Token `Id("a")` at 4..5
            Token `Colon` at 5..6
            Token `Id("int")` at 7..10
            Token `Equals` at 11..12
            Token `Minus` at 13..14
            Token `Minus` at 14..15
            Token `Minus` at 15..16
            Token `Minus` at 16..17
            Token `Minus` at 17..18
            Token `Minus` at 18..19
            Token `Minus` at 19..20
            Token `Minus` at 20..21
            Token `Minus` at 21..22
            Token `Minus` at 22..23
            Token `Minus` at 23..24
            Token `Minus` at 24..25
            Token `Minus` at 25..26
            Token `Minus` at 26..27
            Token `Plus` at 27..28
            Token `Plus` at 28..29
            Token `Plus` at 29..30
            Token `Plus` at 30..31
            Token `Plus` at 31..32
            Token `Minus` at 32..33
            Token `Minus` at 33..34
            Token `Plus` at 34..35
            Token `Minus` at 35..36
            Token `Float(2.0)` at 37..39
        "#]],
    );
}
