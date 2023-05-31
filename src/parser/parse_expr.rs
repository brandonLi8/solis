// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Defines the functions for parsing various types of expressions.

use parser::ast::{Block, Expr};
use parser::parse_infix::parse_infix_expr;
use parser::parser::parse_type;
use parser::parser_utils::{parse_block, parse_expr_and_position, ParseBlockStopMode};
use tokenizer::token_iterator::TokenIterator;
use tokenizer::tokenizer::Token;
use utils::error_messages::internal_compiler_error;

/// Corresponds to `<expr>` rule and parses into a `ast::Expr`.
pub fn parse_expr(tokens: TokenIterator) -> (Expr, TokenIterator) {
    match tokens.peek_or_error().0 {
        Token::Let => parse_let_expr(tokens),
        Token::If => parse_if_expr(tokens),
        _ => parse_infix_expr(tokens),
    }
}

/// Corresponds to `<let-expr>` rule and parses into a `ast::Expr::Let`.
pub fn parse_let_expr(mut tokens: TokenIterator) -> (Expr, TokenIterator) {
    tokens.consume_token(Token::Let);

    // Consume the let expression identifier
    let (id_token, id_position) = tokens.consume_token(Token::Id("identifier"));

    tokens.consume_token(Token::Colon);

    // Parse the type reference.
    let (type_reference, mut tokens) = parse_type(tokens);

    tokens.consume_token(Token::Equals);

    // Binding initial expression
    let ((init_expr, init_expr_position), tokens) = parse_expr_and_position(tokens);

    (
        Expr::Let {
            id: if let Token::Id(id) = id_token { id } else { internal_compiler_error("id not Token::Id variant") },
            id_position,
            type_reference,
            init_expr: Box::new(init_expr),
            init_expr_position,
        },
        tokens,
    )
}

/// Corresponds to `<if-expr>` rule and parses into a `ast::Expr::If`.
pub fn parse_if_expr(mut tokens: TokenIterator) -> (Expr, TokenIterator) {
    let (_, if_position) = tokens.consume_token(Token::If);

    // Parse the condition expression
    let (condition, mut tokens) = parse_expr(tokens);

    tokens.consume_token(Token::OpenBrace);

    // Parse the consequent block
    let (then_block, tokens) = parse_block(ParseBlockStopMode::Brace, tokens);

    // Parse the alternate block
    let (else_block, tokens) = match tokens.peek() {
        Some((Token::Else, _)) => {
            let (else_block, tokens) = parse_else_block(tokens);
            (Some(else_block), tokens)
        }
        _ => (None, tokens),
    };

    (
        Expr::If { condition: Box::new(condition), then_block, else_block, if_position },
        tokens,
    )
}

// Corresponds to `<else-block>` rule and parses into a `ast::Block`.
fn parse_else_block(mut tokens: TokenIterator) -> (Block, TokenIterator) {
    tokens.consume_token(Token::Else);

    // Else If vs
    if let Token::If = tokens.peek_or_error().0 {
        let (else_expr, tokens) = parse_if_expr(tokens);
        (Block { exprs: vec![else_expr] }, tokens)
    } else {
        tokens.consume_token(Token::OpenBrace);
        parse_block(ParseBlockStopMode::Brace, tokens)
    }
}
