// Copyright © 2022 Brandon Li. All rights reserved.

//! Tests tokenizing prefix expressions.

use expect_test::expect;
use test_utils::tokenize_check;

#[test]
fn test_prefix() {
    tokenize_check(
        "let name: +2 - -3 - - 4 + !4 !",
        expect![[r#"
            Token { kind: Let, position: 0..3 }
            Token { kind: Id("name"), position: 4..8 }
            Token { kind: Colon, position: 8..9 }
            Token { kind: Plus, position: 10..11 }
            Token { kind: Int(2), position: 11..12 }
            Token { kind: Minus, position: 13..14 }
            Token { kind: Minus, position: 15..16 }
            Token { kind: Int(3), position: 16..17 }
            Token { kind: Minus, position: 18..19 }
            Token { kind: Minus, position: 20..21 }
            Token { kind: Int(4), position: 22..23 }
            Token { kind: Plus, position: 24..25 }
            Token { kind: Not, position: 26..27 }
            Token { kind: Int(4), position: 27..28 }
            Token { kind: Not, position: 29..30 }
        "#]],
    );

    tokenize_check(
        "let a: int = --------------+++++--+- 2",
        expect![[r#"
            Token { kind: Let, position: 0..3 }
            Token { kind: Id("a"), position: 4..5 }
            Token { kind: Colon, position: 5..6 }
            Token { kind: Id("int"), position: 7..10 }
            Token { kind: Equals, position: 11..12 }
            Token { kind: Minus, position: 13..14 }
            Token { kind: Minus, position: 14..15 }
            Token { kind: Minus, position: 15..16 }
            Token { kind: Minus, position: 16..17 }
            Token { kind: Minus, position: 17..18 }
            Token { kind: Minus, position: 18..19 }
            Token { kind: Minus, position: 19..20 }
            Token { kind: Minus, position: 20..21 }
            Token { kind: Minus, position: 21..22 }
            Token { kind: Minus, position: 22..23 }
            Token { kind: Minus, position: 23..24 }
            Token { kind: Minus, position: 24..25 }
            Token { kind: Minus, position: 25..26 }
            Token { kind: Minus, position: 26..27 }
            Token { kind: Plus, position: 27..28 }
            Token { kind: Plus, position: 28..29 }
            Token { kind: Plus, position: 29..30 }
            Token { kind: Plus, position: 30..31 }
            Token { kind: Plus, position: 31..32 }
            Token { kind: Minus, position: 32..33 }
            Token { kind: Minus, position: 33..34 }
            Token { kind: Plus, position: 34..35 }
            Token { kind: Minus, position: 35..36 }
            Token { kind: Int(2), position: 37..38 }
        "#]],
    );
}
