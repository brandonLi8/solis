// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Main starting point driver for the Solis compiler.

extern crate clap;
extern crate colored;
extern crate derive_more;
extern crate lazy_static;
extern crate regex;

// pub mod asm;
// pub mod compiler;
// pub mod ir_re;
pub mod parser;
// pub mod register_allocation;
pub mod tokenizer;
pub mod utils;

use clap::Parser;
use std::path::Path;
use std::process::exit;
use utils::cli_driver::CLIDriver;
use utils::context::Context;

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
    let _program_ast = parser::parser::parse(tokens);

    // let program_ir = ir::translator::translate_program(&file, program_ast);

    // let instructions = compiler::compiler::compile(program_ir);

    // bootstrapper::bootstrap(instructions, destination, &name, cli_args.run, cli_args.clean);
}
