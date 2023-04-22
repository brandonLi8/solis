// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Responsible for parsing function declarations.

use error_messages::internal_compiler_error;
use parser::ast::{Expr, Function, Param};
use parser::parse_expr::parse_expr;
use parser::parser::parse_type;
use parser::parser_utils::{parse_block, parse_comma_separated_list, ParseBlockStopMode};
use tokenizer::token_iterator::TokenIterator;
use tokenizer::tokenizer::Token;

/// Corresponds to the `<functions>` rule and parses into a `Vec<ast::Function>`
///
/// Note: the implementation of this doesn't match the rule definition in `.solis_grammar.txt`. Instead of recursively
/// filling the expressions of the block, we iteratively fill the block.
pub fn parse_functions(mut tokens: TokenIterator) -> (Vec<Function>, TokenIterator) {
    let mut functions = vec![];
    while matches!(tokens.peek(), Some((Token::Fun, _))) {
        let (function, next_tokens) = parse_function(tokens);
        functions.push(function);

        // The compiler is not smart enough to figure out that we are reassigning the moved tokens, so we use
        // an intermediate `next_tokens` to explicitly do it.
        tokens = next_tokens;

        // Remove optional semicolons. See https://github.com/brandonLi8/solis/issues/28
        if let Some((Token::Semi, _)) = tokens.peek() {
            tokens.advance();
        }
    }

    (functions, tokens)
}

/// Corresponds to the `<function>` rule and parses into a `ast::Function`
fn parse_function(mut tokens: TokenIterator) -> (Function, TokenIterator) {
    tokens.consume_token(Token::Fun);

    // Consume the function id
    let (id_token, _) = tokens.consume_token(Token::Id("identifier"));

    tokens.consume_token(Token::OpenParen);

    let (params, mut tokens) = parse_comma_separated_list::<Param>(parse_param, tokens);

    tokens.consume_token(Token::Colon);
    let (return_type, mut tokens) = parse_type(tokens);

    tokens.consume_token(Token::OpenBrace);
    let (body, tokens) = parse_block(ParseBlockStopMode::Brace, tokens);

    (
        Function {
            params,
            return_type,
            body,
            id: if let Token::Id(id) = id_token { id } else { internal_compiler_error("id not Token::Id variant") },
        },
        tokens,
    )
}

/// Corresponds to `<param>` rule and parses into a `ast::Param`.
fn parse_param(mut tokens: TokenIterator) -> (Param, TokenIterator) {
    let (param_id_token, _) = tokens.consume_token(Token::Id("identifier"));

    tokens.consume_token(Token::Colon);
    let (type_reference, tokens) = parse_type(tokens);

    (
        Param {
            type_reference,
            id: if let Token::Id(id) = param_id_token {
                id
            } else {
                internal_compiler_error("id not Token::Id variant")
            },
        },
        tokens,
    )
}

/// Corresponds to `<call>` rule and parses into a `ast::Expr::Call`.
/// * id - the name of the function
pub fn parse_call<'a>(id: &'a str, tokens: TokenIterator<'a>) -> (Expr<'a>, TokenIterator<'a>) {
    let (args, tokens) = parse_comma_separated_list::<Expr>(parse_expr, tokens);

    (Expr::Call { id, args }, tokens)
}
