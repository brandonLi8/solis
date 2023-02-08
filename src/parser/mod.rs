// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! The parsing module is responsible for parsing an input program into an AST.

pub mod ast;
pub mod parser;

mod parse_expr;
mod parse_infix;
mod tokens_cursor;
