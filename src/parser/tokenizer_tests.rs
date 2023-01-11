// Copyright © 2022 Brandon Li. All rights reserved.

//! Unit tests for the tokenizer.

use expect_test::{expect, Expect};
use parser::tokenizer::tokenize;
use utils;

#[test]
fn test_empty() {
    tokenize_check("", expect![""]);
}

#[test]
fn test_basic() {
    tokenize_check(
        "let name: int = 32",
        expect![[r#"
            Token { kind: Let, position: 0..3 }
            Token { kind: Id("name"), position: 4..8 }
            Token { kind: Colon, position: 8..9 }
            Token { kind: Id("int"), position: 10..13 }
            Token { kind: Equals, position: 14..15 }
            Token { kind: Int(32), position: 16..18 }
        "#]],
    );

    tokenize_check(
        "let final name: int = -123",
        expect![[r#"
            Token { kind: Let, position: 0..3 }
            Token { kind: Final, position: 4..9 }
            Token { kind: Id("name"), position: 10..14 }
            Token { kind: Colon, position: 14..15 }
            Token { kind: Id("int"), position: 16..19 }
            Token { kind: Equals, position: 20..21 }
            Token { kind: Int(-123), position: 22..26 }
        "#]],
    );
}

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
}

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

#[test]
#[should_panic(expected = "Syntax Error: Invalid or unexpected token at 10..11")]
fn test_syntax_error_basic_1() {
    tokenize_check("let name: [int = 32", expect![]);
}

#[test]
#[should_panic(expected = "Syntax Error: Invalid or unexpected token at 12..13")]
fn test_syntax_error_basic_2() {
    tokenize_check("let name: in[t = 32", expect![]);
}

#[test]
#[should_panic(expected = "Syntax Error: Invalid or unexpected token at 23..24")]
fn test_syntax_error_whitespace() {
    tokenize_check("\n  \nlet name:     \nint [= 32 \n\n  ", expect![]);
}

/// A helper function to test tokenizing a program, where the filename does not matter and only the contents matter.
pub fn tokenize_check(program: &str, expect: Expect) {
    let tokens = tokenize(&utils::File { name: String::new(), contents: program.to_string() });
    expect.assert_eq(
        &tokens
            .iter()
            .fold(String::new(), |acc, token| acc + &format!("{token:?}") + "\n"),
    )
}
