// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Basic tests for register allocator correctness.

use expect_test::expect;
use solis::asm::asm::{FloatRegister, Register};
use solis::Set;
use test_utils::register_allocator_check;

#[test]
fn test_empty_program() {
    register_allocator_check("", Set::from([&Register::R8, &Register::R9]), Set::new(), expect!["{}"]);
}

#[test]
fn test_empty_registers() {
    register_allocator_check(
        "let a: int = 1 + 2
         let b: int = 6
         let c: int = 8 + a + b + 7
         let d: int = a + c
         let e: float = a + b + c + d + 0.0",
        Set::new(),
        Set::new(),
        expect![[r#"
            {
                "@temp0": Spill,
                "@temp1": Spill,
                "@temp2": Spill,
                "@temp3": Spill,
                "@temp4": Spill,
                "@temp5": Spill,
                "a": Spill,
                "b": Spill,
                "c": Spill,
                "d": Spill,
                "e": Spill,
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
        Set::from([&FloatRegister::Xmm1, &FloatRegister::Xmm2]),
        expect!["{}"],
    );
}

#[test]
fn test_no_conflicts_1() {
    register_allocator_check(
        "1 + 2 + 3       # @temp0 = 1 + 2; @temp0 + 1
         1 < 2 != false  # @temp1 = 1 < 2; @temp1 != false",
        Set::from([&Register::R8, &Register::R9, &Register::R10]),
        Set::new(),
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
         let d: bool = 6 + 7 < 9.",
        Set::from([&Register::R8, &Register::R9, &Register::R10]),
        Set::from([&FloatRegister::Xmm1, &FloatRegister::Xmm2]),
        expect![[r#"
            {
                "@temp0": FloatRegister(
                    Xmm1,
                ),
                "@temp1": Register(
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
