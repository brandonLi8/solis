// Copyright © 2022 Brandon Li. All rights reserved.

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
            Token { kind: Let, position: 18..21 }
            Token { kind: Id("a"), position: 22..23 }
            Token { kind: Colon, position: 23..24 }
            Token { kind: Id("int"), position: 25..28 }
            Token { kind: Equals, position: 29..30 }
            Token { kind: Id("b"), position: 31..32 }
            Token { kind: Let, position: 56..59 }
            Token { kind: Id("a"), position: 60..61 }
            Token { kind: Colon, position: 61..62 }
            Token { kind: Id("int"), position: 82..85 }
            Token { kind: Equals, position: 86..87 }
            Token { kind: Id("b"), position: 88..89 }
            Token { kind: Int(1), position: 219..220 }
            Token { kind: Plus, position: 221..222 }
            Token { kind: Int(2), position: 223..224 }
            Token { kind: Id("a"), position: 252..253 }
        "#]],
    )
}
