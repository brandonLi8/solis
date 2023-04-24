// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Utility functions used in the testing module.

extern crate expect_test;
extern crate solis;

use expect_test::{expect, Expect};
use solis::tokenizer::tokenizer::{find_next_token, tokenize};
// use solis::asm::asm::{FloatRegister, Register};
// use solis::ir::ir::Type;
// use solis::ir::translator::translate_program;
use solis::parser::parser::parse;
// use solis::register_allocation::conflict_analysis::{conflict_analysis, InterferenceGraph};
// use solis::register_allocation::liveness_analysis::liveness_analysis;
// use solis::register_allocation::register_allocator::{allocate_registers, Map, Set};
use solis::utils::context::Context;

/// Tests tokenizer output on program.
pub fn tokenize_check(program: &str, expect: Expect) {
    let context = Context { file_path: String::new(), file: program.to_string() };
    let mut tokens = String::new();
    let mut cursor = 0;

    while let Some((token, position)) = find_next_token(&context, &mut cursor) {
        tokens.push_str(&format!("Token `{token:?}` at {position:?}\n"));
    }

    expect.assert_eq(&tokens);
}

/// Tests tokenizer output on program, where a compilation error is expected.
pub fn tokenize_error_check(program: &str, expect: Expect) {
    expect_error(
        || {
            tokenize_check(program, expect![]);
        },
        expect,
    );
}

/// Tests parser output on program.
pub fn parse_check(program: &str, expect: Expect) {
    let context = Context { file_path: String::new(), file: program.to_string() };
    expect.assert_eq(&format!("{:#?}", parse(tokenize(&context))));
}

/// Tests parser output on program, where a compilation error is expected.
pub fn parse_error_check(program: &str, expect: Expect) {
    expect_error(
        || {
            parse_check(program, expect![]);
        },
        expect,
    );
}

// /// Test translator output program.
// pub fn translate_check(program: &str, expect: Expect) {
//     let file = Context { file_path: String::new(), file: program.to_string() };

//     expect.assert_eq(&format!(
//         "{:#?}",
//         translate_program(&file, parse(&file, tokenize(&file)))
//     ));
// }

// /// Tests translator output on program, where a compilation error is expected.
// pub fn translate_error_check(program: &str, expect: Expect) {
//     expect_error(
//         || {
//             translate_check(program, expect![]);
//         },
//         expect,
//     );
// }

// /// Test function for liveness analysis of an expression (runs it on the last expression of the block passed in).
// pub fn liveness_analysis_check(
//     block: &str,
//     live_variables: Map<&String, &Type>,
//     variable_frequencies: Map<&String, usize>,
//     expect_live_variables: Expect,
//     expect_frequencies: Expect,
// ) {
//     let file = Context { file_path: String::new(), file: block.to_string() };
//     let program = translate_program(&file, parse(&file, tokenize(&file)));

//     let mut live_variables = live_variables.clone();
//     let mut variable_frequencies = variable_frequencies.clone();
//     liveness_analysis(
//         program.body.exprs.last().unwrap(),
//         &mut live_variables,
//         &mut variable_frequencies,
//         &Set::new(),
//         &mut InterferenceGraph::new(),
//         &mut InterferenceGraph::new(),
//     );

//     expect_live_variables.assert_eq(&format!("{live_variables:?}"));
//     expect_frequencies.assert_eq(&format!("{variable_frequencies:?}"));
// }

// /// Test function for conflict analysis of a block.
// pub fn conflict_analysis_check(block: &str, expect: Expect) {
//     let file = Context { file_path: String::new(), file: block.to_string() };
//     let program = translate_program(&file, parse(&file, tokenize(&file)));

//     expect.assert_eq(&format!("{:#?}", conflict_analysis(&program.body, &Set::new())));
// }

// /// Test function for conflict analysis of a block, but the IR is formatted instead of the interference graph.
// /// This is typically done to test that Call sites are updated with the correct caller save information.
// pub fn conflict_analysis_ir_check(block: &str, expect: Expect) {
//     let file = Context { file_path: String::new(), file: block.to_string() };
//     let program = translate_program(&file, parse(&file, tokenize(&file)));
//     conflict_analysis(&program.body, &Set::new());

//     expect.assert_eq(&format!("{:#?}", program.body));
// }

// /// Test function for conflict analysis of a block.
// pub fn register_allocator_check(
//     block: &str,
//     registers: Set<&Register>,
//     float_registers: Set<&FloatRegister>,
//     expect: Expect,
// ) {
//     let file = Context { file_path: String::new(), file: block.to_string() };
//     let program = translate_program(&file, parse(&file, tokenize(&file)));

//     expect.assert_eq(&format!(
//         "{:#?}",
//         allocate_registers(&program.body, &Set::new(), registers, float_registers)
//     ));
// }

// Function the expects a panic message when calling `function`.
fn expect_error<F>(function: F, expect: expect_test::Expect)
where
    F: FnOnce() + std::panic::UnwindSafe,
{
    let error = std::panic::catch_unwind(function).unwrap_err();
    let message = error.downcast_ref::<String>().unwrap();
    expect.assert_eq(message);
}
