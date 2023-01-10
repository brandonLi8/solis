// Copyright © 2022 Brandon Li. All rights reserved.

extern crate colored;
extern crate lazy_static;
extern crate regex;

#[cfg(test)]
extern crate expect_test;

mod parser;
mod utils;

fn main() {
    let file = utils::read_file(&"./examples/example.sl".to_string());

    println!("{:?}", parser::parser::parse(&file, parser::tokenizer::tokenize(&file)));
}
