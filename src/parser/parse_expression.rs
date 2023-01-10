// Copyright © 2022 Brandon Li. All rights reserved.

//! Defines the functions for parsing various types of expressions.

use parser::ast::*;
use parser::parse_infix::*;
use parser::parser::*;
use parser::tokenizer::*;
use utils;

// Corresponds to <expr> rule and parses into ast::Expr.
pub fn parse_expr(context: &mut ParseContext) -> Expr {
    match context.remaining_tokens {
        [token, ..] => match &token.kind {
            TokenKind::Let => parse_let_expr(context),
            _ => parse_infix_operation(context),
        },
        [] => utils::compilation_error(
            context.file,
            &context
                .last_token
                .unwrap_or_else(|| utils::internal_compiler_error("EOF but no last_token"))
                .position,
            "Syntax Error: unexpected end of file",
        ),
    }
}

// Corresponds to <let-expr> rule and parses into ast::Expr::Let.
pub fn parse_let_expr(context: &mut ParseContext) -> Expr {
    consume_token(TokenKind::Let, context);

    // Consume the let expression identifier
    consume_token(TokenKind::Id("any variable name".to_string()), context);
    let id_token_kind = &context.last_token.unwrap().kind;

    consume_token(TokenKind::Colon, context);

    // Consume the type reference identifier
    consume_token(TokenKind::Id("type reference".to_string()), context);
    let type_reference_token_kind = &context.last_token.unwrap().kind;

    consume_token(TokenKind::Equals, context);

    // Binding initial expression
    let init_expr = parse_expr(context);

    if let (TokenKind::Id(id), TokenKind::Id(type_reference)) = (id_token_kind, type_reference_token_kind) {
        Expr::Let {
            id: id.to_string(),
            type_reference: type_reference.to_string(),
            init_expr: Box::new(init_expr),
        }
    } else {
        utils::internal_compiler_error("Unable to get id or type_reference. Should have been consumed.")
    }
}
