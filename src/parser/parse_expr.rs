// Copyright © 2022 Brandon Li. All rights reserved.

//! Defines the functions for parsing various types of expressions.

use error_messages::internal_compiler_error;
use parser::ast::{Expr, ExprKind};
use parser::parse_infix::parse_infix_expr;
use parser::tokens_cursor::TokensCursor;
use tokenizer::tokenizer::{Token, TokenKind};

// Corresponds to <expr> rule and parses into ast::Expr.
pub fn parse_expr(tokens_cursor: &mut TokensCursor) -> Expr {
    let (next_token, tokens_cursor) = tokens_cursor.peek_unwrap();

    let expr = match &next_token.kind {
        TokenKind::Let => parse_let_expr(tokens_cursor),
        _ => parse_infix_expr(tokens_cursor),
    };

    // Remove optional semicolons. See https://github.com/brandonLi8/solis/issues/28
    while let (Some(Token { kind: TokenKind::Semi, .. }), _) = tokens_cursor.peek() {
        tokens_cursor.advance();
    }

    expr
}

// Corresponds to <let-expr> rule and parses into ast::Expr::Let.
pub fn parse_let_expr(tokens_cursor: &mut TokensCursor) -> Expr {
    tokens_cursor.consume_token(TokenKind::Let);

    // Consume the let expression identifier
    tokens_cursor.consume_token(TokenKind::Id("any variable name".to_string()));
    let id_token_kind = &tokens_cursor.prev().kind;

    tokens_cursor.consume_token(TokenKind::Colon);

    // Consume the type reference identifier
    tokens_cursor.consume_token(TokenKind::Id("type reference".to_string()));
    let type_reference_token = tokens_cursor.prev();

    tokens_cursor.consume_token(TokenKind::Equals);

    // Binding initial expression
    let init_expr = parse_expr(tokens_cursor);

    if let (TokenKind::Id(id), TokenKind::Id(type_reference)) = (id_token_kind, &type_reference_token.kind) {
        Expr {
            kind: ExprKind::Let {
                id: id.to_string(),
                type_reference: type_reference.to_string(),
                init_expr: Box::new(init_expr),
            },
            position: type_reference_token.position.clone(),
        }
    } else {
        internal_compiler_error("Unable to get id or type_reference. Should have been consumed.")
    }
}
