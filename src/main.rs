// Copyright © 2022 Brandon Li. All rights reserved.

extern crate colored;
extern crate lazy_static;
extern crate regex;

use std::fs;

mod parser;
mod utils;

fn main() {
    let contents = fs::read_to_string("./examples/example.sl")
        .expect("Should have been able to read the file");

    println!(
        "{:?}",
        parser::parser::parse_program(&parser::tokenizer::tokenize(contents))
    )
}
