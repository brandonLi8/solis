// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! A `token_pattern` describes information to match and create different types of `Token`s.
//! See `tokenizer.rs` for background context.

#![macro_use]

use regex::Regex;
use tokenizer::tokenizer::Token;

pub struct TokenPattern {
    // Pattern to match for the token
    pub match_regex: Regex,

    // Converts matched text to a Token instance.
    pub token_constructor: fn(&str) -> Token,

    // There can be some scenarios where we want to match a token with `match_regex`, but ensure that what is after
    // the match is not something else (`error_match`). For example, for floating point, we want to match `1.2`, but
    // not match `1.2.2`. This is a workaround since Rust's Regex crate does not support look-ahead.
    pub error_match: Option<Regex>,
}

// Macro for creating a `TokenPattern`
macro_rules! token_pattern {
    // Helper token_pattern constructor.
    ($pattern:expr, $token_constructor:expr, $error_match:expr) => {
        TokenPattern {
            match_regex: Regex::new(&format!("^{}", $pattern)).unwrap(),
            token_constructor: $token_constructor,
            error_match: $error_match
        }
    };

    // Token Pattern with no arguments passed to the Token variant, and no error_match.
    ($token_variant:expr, $pattern:expr) => {
        token_pattern!($pattern, |_| $token_variant, None)
    };

    // Token Pattern with the matched string (only) passed to the Token variant, and no error_match.
    ($token_variant:expr => from_match $pattern:expr) => {
        token_pattern!($pattern, |m| $token_variant(m), None)
    };

    // Token variant where data is from a simple string parse.
    ($token_variant:expr, $pattern:expr => $to_type:ty, $error_match:expr) => {
        token_pattern!($pattern, |m| {
                $token_variant(
                    m.parse::<$to_type>()
                        .unwrap_or_else(|error|
                            crate::utils::error_messages::internal_compiler_error(&format!("unable to parse {m}: {error}")),
                        ),
                )
            }, $error_match)
    };

    // Token variant where data is from a simple string parse, with no error_match.
    ($token_variant:expr, $pattern:expr => $to_type:ty) => {
        token_pattern!($token_variant, $pattern => $to_type, None)
    };

    // Token variant where data is from a simple string parse, with a error_match.
    ($token_variant:expr, $pattern:expr => $to_type:ty, error_if_next $error_match:expr) => {
        token_pattern!($token_variant, $pattern => $to_type, Some(Regex::new(&format!("^{}", $error_match)).unwrap()))
    };
}
