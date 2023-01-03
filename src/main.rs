// Copyright © 2022 Brandon Li. All rights reserved.

extern crate colored;
extern crate lazy_static;
extern crate regex;

use std::fs;

#[macro_use]
mod utils;
mod parser;

fn main() {
    let contents = fs::read_to_string("./examples/example.sl")
        .expect("Should have been able to read the file");

    parser::tokenizer::tokenize(contents);
}

#[test]
#[should_panic]
fn it_works() {
    main()
}
