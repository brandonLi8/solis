// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Tests tokenizing infix expressions.

use expect_test::expect;
use test_utils::tokenize_check;

#[test]
fn test_infix() {
    tokenize_check(
        "let name: int = 32 - 2 + 3 * 4 / 5 % 1 + 2 * (3 + 1)",
        expect![[r#"
            Token { kind: Let, position: 0..3 }
            Token { kind: Id("name"), position: 4..8 }
            Token { kind: Colon, position: 8..9 }
            Token { kind: Id("int"), position: 10..13 }
            Token { kind: Equals, position: 14..15 }
            Token { kind: Int(32), position: 16..18 }
            Token { kind: Minus, position: 19..20 }
            Token { kind: Int(2), position: 21..22 }
            Token { kind: Plus, position: 23..24 }
            Token { kind: Int(3), position: 25..26 }
            Token { kind: Times, position: 27..28 }
            Token { kind: Int(4), position: 29..30 }
            Token { kind: Divide, position: 31..32 }
            Token { kind: Int(5), position: 33..34 }
            Token { kind: Mod, position: 35..36 }
            Token { kind: Int(1), position: 37..38 }
            Token { kind: Plus, position: 39..40 }
            Token { kind: Int(2), position: 41..42 }
            Token { kind: Times, position: 43..44 }
            Token { kind: OpenParen, position: 45..46 }
            Token { kind: Int(3), position: 46..47 }
            Token { kind: Plus, position: 48..49 }
            Token { kind: Int(1), position: 50..51 }
            Token { kind: CloseParen, position: 51..52 }
        "#]],
    );

    tokenize_check(
        "1 + 58 * 67 % 35 - 45 < 6 == (95 / 42) != 19 >= 42 <= 54 > 58",
        expect![[r#"
            Token { kind: Int(1), position: 0..1 }
            Token { kind: Plus, position: 2..3 }
            Token { kind: Int(58), position: 4..6 }
            Token { kind: Times, position: 7..8 }
            Token { kind: Int(67), position: 9..11 }
            Token { kind: Mod, position: 12..13 }
            Token { kind: Int(35), position: 14..16 }
            Token { kind: Minus, position: 17..18 }
            Token { kind: Int(45), position: 19..21 }
            Token { kind: LessThan, position: 22..23 }
            Token { kind: Int(6), position: 24..25 }
            Token { kind: EqualsEquals, position: 26..28 }
            Token { kind: OpenParen, position: 29..30 }
            Token { kind: Int(95), position: 30..32 }
            Token { kind: Divide, position: 33..34 }
            Token { kind: Int(42), position: 35..37 }
            Token { kind: CloseParen, position: 37..38 }
            Token { kind: NotEquals, position: 39..41 }
            Token { kind: Int(19), position: 42..44 }
            Token { kind: MoreThanOrEquals, position: 45..47 }
            Token { kind: Int(42), position: 48..50 }
            Token { kind: LessThanOrEquals, position: 51..53 }
            Token { kind: Int(54), position: 54..56 }
            Token { kind: MoreThan, position: 57..58 }
            Token { kind: Int(58), position: 59..61 }
        "#]],
    )
}
