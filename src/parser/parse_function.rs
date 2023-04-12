// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Responsible for parsing function declarations.

use error_messages::internal_compiler_error;
use parser::ast::{Block, Expr, ExprKind, Function, Param};
use parser::parse_expr::parse_expr;
use parser::parser::{parse_closed_block, parse_type};
use parser::tokens_cursor::TokensCursor;
use std::ops::Range;
use tokenizer::tokenizer::Token;

/// Corresponds to the `<functions>` rule and parses into `Vec<ast::Function>`
// * functions: previous functions that were parsed.
pub fn parse_functions(mut functions: Vec<Function>, tokens_cursor: &mut TokensCursor) -> Vec<Function> {
    let (next_token, tokens_cursor) = tokens_cursor.peek();

    if let Some((Token::Fun, _ )) = next_token {
        functions.push(parse_function(tokens_cursor));

        // Remove optional semicolons. See https://github.com/brandonLi8/solis/issues/28
        if let (Some((Token::Semi, _ )), _) = tokens_cursor.peek() {
            tokens_cursor.advance();
        }
        parse_functions(functions, tokens_cursor)
    } else {
        functions
    }
}

// Corresponds to the `<function>` rule and parses into `ast::Function`
fn parse_function(tokens_cursor: &mut TokensCursor) -> Function {
    tokens_cursor.consume_token(Token::Fun);

    // Consume the function id
    tokens_cursor.consume_token(Token::Id("identifier"));
    let (id_token, tokens_cursor) = tokens_cursor.prev();

    tokens_cursor.consume_token(Token::OpenParen);
    let params = parse_comma_separated_list::<Param>(vec![], parse_param, tokens_cursor);

    tokens_cursor.consume_token(Token::Colon);
    let return_type = parse_type(tokens_cursor);

    tokens_cursor.consume_token(Token::OpenBrace);
    let body = parse_closed_block(Block { exprs: vec![] }, tokens_cursor);

    if let Token::Id(id) = &id_token.0 {
        Function {
            params,
            return_type,
            body,
            id: id.to_string(),
            position: id_token.1.clone(),
        }
    } else {
        internal_compiler_error("Unable to get id. Should have been consumed.")
    }
}

// Corresponds to `<param>` rule and parses into `ast::Param`.
fn parse_param(tokens_cursor: &mut TokensCursor) -> Param {
    tokens_cursor.consume_token(Token::Id("identifier"));
    let ((param_id_kind, _), tokens_cursor) = tokens_cursor.prev();

    tokens_cursor.consume_token(Token::Colon);
    let type_reference = parse_type(tokens_cursor);

    if let Token::Id(id) = param_id_kind {
        Param { type_reference, id: id.to_string() }
    } else {
        internal_compiler_error("Unable to get id. Should have been consumed.")
    }
}

/// Corresponds to `<call>` rule and parses into `ast::Expr::Call`.
/// * id - the name of the function
pub fn parse_call(id: String, id_position: Range<usize>, tokens_cursor: &mut TokensCursor) -> Expr {
    Expr {
        kind: ExprKind::Call {
            id,
            args: parse_comma_separated_list::<Expr>(vec![], parse_expr, tokens_cursor),
        },
        position: id_position,
    }
}

// Corresponds to `<comma-separated-list>` rule and parses into `Vec<T>`.
// * list: previous items that were parsed.
// * parse_next - function that parses the next item
fn parse_comma_separated_list<T>(
    mut list: Vec<T>,
    parse_next: fn(&mut TokensCursor) -> T,
    tokens_cursor: &mut TokensCursor,
) -> Vec<T> {
    // Peek the next token
    let (next_token, tokens_cursor) = tokens_cursor.peek_unwrap();

    if let Token::CloseParen = &next_token.0 {
        tokens_cursor.advance();
        list
    } else {
        if !list.is_empty() {
            tokens_cursor.consume_token(Token::Comma);
        }
        list.push(parse_next(tokens_cursor));

        parse_comma_separated_list(list, parse_next, tokens_cursor)
    }
}
