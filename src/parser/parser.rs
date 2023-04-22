// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! The parser for Solis programs. A parser turns tokens into an AST representation of the program. This is the second
//! stage of the front end of the compiler. The AST is a more "nested" representation compared to tokens, and will be
//! ideal for the code-gen phase. The AST structure is defined in ast.rs
//!
//! Internally, the Solis parser is a recursive decent parser. The formal grammar is described in `solis_grammar.txt`
//! It works by creating a bunch of mutually recursive functions -- one for each production rule. Each function does:
//!   1. making sure there exists a parse tree for the program (is syntactically valid) by "walking" to the next rule
//!   2. transforming the parse tree into the AST
//!
//! The parser runs in O(n) time with respect to the size of the program, since the grammar is a LL(k) class grammar.

use error_messages::{compilation_error, internal_compiler_error, ErrorPosition};
use parser::ast::{Expr, Program, Type};
use parser::parse_function::parse_call;
use parser::parse_function::parse_functions;
use parser::parser_utils::{parse_block, ParseBlockStopMode};
use tokenizer::token_iterator::TokenIterator;
use tokenizer::tokenizer::Token;

/// Main parser function, which returns a `ast::Program`.
/// * tokens: output from the tokenizer
pub fn parse<'a>(tokens: TokenIterator<'a>) -> Program<'a> {
    let (program, tokens) = parse_program(tokens);

    // In the case that there are some remaining tokens.
    if tokens.peek().is_some() {
        internal_compiler_error("Leftover tokens in program")
    }

    program
}

// Corresponds to `<program>` rule and parses into a `ast::Program`.
fn parse_program<'a>(tokens: TokenIterator<'a>) -> (Program<'a>, TokenIterator<'a>) {
    let (functions, tokens) = parse_functions(tokens);
    let (body, tokens) = parse_block(ParseBlockStopMode::EndOfFile, tokens);

    (Program { body, functions }, tokens)
}

// Corresponds to `<terminal>` rule and parses into `ast::Id`, `ast::Int`, etc.
pub fn parse_terminal<'a>(mut tokens: TokenIterator<'a>) -> (Expr<'a>, TokenIterator<'a>) {
    let (next_token, next_token_position) = tokens.next_or_error();

    (
        match next_token {
            Token::Id(id) => {
                // Either Call or plain ID
                if let Some((Token::OpenParen, _)) = tokens.peek() {
                    tokens.advance();
                    return parse_call(id, tokens);
                } else {
                    Expr::Id { value: id }
                }
            }

            Token::Int(value) => Expr::Int { value },
            Token::Bool(value) => Expr::Bool { value },
            Token::Float(value) => Expr::Float { value },

            _ => compilation_error(
                tokens.context,
                ErrorPosition::Span(next_token_position.clone()),
                "Syntax Error: unexpected token",
            ),
        },
        tokens,
    )
}

/// Corresponds to `<type>` rule and parses into `ast::Type`.
pub fn parse_type(mut tokens: TokenIterator) -> (Type, TokenIterator) {
    let (next_token, next_token_position) = tokens.next_or_error();

    (
        match next_token {
            Token::Id("int") => Type::Int,
            Token::Id("bool") => Type::Bool,
            Token::Id("float") => Type::Float,
            Token::OpenParen => {
                tokens.consume_token(Token::CloseParen);
                Type::Unit
            }
            _ => compilation_error(
                tokens.context,
                ErrorPosition::Span(next_token_position.clone()),
                &format!("Invalid type: {next_token}"),
            ),
        },
        tokens,
    )
}
