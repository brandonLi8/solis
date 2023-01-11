// Copyright © 2022 Brandon Li. All rights reserved.

//! The parser for Solis programs. A parser turns tokens into an AST representation of the program. This is the second
//! stage of the front end of the compiler. The AST is a more "nested" representation compared to tokens, and will be
//! ideal for the code-gen phase. The AST structure is defined in ast.rs
//!
//! Internally, the Solis parser is a recursive decent parser. The formal grammar is described in `solis_grammar.txt`
//! It works by creating a bunch of mutually recursive functions -- one for each production rule. Each function does:
//!   1. making sure there exists a parse tree for the program (is syntactically valid) by "walking" to the next rule
//!   2. transforming the parse tree into the AST
//! The parser runs in O(n) time with respect to the size of the program, since the grammar is a LL(k) class grammar.

use error_messages::compilation_error;
use parser::ast::{Expr, Program};
use parser::parse_expr::parse_expr;
use parser::tokens_cursor::TokensCursor;
use tokenizer::tokenizer::{Token, TokenKind};
use File;

/// Main parser function, which returns a `ast::Program`.
/// * file: the original Solis file
/// * tokens: output from the tokenizer
pub fn parse(file: &File, tokens: Vec<Token>) -> Program {
    // Create a parse tokens_cursor that is passed around throughout the entire parse process.
    let mut tokens_cursor = TokensCursor::new(&tokens, file);
    let program = parse_program(&mut tokens_cursor);

    // In the case that there are some remaining tokens (potentially a expression that wasn't fully written),
    // we raise a compilation error. TODO: is this possible? should this be internal error instead?
    if !tokens_cursor.is_end_of_file() {
        compilation_error(
            tokens_cursor.file,
            &tokens.last().unwrap().position,
            "Syntax Error: unexpected end of file",
        )
    }

    program
}

// Corresponds to <program> rule and parses into ast::Program.
fn parse_program(tokens_cursor: &mut TokensCursor) -> Program {
    let exprs = parse_exprs(vec![], tokens_cursor);

    Program { body: Expr::Do { exprs } }
}

// Corresponds to <terminal> rule and parses into ast::Id, ast::Int, etc.
pub fn parse_terminal(tokens_cursor: &mut TokensCursor) -> Expr {
    let (next_token, tokens_cursor) = tokens_cursor.next();
    match &next_token.kind {
        TokenKind::Id(id) => Expr::Id { value: id.to_string() },
        TokenKind::Int(int) => Expr::Int { value: *int },
        _ => compilation_error(
            tokens_cursor.file,
            &next_token.position,
            "Syntax Error: unexpected token",
        ),
    }
}

// Corresponds to <exprs> rule and parses into ast::Expr list.
// * previous_exprs: in order to inexpensively parse expressions and add them to a result vector, recursively.
fn parse_exprs(mut previous_exprs: Vec<Expr>, tokens_cursor: &mut TokensCursor) -> Vec<Expr> {
    if tokens_cursor.is_end_of_file() {
        previous_exprs
    } else {
        let next_expr = parse_expr(tokens_cursor);
        previous_exprs.push(next_expr);
        parse_exprs(previous_exprs, tokens_cursor)
    }
}
