// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! A tokenizer for Solis. A tokenizer takes in a Solis file raw string input and turns it
//! into a vector of Tokens. This is the first stage of the front end of the compiler. Working with
//! tokens will be much easier to work with compared to the raw string. For examples see `tokenizer_tests.rs`
//!
//! Internally, the tokenizer works by creating a regex pattern for each token. It will then match
//! the raw string to find the correct token, and then "consume" the token and move on. The process
//! repeats until all tokens have been consumed.
//!
//! For error checking, the tokenizer only checks for tokens that it recognizes, and doesn't do any other validation
//! or error checking. All other errors are deferred to the parser and code gen stages.

use error_messages::{compilation_error, internal_compiler_error};
use lazy_static::lazy_static;
use regex::Regex;
use std::ops::Range;
use File;

/// Different kinds of tokens and data associated with each token.
#[derive(PartialEq, Debug)]
pub enum TokenKind {
    // Literals
    Int(i64),
    Bool(bool),
    Float(f64),

    // Bindings
    Let,
    Colon,
    Final,
    Equals,
    Id(String),

    // Arithmetic Operators.
    Plus,  // For both unary and binary Plus
    Minus, // For both unary and binary Minus
    Mod,
    Times,
    Divide,

    // Prefix Operators
    Not,

    // Comparison Operators
    LessThan,
    LessThanOrEquals,
    MoreThan,
    MoreThanOrEquals,
    EqualsEquals,
    NotEquals,

    // Other
    OpenParen,
    CloseParen,
    Semi,
}

/// A token returned by the tokenizer
#[derive(PartialEq, Debug)]
pub struct Token {
    pub kind: TokenKind,

    /// For error messaging purposes, we need to link a token to where it was extracted in the original raw source code.
    /// `position` is the range (index based) of where the token is found in the raw input.
    pub position: Range<usize>,
}

// Struct that describes everything that is needed to to match and create a particular TokenKind.
struct TokenPattern {
    // Pattern to match for the token
    pub match_regex: Regex,

    // Converts matched text to a TokenKind instance.
    pub token_kind_constructor: fn(String) -> TokenKind,

    // There can be some scenarios where we want to match a token with `match_regex`, but ensure that what is after
    // the match is not something else (`error_match`). For example, for floating point, we want to match `1.2`, but
    // not match `1.2.2`. This is a workaround since Rust's Regex crate does not support lookahead.
    pub error_match: Option<Regex>,
}

// Macro for creating a `TokenPattern`
macro_rules! token_pattern {
    // Simple token_pattern with no arguments to the TokenKind variant
    ($token_kind:expr, $pattern:expr) => {
        TokenPattern {
            match_regex: Regex::new(&format!("^{}", $pattern)).unwrap(),
            token_kind_constructor: |_| $token_kind,
            error_match: None
        }
    };

    // TokenKind variant where data is from a simple string parse, with generic error_match
    ($token_kind:expr, $pattern:expr => $to_type:ty, $error_match:expr) => {
        TokenPattern {
            match_regex: Regex::new(&format!("^{}", $pattern)).unwrap(),
            token_kind_constructor: |m| {
                $token_kind(
                    m.parse::<$to_type>()
                        .unwrap_or_else(|error| internal_compiler_error(&format!("unable to parse {m}: {error}"))),
                )
            },
            error_match: $error_match
        }
    };

    // TokenKind variant where data is from a simple string parse, with no generic error_match
    ($token_kind:expr, $pattern:expr => $to_type:ty) => {
        token_pattern!($token_kind, $pattern => $to_type, None)
    };

    // TokenKind variant where data is from a simple string parse, with a error_match
    ($token_kind:expr, $pattern:expr => $to_type:ty, error_if_next $error_match:expr) => {
        token_pattern!($token_kind, $pattern => $to_type, Some(Regex::new(&format!("^{}", $error_match)).unwrap()))
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
    static ref TOKEN_PATTERNS: Vec<TokenPattern> = vec![
        // Match literals first
        token_pattern!(TokenKind::Bool,              r"(true|false)\b" => bool),
        token_pattern!(TokenKind::Float,             r"(([0-9]*\.[0-9]+\b)|([0-9]+\.[0-9]*))" => f64, error_if_next r"\."),
        token_pattern!(TokenKind::Int,               r"([0-9]+)\b" => i64),

        // Keywords before Id
        token_pattern!(TokenKind::Let,               r"let\b"),
        token_pattern!(TokenKind::Colon,             r":"),
        token_pattern!(TokenKind::Final,             r"final\b"),

        // Arithmetic
        token_pattern!(TokenKind::Plus,              r"\+"),
        token_pattern!(TokenKind::Minus,             r"-"),
        token_pattern!(TokenKind::Mod,               r"%"),
        token_pattern!(TokenKind::Times,             r"\*"),
        token_pattern!(TokenKind::Divide,            r"/"),

        // Comparison Groups
        token_pattern!(TokenKind::LessThanOrEquals,  r"<="),
        token_pattern!(TokenKind::LessThan,          r"<"),

        token_pattern!(TokenKind::MoreThanOrEquals,  r">="),
        token_pattern!(TokenKind::MoreThan,          r">"),

        token_pattern!(TokenKind::EqualsEquals,      r"=="),
        token_pattern!(TokenKind::Equals,            r"="),
        token_pattern!(TokenKind::NotEquals,         r"!="),
        token_pattern!(TokenKind::Not,               r"!"),

        token_pattern!(TokenKind::OpenParen,         r"\("),
        token_pattern!(TokenKind::CloseParen,        r"\)"),
        token_pattern!(TokenKind::Semi,              r";"),

        // Id
        token_pattern!(TokenKind::Id,                r"([A-Za-z][A-Za-z0-9_]*)\b" => String),
    ];
}

/// Tokenize the input file into a vector of tokens
pub fn tokenize(file: &File) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    // A cursor is the index the represents everything that has been tokenized already (to the left).
    let mut cursor = 0;

    'cursor_loop: while cursor < file.contents.len() {
        // File slice starting at the cursor
        let file_slice = &file.contents[cursor..];

        // First search for characters that we should ignore in the file.
        for ignore_pattern in &*IGNORE_PATTERNS {
            if let Some(ignore_match) = ignore_pattern.find(file_slice) {
                cursor += ignore_match.end();
                continue 'cursor_loop;
            }
        }

        // Find the next token at cursor
        for TokenPattern { match_regex, token_kind_constructor, error_match } in &*TOKEN_PATTERNS {
            if let Some(token_match) = match_regex.find(file_slice) {
                tokens.push(Token {
                    kind: token_kind_constructor(token_match.as_str().to_string()),
                    position: cursor..cursor + token_match.end(),
                });

                cursor += token_match.end();

                if let Some(error_match) = error_match {
                    if error_match.find(&file.contents[cursor..]).is_some() {
                        compilation_error(file, &(cursor..cursor + 1), "Syntax Error: Invalid syntax")
                    }
                }

                continue 'cursor_loop;
            }
        }

        // At this point, nothing was found, so we raise a syntax error.
        compilation_error(file, &(cursor..cursor + 1), "Syntax Error: Invalid or unexpected token")
    }

    tokens
}
