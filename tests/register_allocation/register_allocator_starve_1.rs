// Copyright © 2022 Brandon Li. All rights reserved.

//! Tests for the register allocator, with different numbers of registers.

use expect_test::expect;
use solis::asm::asm::Register;
use solis::register_allocation::register_allocator::Set;
use test_utils::register_allocator_check;

static PROGRAM_1: &str = "# General purpose program
                          let a: int = 1 + 2
                          let b: int = 6
                          let c: int = 8 + a + b + 7
                          let d: int = a + c

                          ##
                          a = 1 + 2           {}
                          b = 6               {a}
                          temp0 = 8 + a       {a, b}
                          temp1 = temp0 + b   {temp0, b, a}
                          c = temp1 + 7       {a, temp1}
                          d = a + c           {a, c}
                          # Frequencies {d: 0, a: 2, b: 1, c: 1, temp0: 1, temp1: 1}
                          ##";

#[test]
fn test_program_1_starve_4() {
    register_allocator_check(
        PROGRAM_1,
        Set::from([&Register::R8, &Register::R9, &Register::R10, &Register::R11]),
        expect![[r#"
            {
                "@temp0": Register(
                    R10,
                ),
                "@temp1": Register(
                    R8,
                ),
                "a": Register(
                    R9,
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

#[test]
fn test_program_1_starve_3() {
    register_allocator_check(
        PROGRAM_1,
        Set::from([&Register::R8, &Register::R9, &Register::R10]),
        expect![[r#"
            {
                "@temp0": Register(
                    R10,
                ),
                "@temp1": Register(
                    R8,
                ),
                "a": Register(
                    R9,
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

#[test]
fn test_program_1_starve_2() {
    register_allocator_check(
        PROGRAM_1,
        Set::from([&Register::R8, &Register::R9]),
        expect![[r#"
            {
                "@temp0": Spill,
                "@temp1": Register(
                    R8,
                ),
                "a": Register(
                    R9,
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

#[test]
fn test_program_1_starve_1() {
    register_allocator_check(
        PROGRAM_1,
        Set::from([&Register::R8]),
        expect![[r#"
            {
                "@temp0": Spill,
                "@temp1": Spill,
                "a": Register(
                    R8,
                ),
                "b": Spill,
                "c": Spill,
                "d": Register(
                    R8,
                ),
            }"#]],
    );
}

#[test]
fn test_program_1_starve_0() {
    register_allocator_check(
        PROGRAM_1,
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
