// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Tests for the register allocator, with different numbers of registers.

use expect_test::expect;
use solis::asm::asm::Register;
use solis::Set;
use test_utils::register_allocator_check;

static PROGRAM_3: &str = "# Program with different frequencies
                          let a: int = 1 + 2 + 3 + 4 + 5
                          let b: int = a * 3
                          a a a a a a a a a a a a a a a a a a a a a a
                          let c: int = 34
                          b b b b b b
                          c
                          let d: int = 4
                          ##
                          temp0 = 1 + 2        {}
                          temp1 = temp0 + 3    {temp0}
                          temp2 = temp1 + 4    {temp1}
                          a = temp2 + 5        {temp2}
                          b = a * 3            {a}
                          a a a a a...         {a, b}
                          c = 35               {b}
                          b b b b b b          {b, c}
                          c                    {c}
                          d = 4                {}

                           frequencies {
                             @temp0: 1, @temp1: 1, @temp2: 1,
                             a: high,
                             b: medium,
                             c: 2
                             d: 1
                           }
                          ##";

#[test]
fn test_program_3_starve_3() {
    register_allocator_check(
        PROGRAM_3,
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
                "@temp2": Register(
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

#[test]
fn test_program_3_starve_2() {
    register_allocator_check(
        PROGRAM_3,
        Set::from([&Register::R8, &Register::R9]),
        Set::new(),
        expect![[r#"
            {
                "@temp0": Register(
                    R8,
                ),
                "@temp1": Register(
                    R8,
                ),
                "@temp2": Register(
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

#[test]
fn test_program_3_starve_1() {
    register_allocator_check(
        PROGRAM_3,
        Set::from([&Register::R8]),
        Set::new(),
        expect![[r#"
            {
                "@temp0": Register(
                    R8,
                ),
                "@temp1": Register(
                    R8,
                ),
                "@temp2": Register(
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

#[test]
fn test_program_3_starve_0() {
    register_allocator_check(
        PROGRAM_3,
        Set::new(),
        Set::new(),
        expect![[r#"
            {
                "@temp0": Spill,
                "@temp1": Spill,
                "@temp2": Spill,
                "a": Spill,
                "b": Spill,
                "c": Spill,
                "d": Spill,
            }"#]],
    );
}
