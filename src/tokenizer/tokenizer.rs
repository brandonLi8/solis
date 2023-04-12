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
use error_messages::Position;
use lazy_static::lazy_static;
use regex::Regex;
use tokenizer::token_pattern::TokenPattern;

/// Different kinds of tokens and data associated with each token.
#[derive(Display, Debug)]
pub enum Token<'a> {
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
        token_pattern!(Token::Bool,              r"(true|false)\b" => bool),
        token_pattern!(Token::Float,             r"(([0-9]*\.[0-9]+\b)|([0-9]+\.[0-9]*))" => f64, error_if_next r"\."),
        token_pattern!(Token::Int,               r"([0-9]+)\b" => i64),

        // Keywords before Id
        token_pattern!(Token::Let,               r"let\b"),
        token_pattern!(Token::Colon,             r":"),
        token_pattern!(Token::Final,             r"final\b"),
        token_pattern!(Token::If,                r"if\b"),
        token_pattern!(Token::Else,              r"else\b"),
        token_pattern!(Token::Fun,               r"fun\b"),
        token_pattern!(Token::Comma,             r","),

        // Arithmetic
        token_pattern!(Token::Plus,              r"\+"),
        token_pattern!(Token::Minus,             r"-"),
        token_pattern!(Token::Mod,               r"%"),
        token_pattern!(Token::Times,             r"\*"),
        token_pattern!(Token::Divide,            r"/"),

        // Comparison Groups
        token_pattern!(Token::LessThanOrEquals,  r"<="),
        token_pattern!(Token::LessThan,          r"<"),

        token_pattern!(Token::MoreThanOrEquals,  r">="),
        token_pattern!(Token::MoreThan,          r">"),

        token_pattern!(Token::EqualsEquals,      r"=="),
        token_pattern!(Token::Equals,            r"="),
        token_pattern!(Token::NotEquals,         r"!="),
        token_pattern!(Token::Not,               r"!"),

        token_pattern!(Token::OpenParen,         r"\("),
        token_pattern!(Token::CloseParen,        r"\)"),
        token_pattern!(Token::OpenBrace,         r"\{"),
        token_pattern!(Token::CloseBrace,        r"\}"),
        token_pattern!(Token::Semi,              r";"),

        // Id
        token_pattern!(Token::Id => from_match   r"([A-Za-z][A-Za-z0-9_]*)\b"),
    ];
}

/// Tokenize the input file into a TokenIterator.
/// * context - compilation context
pub fn tokenize(context: &Context) -> impl Iterator<Item = (Token, Position)> {
    let mut cursor = 0;
    std::iter::from_fn(move || find_next_token(context, &mut cursor))
}

/// Function that finds the next token, advancing a passed in `cursor`
/// * context - compilation context
/// * cursor - the index the represents everything that has been tokenized already (to the left).
/// * return Option<(
///    - the token instance
///    - the position of the token, for error messaging purposes
/// * )>
pub fn find_next_token<'a>(context: &'a Context, cursor: &mut usize) -> Option<(Token<'a>, Position)> {
    if *cursor >= context.file.len() {
        return None;
    };

    // File slice starting at the cursor
    let file_slice = &context.file[*cursor..];

    // First search for ignore_patterns in the slice.
    for ignore_pattern in &*IGNORE_PATTERNS {
        if let Some(ignore_match) = ignore_pattern.find(file_slice) {
            *cursor += ignore_match.end();
            return find_next_token(context, cursor);
        }
    }

    // Find the next token at cursor
    for TokenPattern { match_regex, token_constructor, error_match } in &*TOKEN_PATTERNS {
        if let Some(token_match) = match_regex.find(file_slice) {
            let token = token_constructor(token_match.as_str());
            let position = *cursor..*cursor + token_match.end();

            *cursor += token_match.end();

            if let Some(error_match) = error_match {
                if error_match.find(&context.file[*cursor..]).is_some() {
                    compilation_error(context, &(*cursor..*cursor + 1), "Syntax Error: Invalid syntax")
                }
            }

            return Some((token, position));
        }
    }

    // At this point, nothing was found, so we raise a syntax error.
    compilation_error(
        context,
        &(*cursor..*cursor + 1),
        "Syntax Error: Invalid or unexpected token",
    )
}
