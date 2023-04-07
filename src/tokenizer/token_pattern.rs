// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! A `token_pattern` describes information to match and create different kinds of `TokenKinds`.
//! See `tokenizer.rs` for background context.

#![macro_use]
use regex::Regex;
use tokenizer::tokenizer::TokenKind;

pub struct TokenPattern {
    // Pattern to match for the token
    pub match_regex: Regex,

    // Converts matched text to a TokenKind instance.
    pub token_kind_constructor: fn(&str) -> TokenKind,

    // There can be some scenarios where we want to match a token with `match_regex`, but ensure that what is after
    // the match is not something else (`error_match`). For example, for floating point, we want to match `1.2`, but
    // not match `1.2.2`. This is a workaround since Rust's Regex crate does not support lookahead.
    pub error_match: Option<Regex>,
}

// Macro for creating a `TokenPattern`
macro_rules! token_pattern {
    // Helper token_pattern constructor.
    ($pattern:expr, $token_kind_constructor:expr, $error_match:expr) => {
        TokenPattern {
            match_regex: Regex::new(&format!("^{}", $pattern)).unwrap(),
            token_kind_constructor: $token_kind_constructor,
            error_match: $error_match
        }
    };

    // Token Pattern with no arguments passed to the TokenKind variant, and no error_match.
    ($token_kind:expr, $pattern:expr) => {
      token_pattern!($pattern, |_| $token_kind, None)
    };

    // Token Pattern with the matched string (only) passed to the TokenKind variant, and no error_match.
    ($token_kind:expr => from_match $pattern:expr) => {
      token_pattern!($pattern, |m| $token_kind(m), None)
    };

    // TokenKind variant where data is from a simple string parse.
    ($token_kind:expr, $pattern:expr => $to_type:ty, $error_match:expr) => {
        token_pattern!($pattern, |m| {
                $token_kind(
                    m.parse::<$to_type>()
                        .unwrap_or_else(|error|
                            crate::error_messages::internal_compiler_error(&format!("unable to parse {m}: {error}")),
                        ),
                )
            }, $error_match)
    };

    // TokenKind variant where data is from a simple string parse, with no error_match.
    ($token_kind:expr, $pattern:expr => $to_type:ty) => {
      token_pattern!($token_kind, $pattern => $to_type, None)
    };

    // TokenKind variant where data is from a simple string parse, with a error_match.
    ($token_kind:expr, $pattern:expr => $to_type:ty, error_if_next $error_match:expr) => {
        token_pattern!($token_kind, $pattern => $to_type, Some(Regex::new(&format!("^{}", $error_match)).unwrap()))
    };
}
