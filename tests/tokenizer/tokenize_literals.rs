// Copyright © 2022 Brandon Li. All rights reserved.

//! Tests tokenizing all literals.

use expect_test::expect;
use test_utils::tokenize_check;

#[test]
fn test_literals() {
    tokenize_check(
        "32 -123 true false 0 +999999",
        expect![[r#"
            Token { kind: Int(32), position: 0..2 }
            Token { kind: Minus, position: 3..4 }
            Token { kind: Int(123), position: 4..7 }
            Token { kind: Bool(true), position: 8..12 }
            Token { kind: Bool(false), position: 13..18 }
            Token { kind: Int(0), position: 19..20 }
            Token { kind: Plus, position: 21..22 }
            Token { kind: Int(999999), position: 22..28 }
        "#]],
    );
    tokenize_check(
        "truee falsee a999999 varname_true_false_1_end", // Close to being literals
        expect![[r#"
            Token { kind: Id("truee"), position: 0..5 }
            Token { kind: Id("falsee"), position: 6..12 }
            Token { kind: Id("a999999"), position: 13..20 }
            Token { kind: Id("varname_true_false_1_end"), position: 21..45 }
        "#]],
    )
}
