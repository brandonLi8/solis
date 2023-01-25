// Copyright © 2022 Brandon Li. All rights reserved.

//! A `TokensCursor` is a simple iterator for tokens for the purposes of parsing. For Solis, we are parsing a LL(1)
//! grammar, so we are parsing from left to right and on each production rule we only need to look ahead by 1 token.
//! This iterator supports a few convenience methods, including advancing, peeking, and backwards peeking.
//! Here is an overview of the methods that it provides:
//!
//!   * `Advance` - internal compiler error if EOF
//!   * `Next` - same as advance but compilation error if EOF
//!   * `Consume` - if next = expected and not EOF next, advance. Else compilation error.
//!   * `Peek` - peek the next as an Option indicating if there is a next.
//!   * `PeekUnwrap` - compilation error if Peek is None
//!   * `Prev` - gets prev, with a internal error if position is 0
//!
//! See the documentation of each method for full details.

use error_messages::{compilation_error, internal_compiler_error};
use std::mem::discriminant;
use tokenizer::tokenizer::{Token, TokenKind};
use File;

pub struct TokensCursor<'a> {
    /// Reference to the tokens to be parsed.
    tokens: &'a [Token],

    /// Current cursor position within the tokens. Formally, it is the index of the next token of the iterator.
    position: usize,

    /// The original Solis input file, for error messaging purposes.
    pub file: &'a File,
}

impl<'a> TokensCursor<'a> {
    /// Tokens Cursor constructor.
    /// * tokens: tokens to parse
    /// * file: the original Solis file
    pub const fn new(tokens: &'a [Token], file: &'a File) -> Self {
        TokensCursor { tokens, position: 0, file }
    }

    /// Advances the cursor forwards (to the right) by one, **assuming that there is a next token** and throws an
    /// *internal* compiler error if not.
    pub fn advance(&mut self) {
        if self.is_end_of_file() {
            internal_compiler_error("End of File inside advance")
        }
        self.position += 1;
    }

    /// Advances the cursor forwards (to the right) by one, **where you are expecting there to be a next token but not
    /// fully sure** and throws a user *compilation* compiler error if not. This also returns the advanced token.
    pub fn next(&mut self) -> (&Token, &Self) {
        if self.is_end_of_file() {
            compilation_error(self.file, &self.prev().position, "Syntax Error: unexpected end of file")
        }
        let next_token = &self.tokens[self.position];
        self.advance();
        (next_token, self)
    }

    /// Ensures that next token has kind `expected_token_kind`. If it is, we "consume" it by advancing.
    /// Otherwise, it raises a compilation error.
    ///
    /// NOTE: this works for `TokenKinds` that have data in them, because this checks that the `TokenKind` *variant*.
    /// matches. The actual data within the variant does not have to be equal, and the data within `expected_token_kind`
    /// is used for further error messaging help.
    pub fn consume_token(&mut self, expected_token_kind: TokenKind) {
        if self.is_end_of_file() {
            compilation_error(self.file, &self.prev().position, "Syntax Error: unexpected end of file")
        }

        // See https://stackoverflow.com/questions/32554285/compare-enums-only-by-variant-not-value
        if discriminant(&expected_token_kind) != discriminant(&self.tokens[self.position].kind) {
            compilation_error(
                self.file,
                &self.prev().position,
                &format!("Syntax Error: expected {expected_token_kind:?}"),
            )
        }
        self.advance();
    }

    /// Gets the previous token, **assuming that advance() has been called before** and throws a user compilation error
    /// if not.
    pub fn prev(&self) -> &'a Token {
        if self.position == 0 {
            internal_compiler_error("prev() but no advance() has been called")
        }
        &self.tokens[self.position - 1]
    }

    /// Peeks the next value. It returns an option, where None indicates that we are at the end of the file.
    pub fn peek(&mut self) -> (Option<&Token>, &mut Self) {
        if self.is_end_of_file() {
            (None, self)
        } else {
            (Some(&self.tokens[self.position]), self)
        }
    }

    /// Peeks the next value, **where you are fully expecting there to be a next token**. It can be logically thought of
    /// as peeking, and unwrapping the value, and throwing a compilation error on unwrap failure.
    pub fn peek_unwrap(&mut self) -> (&Token, &mut Self) {
        if self.is_end_of_file() {
            compilation_error(self.file, &self.prev().position, "Syntax Error: unexpected end of file")
        } else {
            (&self.tokens[self.position], self)
        }
    }

    /// Returns whether we are at the end of the file (no next token) or not.
    pub const fn is_end_of_file(&self) -> bool {
        self.tokens.len() == self.position
    }
}
