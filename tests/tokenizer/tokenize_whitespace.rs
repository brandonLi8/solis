// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Tests tokenizing various programs with abnormal white space.

use expect_test::expect;
use test_utils::tokenize_check;

#[test]
fn test_abnormal_whitespace() {
    tokenize_check(
        "\nlet     a  \n :int = \n 2     + 3\n\n     1 +  2\n\n\n\n",
        expect![[r#"
            Token `Let` at 1..4
            Token `Id("a")` at 9..10
            Token `Colon` at 14..15
            Token `Id("int")` at 15..18
            Token `Equals` at 19..20
            Token `Int(2)` at 23..24
            Token `Plus` at 29..30
            Token `Int(3)` at 31..32
            Token `Int(1)` at 39..40
            Token `Plus` at 41..42
            Token `Int(2)` at 44..45
        "#]],
    );
}
