// Copyright © 2022 Brandon Li. All rights reserved.

//! Main starting point driver for the Solis compiler. Responsible for parsing command line arguments and starting
//! the compiler process.

extern crate clap;
extern crate colored;
extern crate lazy_static;
extern crate regex;

pub mod asm;
pub mod bootstrapper;
pub mod compiler;
pub mod error_messages;
pub mod ir;
pub mod parser;
pub mod register_allocation;
pub mod tokenizer;

use clap::Parser;
use colored::Colorize;

use std::fs;
use std::path::Path;
use std::process::exit;

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

// Describes the schema for the CLI for Solis.
#[derive(Parser)]
#[command(author = "Brandon Li <brandon.li@berkeley.edu>", version)]
struct CLIArgs {
    /// The input Solis file to compile
    file: String,

    /// Name of the executable. [default: file name of FILE]
    #[arg(short, long)]
    name: Option<String>,

    /// Output directory, including artifacts.
    #[arg(short = 'd', long = "dest", default_value = "build")]
    destination: String,

    /// Use to immediately run executable after compiling.
    #[arg(short, long)]
    run: bool,

    /// Use to remove the contents in DESTINATION before compiling.
    #[arg(short, long)]
    clean: bool,
}

pub fn main() {
    let args = CLIArgs::parse();
    let file_name = args.file;
    let destination = Path::new(&args.destination);
    let file = read_file(&file_name);
    let name = args
        .name
        .unwrap_or_else(|| Path::new(&file_name).file_stem().unwrap().to_str().unwrap().to_string());

    let tokens = tokenizer::tokenizer::tokenize(&file);
    let program_ast = parser::parser::parse(&file, tokens);
    let program_ir = ir::translator::translate_program(program_ast);

    println!(
        "{:#?}",
        register_allocation::conflict_analysis::conflict_analysis(&program_ir.body)
    );

    let instructions = compiler::compiler::compile(program_ir);

    bootstrapper::bootstrap(instructions, destination, &name, args.run, args.clean);
}
