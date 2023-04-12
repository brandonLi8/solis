// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Defines the functions for parsing various types of expressions.

use error_messages::internal_compiler_error;
use parser::ast::{Block, Expr, ExprKind};
use parser::parse_infix::parse_infix_expr;
use parser::parser::{parse_closed_block, parse_type};
use parser::tokens_cursor::TokensCursor;
use tokenizer::tokenizer::Token;

/// Corresponds to <expr> rule and parses into `ast::Expr`.
pub fn parse_expr(tokens_cursor: &mut TokensCursor) -> Expr {
    let (next_token, tokens_cursor) = tokens_cursor.peek_unwrap();

    match &next_token.0 {
        Token::Let => parse_let_expr(tokens_cursor),
        Token::If => parse_if_expr(tokens_cursor),
        _ => parse_infix_expr(tokens_cursor),
    }
}

/// Corresponds to <let-expr> rule and parses into `ast::Expr::Let`.
pub fn parse_let_expr(tokens_cursor: &mut TokensCursor) -> Expr {
    tokens_cursor.consume_token(Token::Let);

    // Consume the let expression identifier
    tokens_cursor.consume_token(Token::Id(&"identifier".to_string()));
    let ((id_token_kind, _), tokens_cursor) = tokens_cursor.prev();

    tokens_cursor.consume_token(Token::Colon);

    // Parse the type reference.
    let type_reference = parse_type(tokens_cursor);
    let (type_reference_token, tokens_cursor) = tokens_cursor.prev();

    tokens_cursor.consume_token(Token::Equals);

    // Binding initial expression
    let init_expr = parse_expr(tokens_cursor);

    if let Token::Id(id) = id_token_kind {
        Expr {
            kind: ExprKind::Let { id: id.to_string(), type_reference, init_expr: Box::new(init_expr) },
            position: type_reference_token.1.clone(),
        }
    } else {
        internal_compiler_error("Unable to get id. Should have been consumed.")
    }
}

/// Corresponds to <if-expr> rule and parses into `ast::Expr::If`.
pub fn parse_if_expr(tokens_cursor: &mut TokensCursor) -> Expr {
    tokens_cursor.consume_token(Token::If);
    let (if_token, tokens_cursor) = tokens_cursor.prev();

    // Parse the condition expression
    let condition = parse_expr(tokens_cursor);

    tokens_cursor.consume_token(Token::OpenBrace);

    // Parse the consequent block
    let then_block = parse_closed_block(Block { exprs: vec![] }, tokens_cursor);

    // Peek the next two tokens
    let (next_token, tokens_cursor) = tokens_cursor.peek();

    // Parse the alternate block
    let else_block = match next_token {
        Some((Token::Else, _)) => Some(parse_else_block(tokens_cursor)),
        _ => None,
    };

    Expr {
        kind: ExprKind::If { condition: Box::new(condition), then_block, else_block },
        position: if_token.1.clone(),
    }
}

// Corresponds to <else-block> rule and parses into ast::Block.
fn parse_else_block(tokens_cursor: &mut TokensCursor) -> Block {
    tokens_cursor.consume_token(Token::Else);

    // Peek the next token
    let (next_token, tokens_cursor) = tokens_cursor.peek_unwrap();

    if let Token::If = &next_token.0 {
        Block { exprs: vec![parse_if_expr(tokens_cursor)] }
    } else {
        tokens_cursor.consume_token(Token::OpenBrace);
        parse_closed_block(Block { exprs: vec![] }, tokens_cursor)
    }
}
