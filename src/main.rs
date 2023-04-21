// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Main starting point driver for the Solis compiler.

extern crate clap;
extern crate colored;
extern crate derive_more;
extern crate lazy_static;
extern crate regex;

// pub mod asm;
// pub mod bootstrapper;
// pub mod compiler;
pub mod error_messages;
// pub mod ir;
pub mod parser_re;
// pub mod register_allocation;
pub mod cli_driver;
pub mod context;
pub mod tokenizer;

use clap::Parser;
use cli_driver::CLIDriver;
use context::Context;
use std::path::Path;
use std::process::exit;

pub fn main() {
    // Parse the command line arguments.
    let cli_args = CLIDriver::parse();

    // Move out the file_path.
    let file_path = cli_args.file;

    // Destination as a Path
    let _destination = Path::new(&cli_args.destination);

    // Get the name of the input file, without the extension.
    let _name = cli_args
        .name
        .unwrap_or_else(|| Path::new(&file_path).file_stem().unwrap().to_str().unwrap().to_string());

    // Create the compilation Context,
    let context = Context::from_file_path(file_path);

    let tokens = tokenizer::tokenizer::tokenize(&context);
    dbg!(parser_re::parser::parse(tokens));

    // let program_ast = parser::parser::parse(&file, tokens);

    // let program_ir = ir::translator::translate_program(&file, program_ast);

    // let instructions = compiler::compiler::compile(program_ir);

    // bootstrapper::bootstrap(instructions, destination, &name, cli_args.run, cli_args.clean);
}
