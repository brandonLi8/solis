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
use parser::ast::{Block, Expr, ExprKind, Program};
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
    let block = parse_block(Block { exprs: vec![] }, tokens_cursor);

    Program { body: block }
}

// Corresponds to <terminal> rule and parses into ast::Id, ast::Int, etc.
pub fn parse_terminal(tokens_cursor: &mut TokensCursor) -> Expr {
    let (next_token, tokens_cursor) = tokens_cursor.next();
    match &next_token.kind {
        TokenKind::Id(id) => Expr {
            kind: ExprKind::Id { value: id.to_string() },
            position: next_token.position.clone(),
        },
        TokenKind::Int(int) => Expr {
            kind: ExprKind::Int { value: *int },
            position: next_token.position.clone(),
        },
        TokenKind::Bool(b) => Expr {
            kind: ExprKind::Bool { value: *b },
            position: next_token.position.clone(),
        },
        _ => compilation_error(
            tokens_cursor.file,
            &next_token.position,
            "Syntax Error: unexpected token",
        ),
    }
}

// Corresponds to <block> rule and parses into ast::Block.
// * block: in order to inexpensively parse expressions and add them to a result block, recursively.
fn parse_block(mut block: Block, tokens_cursor: &mut TokensCursor) -> Block {
    if tokens_cursor.is_end_of_file() {
        block
    } else {
        let next_expr = parse_expr(tokens_cursor);
        block.exprs.push(next_expr);
        parse_block(block, tokens_cursor)
    }
}
