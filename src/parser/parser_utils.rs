// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Utility functions used in the parsing module.

use parser::ast::Block;
use parser::parse_expr::parse_expr;
use tokenizer::token_iterator::TokenIterator;
use tokenizer::tokenizer::Token;

/// Used to determine behavior of the `parse_block` function.
pub enum ParseBlockStopMode {
    /// Keep continuing to parse until the end of the file
    EndOfFile,

    /// Keep continuing to reaching the matching CloseBrace
    Brace,
}

/// Corresponds to `<block>` rule and parses into a `ast::Block`.
///
/// Note: the implementation of this doesn't match the rule definition in `.solis_grammar.txt`. Instead of recursively
/// filling the expressions of the block, we iteratively fill the block.
pub fn parse_block(stop_mode: ParseBlockStopMode, mut tokens: TokenIterator) -> (Block, TokenIterator) {
    let mut exprs = vec![];

    // Iteratively parse the expressions of the block.
    loop {
        match (&stop_mode, tokens.peek()) {
            // EndOfFile stop mode - clean break
            (ParseBlockStopMode::EndOfFile, None) => break,

            // Brace stop mode - advance then break
            (ParseBlockStopMode::Brace, Some((Token::CloseBrace, _))) => {
                tokens.advance();
                break;
            }

            // Continue condition
            _ => {
                let (next_expr, next_tokens) = parse_expr(tokens);

                // The compiler is not smart enough to figure out that we are reassigning the moved tokens, so we use
                // an intermediate `next_tokens` to explicitly do it.
                tokens = next_tokens;

                exprs.push(next_expr);

                // Remove optional semicolons. See https://github.com/brandonLi8/solis/issues/28
                if let Some((Token::Semi, _)) = tokens.peek() {
                    tokens.advance();
                }
            }
        }
    }

    (Block { exprs }, tokens)
}

/// Corresponds to `<comma-separated-list>` rule and parses into a `Vec<T>`.
/// * `parse_next` - function that parses the next item
///
/// Note: the implementation of this doesn't match the rule definition in `.solis_grammar.txt`. Instead of recursively
/// filling the expressions of the block, we iteratively fill the block.
pub fn parse_comma_separated_list<'a, T>(
    parse_next: fn(TokenIterator<'a>) -> (T, TokenIterator),
    mut tokens: TokenIterator<'a>,
) -> (Vec<T>, TokenIterator) {
    let mut items = vec![];

    while !matches!(tokens.peek_or_error().0, Token::CloseParen) {
        if !items.is_empty() {
            tokens.consume_token(Token::Comma);
        }

        let (item, next_tokens) = parse_next(tokens);
        items.push(item);

        // The compiler is not smart enough to figure out that we are reassigning the moved tokens, so we use
        // an intermediate `next_tokens` to explicitly do it.
        tokens = next_tokens;
    }

    tokens.consume_token(Token::CloseParen);
    (items, tokens)
}
