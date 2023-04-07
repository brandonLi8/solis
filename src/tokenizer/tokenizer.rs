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

use context::Context;
use derive_more::Display;
use error_messages::compilation_error;
use lazy_static::lazy_static;
use regex::Regex;
use std::ops::Range;
use tokenizer::token_pattern::TokenPattern;

/// Different kinds of tokens and data associated with each token.
#[derive(Display, Debug)]
pub enum TokenKind<'a> {
    // Literals
    Int(i64),
    Bool(bool),
    Float(f64),

    // Bindings
    #[display(fmt = "let")]
    Let,

    #[display(fmt = ":")]
    Colon,
    Final,

    #[display(fmt = "=")]
    Equals,
    Id(&'a str),

    // If
    #[display(fmt = "if")]
    If,

    #[display(fmt = "else")]
    Else,

    // Functions
    #[display(fmt = "fun")]
    Fun,

    #[display(fmt = ",")]
    Comma,

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
    #[display(fmt = "(")]
    OpenParen,

    #[display(fmt = ")")]
    CloseParen,

    #[display(fmt = "{{")]
    OpenBrace,

    #[display(fmt = "}}")]
    CloseBrace,
    Semi,
}

/// A token returned by the tokenizer
#[derive(Debug)]
pub struct Token<'a> {
    pub kind: TokenKind<'a>,

    /// For error messaging purposes, we need to link a token to where it was extracted in the original raw source code.
    /// `position` is the range (index based) of where the token is found in the raw input.
    pub position: Range<usize>,
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
        token_pattern!(TokenKind::If,                r"if\b"),
        token_pattern!(TokenKind::Else,              r"else\b"),
        token_pattern!(TokenKind::Fun,               r"fun\b"),
        token_pattern!(TokenKind::Comma,             r","),

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
        token_pattern!(TokenKind::OpenBrace,         r"\{"),
        token_pattern!(TokenKind::CloseBrace,        r"\}"),
        token_pattern!(TokenKind::Semi,              r";"),

        // Id
        token_pattern!(TokenKind::Id => from_match   r"([A-Za-z][A-Za-z0-9_]*)\b"),
    ];
}

/// Tokenize the input file into a iterator of tokens.
/// * context - compilation context
pub fn tokenize(context: &Context) -> impl Iterator<Item = Token> {
    // A file_cursor is the index the represents everything that has been tokenized already (to the left).
    let mut file_cursor = 0;

    std::iter::from_fn(move || find_next_token(context, &mut file_cursor))
}

// Function that finds the next token, advancing a passed in `file_cursor`
// * context - compilation context
// * file_cursor - the index the represents everything that has been tokenized already (to the left).
fn find_next_token<'a>(context: &'a Context, file_cursor: &mut usize) -> Option<Token<'a>> {
    if *file_cursor >= context.file.len() {
        return None
    };

    // File slice starting at the file_cursor
    let file_slice = &context.file[*file_cursor..];

    // First search for ignore_patterns in the slice.
    for ignore_pattern in &*IGNORE_PATTERNS {
        if let Some(ignore_match) = ignore_pattern.find(file_slice) {
            *file_cursor += ignore_match.end();
            return find_next_token(context, file_cursor)
        }
    }

    // Find the next token at file_cursor
    for TokenPattern { match_regex, token_kind_constructor, error_match } in &*TOKEN_PATTERNS {
        if let Some(token_match) = match_regex.find(file_slice) {
            let token = Token {
                kind: token_kind_constructor(token_match.as_str()),
                position: *file_cursor..*file_cursor + token_match.end(),
            };

            *file_cursor += token_match.end();

            if let Some(error_match) = error_match {
                if error_match.find(&context.file[*file_cursor..]).is_some() {
                    compilation_error(context, &(*file_cursor..*file_cursor + 1), "Syntax Error: Invalid syntax")
                }
            }

            return Some(token)
        }
    }

    // At this point, nothing was found, so we raise a syntax error.
    compilation_error(
        context,
        &(*file_cursor..*file_cursor + 1),
        "Syntax Error: Invalid or unexpected token",
    )
}
