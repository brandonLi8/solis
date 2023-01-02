// Copyright © 2022 Brandon Li. All rights reserved.

use std::fs;

mod parser;

fn main() {
    let contents = fs::read_to_string("./examples/example.sl")
        .expect("Should have been able to read the file");

    parser::tokenizer::tokenize(contents);
}
