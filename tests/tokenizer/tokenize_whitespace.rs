// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Tests tokenizing various programs with abnormal white space.

use expect_test::expect;
use test_utils::tokenize_check;

#[test]
fn test_abnormal_whitespace() {
    tokenize_check(
        "\nlet     a  \n :int = \n 2     + 3\n\n     1 +  2\n\n\n\n",
        expect![[r#"
            Token { kind: Let, position: 1..4 }
            Token { kind: Id("a"), position: 9..10 }
            Token { kind: Colon, position: 14..15 }
            Token { kind: Id("int"), position: 15..18 }
            Token { kind: Equals, position: 19..20 }
            Token { kind: Int(2), position: 23..24 }
            Token { kind: Plus, position: 29..30 }
            Token { kind: Int(3), position: 31..32 }
            Token { kind: Int(1), position: 39..40 }
            Token { kind: Plus, position: 41..42 }
            Token { kind: Int(2), position: 44..45 }
        "#]],
    );
}
