// Copyright © 2022 Brandon Li. All rights reserved.

use colored::Colorize;
use std::fs;
use std::path::Path;

use std::process::exit;

extern crate colored;
extern crate lazy_static;
extern crate regex;

#[cfg(test)]
extern crate expect_test;

mod asm;
mod bootstrapper;
mod compiler;
mod error_messages;
mod ir;
mod parser;
mod tokenizer;

/// Information about the source Solis file, grouped together in a single struct to pass between stages of compilation.
pub struct File {
    pub name: String,
    pub contents: String,
}

/// Reads in the Solis file.
fn read_file(file_name: &String) -> File {
    File {
        name: file_name.to_string(),
        contents: fs::read_to_string(file_name).unwrap_or_else(|error| {
            println!("{}: no such file {file_name}. {error}", "Error".red().bold());
            exit(exitcode::DATAERR)
        }),
    }
}

fn main() {
    let file = read_file(&"./examples/example.sl".to_string());

    let program = parser::parser::parse(&file, tokenizer::tokenizer::tokenize(&file));

    bootstrapper::bootstrap(
        compiler::compiler::compile(ir::translator::translate_program(program)),
        Path::new("./build"),
        "example",
        true,
    );
}
