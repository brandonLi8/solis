// Copyright © 2022 Brandon Li. All rights reserved.

//! A tokenizer for Solis. A tokenizer takes in a Solis file raw string input and turns it
//! into a vector of Tokens. This is the first stage of the front end of the compiler. Working with
//! tokens will be much easier to work with compared to the raw string. For examples see tokenizer_tests.rs
//!
//! Internally, the tokenizer works by creating a regex pattern for each token. It will then match
//! the raw string to find the correct token, and then "consume" the token and move on. The process
//! repeats until all tokens have been consumed.
//!
//! For error checking, the tokenizer only checks for tokens that it recognizes, and doesn't do any other validation
//! or error checking. All other errors are deferred to the parser and code gen stages.

use lazy_static::lazy_static;
use regex::Regex;
use std::ops::Range;
use utils;

/// Different kinds of tokens and data associated with each token.
#[derive(PartialEq, Debug)]
pub enum TokenKind {
    // Bindings
    Let,
    Colon,
    Final,
    Equals,
    Id(String),

    // Literals
    Int(i32),

    // Infix Operators
    Plus,
    Minus,
    Mod,
    Times,
    Divide,
    OpenParen,
    CloseParen,
}

/// A token returned by the tokenizer
#[derive(PartialEq, Debug)]
pub struct Token {
    pub kind: TokenKind,

    /// For error messaging purposes, we need to link a token to where it was extracted in the original raw source code.
    /// `position` is the range (index based) of where the token is found in the raw input.
    pub position: Range<usize>,
}

// Converts matched text to a TokenKind instance.
type TokenKindConstructor = fn(String) -> TokenKind;

// Macro for creating a token pattern, which associates a regex pattern and a TokenKindConstructor as a tuple.
macro_rules! token_pattern {
    ($token_kind:expr, $pattern:expr) => {
        (Regex::new($pattern).unwrap(), |_| $token_kind)
    };
    ($token_kind:expr, $pattern:expr => $to_type:ty) => {
        (Regex::new($pattern).unwrap(), |m| {
            $token_kind(
                m.parse::<$to_type>()
                    .unwrap_or_else(|_| utils::internal_compiler_error(&format!("unable to parse {m}"))),
            )
        })
    };
}

lazy_static! {

    // Regex patterns for matching text that should be ignored in the input.
    static ref IGNORE_PATTERNS: Vec<Regex> = vec![

        // White Space
        Regex::new(r"^[ \n\t\s]+").unwrap(),

        // Line comments
        Regex::new(r"^#(\n|$|[^#].*)").unwrap(),

        // Block comments
        Regex::new(r"^##([\s\S]*?)##").unwrap(),
    ];

    // Regex patterns for matching different types of tokens.
    static ref TOKEN_PATTERNS: Vec<(Regex, TokenKindConstructor)> = vec![
        token_pattern!(TokenKind::Let,        r"^let\b"),
        token_pattern!(TokenKind::Colon,      r"^:"),
        token_pattern!(TokenKind::Final,      r"^final\b"),
        token_pattern!(TokenKind::Equals,     r"^="),
        token_pattern!(TokenKind::Id,         r"^([A-Za-z][A-Za-z0-9_]*)\b" => String),
        token_pattern!(TokenKind::Int,        r"^(-?[0-9]+)\b" => i32),
        token_pattern!(TokenKind::Plus,       r"^\+"),
        token_pattern!(TokenKind::Minus,      r"^-"),
        token_pattern!(TokenKind::Mod,        r"^%"),
        token_pattern!(TokenKind::Times,      r"^\*"),
        token_pattern!(TokenKind::Divide,     r"^/"),
        token_pattern!(TokenKind::OpenParen,  r"^\("),
        token_pattern!(TokenKind::CloseParen, r"^\)"),
    ];
}

/// Tokenize the input file into a vector of tokens
/// TODO: can we "stream" the file in, and "stream" the tokens out?
pub fn tokenize(file: &utils::File) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    // A cursor is the index the represents everything that has been tokenized already (to the left).
    let mut cursor = 0;

    'cursor_loop: while cursor < file.contents.len() {
        // File slice starting at the cursor
        let file_slice = &file.contents[cursor..];

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
                    kind: token_type_constructor(token_match.unwrap().as_str().to_string()),
                    position: cursor..cursor + token_match.unwrap().end(),
                });

                cursor += token_match.unwrap().end();
                continue 'cursor_loop;
            }
        }

        // At this point, nothing was found, so we raise a syntax error.
        utils::compilation_error(file, &(cursor..cursor + 1), "Syntax Error: Invalid or unexpected token")
    }

    return tokens;
}
