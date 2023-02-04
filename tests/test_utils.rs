// Copyright © 2022 Brandon Li. All rights reserved.

//! Utility functions used in the testing module.

extern crate expect_test;
extern crate solis;

use expect_test::Expect;
use solis::asm::asm::Register;
use solis::ir::translator::translate_program;
use solis::parser::parser::parse;
use solis::register_allocation::conflict_analysis::conflict_analysis;
use solis::register_allocation::liveness_analysis::liveness_analysis;
use solis::register_allocation::register_allocator::allocate_registers;
use solis::register_allocation::register_allocator::{Map, Set};
use solis::tokenizer::tokenizer::tokenize;
use solis::File;

/// Test function for tokenizing a program.
pub fn tokenize_check(program: &str, expect: Expect) {
    let tokens = tokenize(&File { name: String::new(), contents: program.to_string() });
    expect.assert_eq(
        &tokens
            .iter()
            .fold(String::new(), |acc, token| acc + &format!("{token:?}") + "\n"),
    )
}

/// Test function for parsing a program.
pub fn parse_check(program: &str, expect: Expect) {
    let file = File { name: String::new(), contents: program.to_string() };

    expect.assert_eq(&format!("{:#?}", parse(&file, tokenize(&file))))
}

/// Test function for translating a program.
pub fn translate_check(program: &str, expect: Expect) {
    let file = File { name: String::new(), contents: program.to_string() };

    expect.assert_eq(&format!("{:#?}", translate_program(parse(&file, tokenize(&file)))))
}

/// Test function for liveness analysis of an expression.
pub fn liveness_analysis_check(
    expr: &str,
    live_variables: Set<&String>,
    variable_frequencies: Map<&String, usize>,
    expect_live_variables: Expect,
    expect_frequencies: Expect,
) {
    let file = File { name: String::new(), contents: expr.to_string() };
    let program = translate_program(parse(&file, tokenize(&file)));

    let mut live_variables = live_variables.clone();
    let mut variable_frequencies = variable_frequencies.clone();
    liveness_analysis(
        program.body.exprs.get(0).unwrap(),
        &mut live_variables,
        &mut variable_frequencies,
    );

    expect_live_variables.assert_eq(&format!("{live_variables:?}"));
    expect_frequencies.assert_eq(&format!("{variable_frequencies:?}"));
}

/// Test function for conflict analysis of a block.
pub fn conflict_analysis_check(block: &str, expect: Expect) {
    let file = File { name: String::new(), contents: block.to_string() };
    let program = translate_program(parse(&file, tokenize(&file)));

    expect.assert_eq(&format!("{:#?}", conflict_analysis(&program.body)));
}

/// Test function for conflict analysis of a block.
pub fn register_allocator_check(block: &str, registers: Set<&Register>, expect: Expect) {
    let file = File { name: String::new(), contents: block.to_string() };
    let program = translate_program(parse(&file, tokenize(&file)));

    expect.assert_eq(&format!("{:#?}", allocate_registers(&program.body, registers)));
}
