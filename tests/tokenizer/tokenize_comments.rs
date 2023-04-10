// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Tests tokenizing various programs with comments.

use expect_test::expect;
use test_utils::tokenize_check;

#[test]
fn test_comments_ignored() {
    tokenize_check(
        r"# comment
        let a: int = b # some comment
        let a: ## some comment ## int = b ## some comment ##
        #

        ## block
        comment
        ##

        ### block
        comment
        ##

        1 + 2

        # comment
        a
        ",
        expect![[r#"
            Token `Let` at 18..21
            Token `Id("a")` at 22..23
            Token `Colon` at 23..24
            Token `Id("int")` at 25..28
            Token `Equals` at 29..30
            Token `Id("b")` at 31..32
            Token `Let` at 56..59
            Token `Id("a")` at 60..61
            Token `Colon` at 61..62
            Token `Id("int")` at 82..85
            Token `Equals` at 86..87
            Token `Id("b")` at 88..89
            Token `Int(1)` at 219..220
            Token `Plus` at 221..222
            Token `Int(2)` at 223..224
            Token `Id("a")` at 252..253
        "#]],
    );
}
