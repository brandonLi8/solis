// Copyright © 2022 Brandon Li. All rights reserved.

//! The parser for Solis programs. A parser turns tokens into an AST representation of the program. This is the second
//! stage of the front end of the compiler. The AST is a more "nested" representation compared to tokens, and will be
//! ideal for the code-gen phase. The AST structure is defined in ast.rs
//!
//! Internally, the Solis parser is a recursive decent parser. The formal grammar is described in solis_grammar.txt
//! It works by creating a bunch of mutually recursive functions -- one for each production rule. Each function does:
//!   1. making sure there exists a parse tree for the program (is syntactically valid) by "walking" to the next rule
//!   2. transforming the parse tree into the AST
//! The parser runs in O(n) time with respect to the size of the program, since the grammar is a LL(k) class grammar.

use parser::ast::*;
use parser::parse_expression::parse_expr;
use parser::tokenizer::*;
use std::mem::discriminant;
use utils;

/// A parse context contains information about what has been parsed so far. It is grouped together to pass between
/// every parse function.
pub struct ParseContext<'a> {
    /// The remaining tokens to be parsed, as a token slice. Parsing happens from left to right. At a given
    /// non-terminal, this represents where it should start parsing, as everything to the left has been parsed already.
    pub remaining_tokens: &'a [Token],

    /// This is the last token that was parsed (a terminal is text that appears in program) when passed into a parse
    /// function. This is used for useful compiler error messaging.
    pub last_token: Option<&'a Token>,

    /// The original Solis input file, for error messaging.
    pub file: &'a utils::File,
}

/// Main parser function, which returns a ast::Program.
/// * file: the original Solis file
/// * tokens: output from the tokenizer
pub fn parse(file: &utils::File, tokens: Vec<Token>) -> Program {
    // Create a parse context that is passed around throughout the entire parse process.
    let mut context = ParseContext { remaining_tokens: &tokens, last_token: None, file };
    let program = parse_program(&mut context);

    // In the case that there are some remaining tokens (potentially a expression that wasn't fully written),
    // we raise a code error.
    if context.remaining_tokens.len() != 0 {
        utils::compilation_error(
            &context.file,
            &context.remaining_tokens.last().unwrap().position,
            "Syntax Error: unexpected end of file",
        )
    }

    return program;
}

// Corresponds to <program> rule and parses into ast::Program.
fn parse_program(context: &mut ParseContext) -> Program {
    let exprs = parse_exprs(vec![], context);

    return Program { body: Expr::Do { exprs } };
}

// Corresponds to <terminal> rule and parses into ast::Id, ast::Int, etc.
pub fn parse_terminal(context: &mut ParseContext) -> Expr {
    match context.remaining_tokens {
        [token, remaining_tokens @ ..] => {
            context.last_token = Some(token);
            context.remaining_tokens = remaining_tokens;

            match &token.kind {
                TokenKind::Id(id) => Expr::Id { value: id.to_string() },
                TokenKind::Int(int) => Expr::Int { value: *int },

                _ => utils::compilation_error(context.file, &token.position, "Syntax Error: unexpected token"),
            }
        }
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

// Corresponds to <exprs> rule and parses into ast::Expr list.
// * previous_exprs: in order to inexpensively parse expressions and add them to a result vector, recursively.
fn parse_exprs(mut previous_exprs: Vec<Expr>, context: &mut ParseContext) -> Vec<Expr> {
    match context.remaining_tokens {
        [] => previous_exprs,
        _ => {
            let next_expr = parse_expr(context);
            previous_exprs.push(next_expr);
            parse_exprs(previous_exprs, context)
        }
    }
}

/// Ensures that the first of remaining_tokens has kind expected_token_kind. If it is, it advances the remaining_tokens
/// and returns the token. Otherwise, it raises a compiler error.
///
/// NOTE: this works for TokenKinds that have data in them, because this checks that the TokenKind *variant* matches.
/// The actual data within the variant does not have to be equal, and the data within expected_token_kind is used for
/// further error messaging help.
pub fn consume_token(expected_token_kind: TokenKind, context: &mut ParseContext) {
    if context.remaining_tokens.len() == 0 {
        utils::compilation_error(
            &context.file,
            &context
                .last_token
                .unwrap_or_else(|| utils::internal_compiler_error("EOF but no last_token"))
                .position,
            "Syntax Error: unexpected end of file",
        )
    }

    // See https://stackoverflow.com/questions/32554285/compare-enums-only-by-variant-not-value
    if discriminant(&expected_token_kind) != discriminant(&context.remaining_tokens[0].kind) {
        utils::compilation_error(
            &context.file,
            &context
                .last_token
                .unwrap_or_else(|| utils::internal_compiler_error("EOF but no last_token"))
                .position,
            &format!("Syntax Error: expected {:?}", expected_token_kind),
        )
    }
    context.last_token = Some(&context.remaining_tokens[0]);
    context.remaining_tokens = &context.remaining_tokens[1..];
}
