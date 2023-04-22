// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! A `TokenIterator` is a simple iterator for tokens that is returned from the tokenizer. For Solis, we are parsing a
//! LL(1) grammar, so we are parsing from left to right and on each production rule we only need to look ahead by 1
//! token. This iterator supports a few convenience methods, including advancing, peeking, and backwards peeking.
//! Here is an overview of the methods that it provides:
//!
//!   * `next`                - gets next token as an Option, indicating if there is a next
//!   * `advance`             - same as `next` but internal compiler error if EOF
//!   * `next_or_error`       - same as `next` but compilation error if EOF
//!   * `consume`             - if next = expected and not EOF next, advance. Else compilation error.
//!   * `peek`                - peek the next as an Option indicating if there is a next.
//!   * `peek_or_error`       - compilation error if Peek is None
//!
//! See the documentation of each method for full details.

use context::{Context, Position};
use error_messages::{compilation_error, internal_compiler_error, ErrorPosition};
use std::mem::discriminant;
use tokenizer::tokenizer::find_next_token;
use tokenizer::tokenizer::Token;

// Type alias to a tuple of a Token and its corresponding file position.
type TokenAndPosition<'a> = (Token<'a>, Position);

pub struct TokenIterator<'a> {
    // compilation context
    pub context: &'a Context,

    // index that represents everything that has been tokenized already (to the left).
    cursor: usize,

    // the peeked token and position
    peeked: Option<TokenAndPosition<'a>>,
}

impl<'a> TokenIterator<'a> {
    /// `TokenIterator` constructor
    pub fn new(context: &'a Context) -> Self {
        // Peek the first token
        let mut cursor = 0;
        let first_token = find_next_token(context, &mut cursor);

        Self { context, cursor, peeked: first_token }
    }

    // Advances the iterator
    // * return - a reference to the next found token and position. None value indicates we have reached EOF.
    fn next(&mut self) -> Option<TokenAndPosition<'a>> {
        // Move ownership from the peeked value
        let next = self.peeked.take();

        self.peeked = find_next_token(self.context, &mut self.cursor);
        next
    }

    /// Advances the iterator, **assuming that there is a next token** and throws a *internal* compiler error if not.
    pub fn advance(&mut self) -> TokenAndPosition<'a> {
        if let Some(next) = self.next() {
            next
        } else {
            internal_compiler_error("EOF inside advance")
        }
    }

    /// Advances the iterator, **where you are expecting there to be a next token but not fully sure** and throws a user
    /// *compilation* compiler error if not.
    /// * return - a reference to the unwrapped token and position
    pub fn next_or_error(&mut self) -> TokenAndPosition<'a> {
        match self.next() {
            Some(token_and_position) => token_and_position,
            None => compilation_error(
                self.context,
                ErrorPosition::EndOfFile,
                "Syntax Error: unexpected end of file",
            ),
        }
    }

    /// Ensures that next token is the same variant as `expected_token`. If it is, we "consume" it by advancing.
    /// Otherwise, it raises a compilation error.
    ///
    /// NOTE: this works for `Token`s that have data in them, because this checks that the `Token` *variant* matches.
    /// The data within the variant does not have to be equal, and the data within `expected_token` is irrelevant.
    pub fn consume_token(&mut self, expected_token: Token) -> TokenAndPosition<'a> {
        if let Some((token, position)) = &self.peeked {
            // See https://stackoverflow.com/questions/32554285/compare-enums-only-by-variant-not-value
            if discriminant(&expected_token) != discriminant(token) {
                compilation_error(
                    self.context,
                    ErrorPosition::WhitespaceBefore(position),
                    &format!("Syntax Error: expected `{expected_token}`",),
                )
            }

            self.advance()
        } else {
            compilation_error(
                self.context,
                ErrorPosition::EndOfFile,
                "Syntax Error: unexpected end of file",
            );
        }
    }

    /// Peeks the next value.
    /// * return - a reference to the next found token and position. None value indicates we have reached EOF.
    pub const fn peek(&self) -> Option<&TokenAndPosition<'a>> {
        self.peeked.as_ref()
    }

    /// Peeks the next value, **where you are expecting there to be a next token but not fully sure** and throws a user
    /// *compilation* compiler error if not.
    /// * return - a reference to the unwrapped token and position
    pub fn peek_or_error(&self) -> &TokenAndPosition {
        match self.peeked {
            None => compilation_error(
                self.context,
                ErrorPosition::EndOfFile,
                "Syntax Error: unexpected end of file",
            ),
            Some(ref peeked) => peeked,
        }
    }
}
