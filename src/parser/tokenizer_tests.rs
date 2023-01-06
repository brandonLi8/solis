// Copyright © 2022 Brandon Li. All rights reserved.

//! Unit tests for the tokenizer.

use parser::tokenizer::*;

#[test]
fn test_empty() {
    assert_eq!(tokenize("".to_string()), vec![]);
}

#[test]
fn test_basic() {
    assert_eq!(
        tokenize("let varName: int = 32".to_string()),
        vec![
            Token {
                token_type: TokenType::Let,
                token_position: 0..3
            },
            Token {
                token_type: TokenType::Id("varName".to_string()),
                token_position: 4..11
            },
            Token {
                token_type: TokenType::Colon,
                token_position: 11..12
            },
            Token {
                token_type: TokenType::Id("int".to_string()),
                token_position: 13..16
            },
            Token {
                token_type: TokenType::Equals,
                token_position: 17..18
            },
            Token {
                token_type: TokenType::Int(32),
                token_position: 19..21
            },
        ]
    );

    assert_eq!(
        tokenize("let final varName: int = -123".to_string()),
        vec![
            Token {
                token_type: TokenType::Let,
                token_position: 0..3
            },
            Token {
                token_type: TokenType::Final,
                token_position: 4..9
            },
            Token {
                token_type: TokenType::Id("varName".to_string()),
                token_position: 10..17
            },
            Token {
                token_type: TokenType::Colon,
                token_position: 17..18
            },
            Token {
                token_type: TokenType::Id("int".to_string()),
                token_position: 19..22
            },
            Token {
                token_type: TokenType::Equals,
                token_position: 23..24
            },
            Token {
                token_type: TokenType::Int(-123),
                token_position: 25..29
            },
        ]
    );
}

#[test]
fn test_infix() {
    assert_eq!(
        tokenize("let varName: int = 32 - 2 + 3 * 4 / 5 % 1 + 2 * (3 + 1)".to_string()),
        vec![
            Token {
                token_type: TokenType::Let,
                token_position: 0..3
            },
            Token {
                token_type: TokenType::Id("varName".to_string()),
                token_position: 4..11
            },
            Token {
                token_type: TokenType::Colon,
                token_position: 11..12
            },
            Token {
                token_type: TokenType::Id("int".to_string()),
                token_position: 13..16
            },
            Token {
                token_type: TokenType::Equals,
                token_position: 17..18
            },
            Token {
                token_type: TokenType::Int(32),
                token_position: 19..21
            },
            Token {
                token_type: TokenType::Minus,
                token_position: 22..23
            },
            Token {
                token_type: TokenType::Int(2),
                token_position: 24..25
            },
            Token {
                token_type: TokenType::Plus,
                token_position: 26..27
            },
            Token {
                token_type: TokenType::Int(3),
                token_position: 28..29
            },
            Token {
                token_type: TokenType::Times,
                token_position: 30..31
            },
            Token {
                token_type: TokenType::Int(4),
                token_position: 32..33
            },
            Token {
                token_type: TokenType::Divide,
                token_position: 34..35
            },
            Token {
                token_type: TokenType::Int(5),
                token_position: 36..37
            },
            Token {
                token_type: TokenType::Mod,
                token_position: 38..39
            },
            Token {
                token_type: TokenType::Int(1),
                token_position: 40..41
            },
            Token {
                token_type: TokenType::Plus,
                token_position: 42..43
            },
            Token {
                token_type: TokenType::Int(2),
                token_position: 44..45
            },
            Token {
                token_type: TokenType::Times,
                token_position: 46..47
            },
            Token {
                token_type: TokenType::Lparen,
                token_position: 48..49
            },
            Token {
                token_type: TokenType::Int(3),
                token_position: 49..50
            },
            Token {
                token_type: TokenType::Plus,
                token_position: 51..52
            },
            Token {
                token_type: TokenType::Int(1),
                token_position: 53..54
            },
            Token {
                token_type: TokenType::Rparen,
                token_position: 54..55
            }
        ]
    );
}

#[test]
#[should_panic(expected = "Syntax Error at 13..14")]
fn test_syntax_error_basic_1() {
    tokenize("let varName: [int = 32".to_string());
}

#[test]
#[should_panic(expected = "Syntax Error at 15..16")]
fn test_syntax_error_basic_2() {
    tokenize("let varName: in[t = 32".to_string());
}

#[test]
#[should_panic(expected = "Syntax Error at 26..27")]
fn test_syntax_error_whitespace() {
    tokenize("\n  \nlet varName:     \nint [= 32 \n\n  ".to_string());
}
