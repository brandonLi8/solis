// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Basic tests for register allocator correctness.

use expect_test::expect;
use solis::asm::asm::Register;
use solis::register_allocation::register_allocator::Set;
use test_utils::register_allocator_check;

#[test]
fn test_empty_program() {
    register_allocator_check("", Set::from([&Register::R8, &Register::R9]), expect!["{}"]);
}

#[test]
fn test_empty_registers() {
    register_allocator_check(
        "let a: int = 1 + 2
         let b: int = 6
         let c: int = 8 + a + b + 7
         let d: int = a + c",
        Set::new(),
        expect![[r#"
            {
                "@temp0": Spill,
                "@temp1": Spill,
                "a": Spill,
                "b": Spill,
                "c": Spill,
                "d": Spill,
            }"#]],
    );
}

#[test]
fn test_no_variables() {
    register_allocator_check(
        "1 + 2
         1 - 2
         (1 + 2)
         1 < 3
         1 / 2
         1 >= 2
         1 % 2",
        Set::from([&Register::R8, &Register::R9]),
        expect!["{}"],
    );
}

#[test]
fn test_no_conflicts_1() {
    register_allocator_check(
        "1 + 2 + 3  # @temp0 = 1 + 2; @temp0 + 1
         1 < 2 < 3  # @temp1 = 1 < 2; @temp1 + 1",
        Set::from([&Register::R8, &Register::R9, &Register::R10]),
        expect![[r#"
            {
                "@temp0": Register(
                    R8,
                ),
                "@temp1": Register(
                    R8,
                ),
            }"#]],
    );
}

#[test]
fn test_no_conflicts_2() {
    register_allocator_check(
        "let a: int = 1 + 2
         let b: int = 2 + 3
         let c: int = 4 + 5
         let d: int = 6 + 7 < 9",
        Set::from([&Register::R8, &Register::R9, &Register::R10]),
        expect![[r#"
            {
                "@temp0": Register(
                    R8,
                ),
                "a": Register(
                    R8,
                ),
                "b": Register(
                    R8,
                ),
                "c": Register(
                    R8,
                ),
                "d": Register(
                    R8,
                ),
            }"#]],
    );
}
