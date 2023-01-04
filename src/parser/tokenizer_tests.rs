// Copyright © 2022 Brandon Li. All rights reserved.

//! Unit tests for the tokenizer.

use parser::tokenizer::*;

#[test]
fn test_empty() {
    assert_eq!(tokenize("".to_string()), vec![]);
}

#[test]
fn test_basic() {
    assert_eq!(tokenize("let varName: int = 32".to_string()), vec![
        Token { token_type: TokenType::Let, token_position: 0..3 },
        Token { token_type: TokenType::Id("varName".to_string()), token_position: 4..11 },
        Token { token_type: TokenType::Colon, token_position: 11..12 },
        Token { token_type: TokenType::Id("int".to_string()), token_position: 13..16 },
        Token { token_type: TokenType::Equals, token_position: 17..18 },
        Token { token_type: TokenType::Int(32), token_position: 19..21 },
    ]);

    assert_eq!(tokenize("let final varName: int = -123".to_string()), vec![
        Token { token_type: TokenType::Let, token_position: 0..3 },
        Token { token_type: TokenType::Final, token_position: 4..9 },
        Token { token_type: TokenType::Id("varName".to_string()), token_position: 10..17 },
        Token { token_type: TokenType::Colon, token_position: 17..18 },
        Token { token_type: TokenType::Id("int".to_string()), token_position: 19..22 },
        Token { token_type: TokenType::Equals, token_position: 23..24 },
        Token { token_type: TokenType::Int(-123), token_position: 25..29 },
    ]);
}

#[test] #[should_panic(expected = "Syntax Error at 13..14")]
fn test_syntax_error_basic_1() {
    tokenize("let varName: -int = 32".to_string());
}

#[test] #[should_panic(expected = "Syntax Error at 15..16")]
fn test_syntax_error_basic_2() {
    tokenize("let varName: in-t = 32".to_string());
}

#[test] #[should_panic(expected = "Syntax Error at 26..27")]
fn test_syntax_error_whitespace() {
    tokenize("\n  \nlet varName:     \nint -= 32 \n\n  ".to_string());
}

