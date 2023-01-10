// Copyright © 2022 Brandon Li. All rights reserved.

//! The parsing module contains all the modules necessary for parsing an input program into an AST.

pub mod ast;
pub mod parse_expression;
pub mod parse_infix;
pub mod parser;
pub mod tokenizer;

// tests
#[cfg(test)]
#[path = "."]
mod tests {
    mod parser_tests;
    mod tokenizer_tests;
}
