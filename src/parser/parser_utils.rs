// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Utility functions used in the parsing module.

use std::mem::discriminant;
use std::iter::Peekable;
use error_messages::{Position, internal_compiler_error};
use tokenizer::tokenizer::Token;

// Type alias to a tuple of a Token and its corresponding file position.
pub type TokenAndPosition<'a> = (Token<'a>, Position);

// ————————————————————————————————————————————————————————————————————————————*!
// The rest of the utility methods are related to operating on the result from
// the tokenizer (specifically, a `Peekable<Iterator<TokenAndPosition>>`). For
// Solis, we are parsing a LL(1) grammar, so we are parsing from left to right
// and on each production rule we only need to look ahead by 1 token. These are
// common patterns that have been turned into utility functions.
// Here is an overview of the utility methods below.
//
//   * `advance`             - same as `peekable.next` but *internal compiler* error if EOF
//   * `next_or_error`       - same as `peekable.next` but *compilation error* if EOF
//   * `consume`             - if `peekable.next` = expected and not EOF next, advance. Else compilation error.
//   * `peek_or_error`       - same as `peekable.peek` but compilation error if Peek is None
//
// See the documentation of each method for full details.
// ————————————————————————————————————————————————————————————————————————————*!

/// Advances the iterator, **assuming that there is a next token** and throws a *internal* compiler error if not.
pub fn advance<'a, I>(tokens: &'a mut Peekable<I>) -> TokenAndPosition // todo should not exist, make use of next_if
where
    I: Iterator<Item = (Token<'a>, Position)>
{
    match tokens.next() {
        Some(token_and_position) => token_and_position,
        None => internal_compiler_error("EOF inside advance")
    }
}

/// Advances the iterator, **where you are expecting there to be a next token but not fully sure** and throws a user
/// *compilation* compiler error if not.
pub fn next_or_error<'a, I>(tokens: &'a mut Peekable<I>) -> TokenAndPosition
where
    I: Iterator<Item = (Token<'a>, Position)>
{
    match tokens.next() {
        Some(token_and_position) => token_and_position,
        None => todo!() // compilation_error(self.context, &self.prev().1, "Syntax Error: unexpected end of file")
    }
}

/// Ensures that next token is the same variant as `expected_token`. If it is, we "consume" it by advancing.
/// Otherwise, it raises a compilation error.
///
/// NOTE: this works for `Token`s that have data in them, because this checks that the `Token` *variant* matches.
/// The data within the variant does not have to be equal, and the data within `expected_token` is irrelevant.
pub fn consume_token<'a, I>(tokens: &'a mut Peekable<I>, expected_token: Token, ) -> TokenAndPosition<'a>
where
    I: Iterator<Item = (Token<'a>, Position)>
{
    // See https://stackoverflow.com/questions/32554285/compare-enums-only-by-variant-not-value
    if let Some(token_and_position) = tokens.next_if(|(token, _)| discriminant(&expected_token) != discriminant(&token)) {
        token_and_position
    } else {
        // compilation_error(
        //     self.context,
        //     &self.prev().1,
        //     &format!("Syntax Error: expected `{expected_token}`"),
        // )
        todo!()
    }
}

/// Peeks the next value, and throws a user compilation compiler error if EOF.
/// * return - a reference to the unwrapped token and position
pub fn peek_or_error<'a, I>(tokens: &'a mut Peekable<I>) -> &'a TokenAndPosition<'a>
where
    I: Iterator<Item = (Token<'a>, Position)>
{
    match tokens.peek() {
        Some(ref peeked) => peeked,
        None => todo!(), // compilation_error(self.context, &self.prev().1, "Syntax Error: unexpected end of file"),
    }
}
