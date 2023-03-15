// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Defines the functions for parsing various types of expressions.

use error_messages::{compilation_error, internal_compiler_error};
use parser::ast::{Block, Expr, ExprKind, Type};
use parser::parse_infix::parse_infix_expr;
use parser::tokens_cursor::TokensCursor;
use tokenizer::tokenizer::{Token, TokenKind};

/// Corresponds to <expr> rule and parses into `ast::Expr`.
pub fn parse_expr(tokens_cursor: &mut TokensCursor) -> Expr {
    let (next_token, tokens_cursor) = tokens_cursor.peek_unwrap();

    let expr = match &next_token.kind {
        TokenKind::Let => parse_let_expr(tokens_cursor),
        TokenKind::If => parse_if_expr(tokens_cursor),
        _ => parse_infix_expr(tokens_cursor),
    };

    // Remove optional semicolons. See https://github.com/brandonLi8/solis/issues/28
    while let (Some(Token { kind: TokenKind::Semi, .. }), _) = tokens_cursor.peek() {
        tokens_cursor.advance();
    }

    expr
}

/// Corresponds to <let-expr> rule and parses into `ast::Expr::Let`.
pub fn parse_let_expr(tokens_cursor: &mut TokensCursor) -> Expr {
    tokens_cursor.consume_token(TokenKind::Let);

    // Consume the let expression identifier
    tokens_cursor.consume_token(TokenKind::Id("any variable name".to_string()));
    let id_token_kind = &tokens_cursor.prev().kind;

    tokens_cursor.consume_token(TokenKind::Colon);

    // Parse the type reference.
    let type_reference = parse_type(tokens_cursor);
    let type_reference_token = &tokens_cursor.prev();

    tokens_cursor.consume_token(TokenKind::Equals);

    // Binding initial expression
    let init_expr = parse_expr(tokens_cursor);

    if let TokenKind::Id(id) = id_token_kind {
        Expr {
            kind: ExprKind::Let { id: id.to_string(), type_reference, init_expr: Box::new(init_expr) },
            position: type_reference_token.position.clone(),
        }
    } else {
        internal_compiler_error("Unable to get id. Should have been consumed.")
    }
}

/// Corresponds to <type> rule and parses into `ast::Type`.
pub fn parse_type(tokens_cursor: &mut TokensCursor) -> Type {
    let (next_token, tokens_cursor) = tokens_cursor.next();

    match &next_token.kind {
        TokenKind::Id(id) => match id.as_str() {
            "int" => Type::Int,
            "bool" => Type::Bool,
            "float" => Type::Float,
            _ => compilation_error(tokens_cursor.file, &next_token.position, &format!("Invalid type: {id}")),
        },
        TokenKind::OpenParen => {
            tokens_cursor.consume_token(TokenKind::CloseParen);
            Type::Unit
        }
        _ => todo!(),
    }
}

/// Corresponds to <if-expr> rule and parses into `ast::Expr::If`.
pub fn parse_if_expr(tokens_cursor: &mut TokensCursor) -> Expr {
    tokens_cursor.consume_token(TokenKind::If);
    let if_token = tokens_cursor.prev();

    // Parse the condition expression
    let condition = parse_expr(tokens_cursor);

    tokens_cursor.consume_token(TokenKind::OpenBrace);

    // Parse the consequent block
    let then_block = parse_if_body(Block { exprs: vec![] }, tokens_cursor);

    // Peek the next two tokens
    let (next_token, tokens_cursor) = tokens_cursor.peek();

    // Parse the alternate block
    let else_block = match next_token {
        Some(Token { kind: TokenKind::Else, .. }) => Some(parse_else_block(tokens_cursor)),
        _ => None,
    };

    Expr {
        kind: ExprKind::If { condition: Box::new(condition), then_block, else_block },
        position: if_token.position.clone(),
    }
}

// Corresponds to <else-block> rule and parses into ast::Block.
fn parse_else_block(tokens_cursor: &mut TokensCursor) -> Block {
    tokens_cursor.consume_token(TokenKind::Else);

    // Peek the next token
    let (next_token, tokens_cursor) = tokens_cursor.peek_unwrap();

    if let TokenKind::If = &next_token.kind {
        Block { exprs: vec![parse_if_expr(tokens_cursor)] }
    } else {
        tokens_cursor.consume_token(TokenKind::OpenBrace);
        parse_if_body(Block { exprs: vec![] }, tokens_cursor)
    }
}

// Corresponds to <if-body> rule and parses into ast::Block.
// * block: in order to inexpensively parse expressions and add them to a result block, recursively.
fn parse_if_body(mut block: Block, tokens_cursor: &mut TokensCursor) -> Block {
    let (next_token, tokens_cursor) = tokens_cursor.peek();

    if let Some(Token { kind: TokenKind::CloseBrace, .. }) = next_token {
        tokens_cursor.advance();
        block
    } else {
        let next_expr = parse_expr(tokens_cursor);
        block.exprs.push(next_expr);
        parse_if_body(block, tokens_cursor)
    }
}
