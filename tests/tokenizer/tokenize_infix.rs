// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Tests tokenizing infix expressions.

use expect_test::expect;
use test_utils::tokenize_check;

#[test]
fn test_infix() {
    tokenize_check(
        "let name: float = 32.4 - 2 + 3 * 4 / 5 % 1 + 2 * (3 + 1)",
        expect![[r#"
            Token `Let` at 0..3
            Token `Id("name")` at 4..8
            Token `Colon` at 8..9
            Token `Id("float")` at 10..15
            Token `Equals` at 16..17
            Token `Float(32.4)` at 18..22
            Token `Minus` at 23..24
            Token `Int(2)` at 25..26
            Token `Plus` at 27..28
            Token `Int(3)` at 29..30
            Token `Times` at 31..32
            Token `Int(4)` at 33..34
            Token `Divide` at 35..36
            Token `Int(5)` at 37..38
            Token `Mod` at 39..40
            Token `Int(1)` at 41..42
            Token `Plus` at 43..44
            Token `Int(2)` at 45..46
            Token `Times` at 47..48
            Token `OpenParen` at 49..50
            Token `Int(3)` at 50..51
            Token `Plus` at 52..53
            Token `Int(1)` at 54..55
            Token `CloseParen` at 55..56
        "#]],
    );

    tokenize_check(
        "1 + 58 * 67 % 35 - 45 < 6 == (95 / 42) != 19 >= 42 <= 54 > 58",
        expect![[r#"
            Token `Int(1)` at 0..1
            Token `Plus` at 2..3
            Token `Int(58)` at 4..6
            Token `Times` at 7..8
            Token `Int(67)` at 9..11
            Token `Mod` at 12..13
            Token `Int(35)` at 14..16
            Token `Minus` at 17..18
            Token `Int(45)` at 19..21
            Token `LessThan` at 22..23
            Token `Int(6)` at 24..25
            Token `EqualsEquals` at 26..28
            Token `OpenParen` at 29..30
            Token `Int(95)` at 30..32
            Token `Divide` at 33..34
            Token `Int(42)` at 35..37
            Token `CloseParen` at 37..38
            Token `NotEquals` at 39..41
            Token `Int(19)` at 42..44
            Token `MoreThanOrEquals` at 45..47
            Token `Int(42)` at 48..50
            Token `LessThanOrEquals` at 51..53
            Token `Int(54)` at 54..56
            Token `MoreThan` at 57..58
            Token `Int(58)` at 59..61
        "#]],
    );
}
