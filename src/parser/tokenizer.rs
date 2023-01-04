// Copyright © 2022 Brandon Li. All rights reserved.

//! A tokenizer for Solis. A tokenizer takes in a Solis file raw string input and turns it
//! into a vector of Tokens. This is the first stage of the front end of the compiler. Working with
//! tokens will be much easier to work with compared to the raw string. For examples see tokenizer_tests.rs
//!
//! Internally, the tokenizer works by creating a regex pattern for each token. It will then match
//! the raw string to find the correct token, and then "consume" the token and move on. The process
//! repeats until all tokens have been consumed.

use lazy_static::lazy_static;
use regex::Regex;
use std::ops::Range;
use utils;

/// Different types of tokens and data associated with each token.
#[derive(PartialEq, Debug)]
pub enum TokenType {
    Let,
    Colon,
    Final,
    Equals,
    Id(String),
    Int(i32),
}

/// A token returned by the tokenizer
#[derive(PartialEq, Debug)]
pub struct Token {
    pub token_type: TokenType,

    /// For error messaging purposes, we need to link a token to where it was extracted in the original raw source code.
    /// token_position is the range (index based) of where the token is found in the raw input.
    pub token_position: Range<usize>,
}

// Converts matched text to a TokenType instance.
type TokenTypeConstructor = fn(String) -> TokenType;

lazy_static! {

    // Regex patterns for matching text that should be ignored in the input.
    static ref IGNORE_PATTERNS: Vec<Regex> = vec![

        // White Space
        Regex::new(r"^[ \n\t\s]+").unwrap()
    ];

    // Regex patterns for matching different types of tokens.
    static ref TOKEN_PATTERNS: Vec<(Regex, TokenTypeConstructor)> = vec![
        /* LET */    (Regex::new(r"^let\b").unwrap(), |_| TokenType::Let),
        /* COLON */  (Regex::new(r"^:").unwrap(), |_| TokenType::Colon),
        /* FINAL */  (Regex::new(r"^final\b").unwrap(), |_| TokenType::Final),
        /* EQUALS */ (Regex::new(r"^=").unwrap(), |_| TokenType::Equals),
        /* ID */     (Regex::new(r"^([A-Za-z][A-Za-z0-9_]*)\b").unwrap(), |m| TokenType::Id(m)),
        /* INT */    (Regex::new(r"^(-?[0-9]+)\b").unwrap(), |m| TokenType::Int(m.parse::<i32>().expect(&format!("unable to convert {} to int", m)))),
    ];
}

/// Tokenize the input file into a vector of tokens
/// TODO: can we "stream" the file in, and "stream" the tokens out?
pub fn tokenize(file: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    // A cursor is the index the represents everything that has been tokenized already (to the left).
    let mut cursor = 0;

    'cursor_loop: while cursor < file.len() {
        // File slice starting at the cursor
        let file_slice = &file[cursor..];

        // First search for characters that we should ignore in the file.
        for ignore_pattern in &*IGNORE_PATTERNS {
            let ignore_match = ignore_pattern.find(file_slice);
            if ignore_match.is_some() {
                cursor += ignore_match.unwrap().end();
                continue 'cursor_loop;
            }
        }

        // Find the next token at cursor
        for (token_pattern, token_type_constructor) in &*TOKEN_PATTERNS {
            let token_match = token_pattern.find(file_slice);
            if token_match.is_some() {
                tokens.push(Token {
                    token_type: token_type_constructor(token_match.unwrap().as_str().to_string()),
                    token_position: cursor..cursor + token_match.unwrap().end(),
                });

                cursor += token_match.unwrap().end();
                continue 'cursor_loop;
            }
        }

        // At this point, nothing was found, so we raise a syntax error.
        utils::raise_code_error(file, cursor..cursor + 1, "Syntax Error")
    }

    return tokens;
}
