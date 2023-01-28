// Copyright © 2022 Brandon Li. All rights reserved.

extern crate expect_test;
extern crate solis;

mod asm;
mod ir;
mod parser;
mod tokenizer;

/// Utility functions used in the testing module.
mod test_utils {
    use expect_test::Expect;
    use solis::ir::translator::translate_program;
    use solis::parser::parser::parse;
    use solis::tokenizer::tokenizer::tokenize;
    use solis::File;

    /// A helper function to test tokenizing a program, where the filename does not matter and only the contents matter.
    pub fn tokenize_check(program: &str, expect: Expect) {
        let tokens = tokenize(&File { name: String::new(), contents: program.to_string() });
        expect.assert_eq(
            &tokens
                .iter()
                .fold(String::new(), |acc, token| acc + &format!("{token:?}") + "\n"),
        )
    }

    /// A helper function to test parsing a program, where the filename does not matter and only the contents matter.
    pub fn parse_check(program: &str, expect: Expect) {
        let file = File { name: String::new(), contents: program.to_string() };

        expect.assert_eq(&format!("{:#?}", parse(&file, tokenize(&file))))
    }

    /// A helper function to test translating a program, where the filename does not matter and only the contents matter.
    pub fn translate_check(program: &str, expect: Expect) {
        let file = File { name: String::new(), contents: program.to_string() };

        expect.assert_eq(&format!("{:#?}", translate_program(parse(&file, tokenize(&file)))))
    }
}
