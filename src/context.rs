// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Represents the context for compiling a source file. It owns all of its contents, and it's lifetime lasts
//! for the duration of the compiler. It is passed throughout various stages of the compiler.

use colored::Colorize;
use exit;
use std::fs;

pub struct Context {
    /// The input file_path to compile, as a owned String.
    pub file_path: String,

    /// The contents of the file_path, as a owned String.
    pub file: String,
}

impl Context {
    /// Creates a new Context from a file_path.
    pub fn from_file_path(file_path: String) -> Self {
        let file = fs::read_to_string(&file_path).unwrap_or_else(|error| {
            println!("{}: no such file {file_path}. {error}", "Error".red().bold());
            exit(exitcode::DATAERR)
        });

        Context { file_path, file }
    }
}
