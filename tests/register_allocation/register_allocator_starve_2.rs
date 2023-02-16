// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Tests for the register allocator, with different numbers of registers.

use expect_test::expect;
use solis::asm::asm::Register;
use solis::register_allocation::register_allocator::Set;
use test_utils::register_allocator_check;

static PROGRAM_2: &str = "# Program with similar frequencies, but complex conflict graph
                          let a: int = 1 + 2 * 3 * (2 + 3)
                          let b: bool = -2 < --+-4 * 1
                          let c: int = 34
                          a + c
                          2
                          3
                          b

                          ##
                          temp0 = 2 * 3              {}
                          temp1 = 2 + 3              {temp0}
                          temp2 = temp0 * temp1      {temp0, temp1}
                          a = 1 + temp2              {temp2}
                          temp3 = -2                 {a}
                          temp4 = -4                 {a, temp3}
                          temp5 = -temp4             {a, temp3, temp4}
                          temp6 = -temp5             {a, temp3, temp5}
                          temp7 = temp6 * 1          {a, temp3, temp6}
                          b = temp3 < temp7          {a, temp3, temp7}
                          c = 34                     {a, b}
                          a + c                      {a, b, c}
                          2                          {b}
                          3                          {b}
                          b                          {b}

                           Frequencies {
                             @temp0: 1, @temp1: 1, @temp2: 1, @temp3: 1, @temp4: 1, @temp5: 1, @temp6: 1, @temp7: 1,
                             a: 1, b: 1, c: 1
                           }
                          ##";

#[test]
fn test_program_2_starve_4() {
    register_allocator_check(
        PROGRAM_2,
        Set::from([&Register::R8, &Register::R9, &Register::R10, &Register::R11]),
        expect![[r#"
            {
                "@temp0": Register(
                    R9,
                ),
                "@temp1": Register(
                    R8,
                ),
                "@temp2": Register(
                    R8,
                ),
                "@temp3": Register(
                    R8,
                ),
                "@temp4": Register(
                    R8,
                ),
                "@temp5": Register(
                    R8,
                ),
                "@temp6": Register(
                    R10,
                ),
                "@temp7": Register(
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
            }"#]],
    );
}

#[test]
fn test_program_2_starve_3() {
    register_allocator_check(
        PROGRAM_2,
        Set::from([&Register::R8, &Register::R9, &Register::R10]),
        expect![[r#"
            {
                "@temp0": Register(
                    R9,
                ),
                "@temp1": Register(
                    R8,
                ),
                "@temp2": Register(
                    R8,
                ),
                "@temp3": Register(
                    R8,
                ),
                "@temp4": Register(
                    R8,
                ),
                "@temp5": Register(
                    R8,
                ),
                "@temp6": Register(
                    R10,
                ),
                "@temp7": Register(
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
            }"#]],
    );
}

#[test]
fn test_program_2_starve_2() {
    register_allocator_check(
        PROGRAM_2,
        Set::from([&Register::R8, &Register::R9]),
        expect![[r#"
            {
                "@temp0": Register(
                    R9,
                ),
                "@temp1": Register(
                    R8,
                ),
                "@temp2": Register(
                    R8,
                ),
                "@temp3": Register(
                    R9,
                ),
                "@temp4": Register(
                    R9,
                ),
                "@temp5": Spill,
                "@temp6": Spill,
                "@temp7": Register(
                    R9,
                ),
                "a": Register(
                    R8,
                ),
                "b": Register(
                    R8,
                ),
                "c": Register(
                    R9,
                ),
            }"#]],
    );
}

#[test]
fn test_program_2_starve_1() {
    register_allocator_check(
        PROGRAM_2,
        Set::from([&Register::R8]),
        expect![[r#"
            {
                "@temp0": Spill,
                "@temp1": Register(
                    R8,
                ),
                "@temp2": Register(
                    R8,
                ),
                "@temp3": Spill,
                "@temp4": Spill,
                "@temp5": Spill,
                "@temp6": Spill,
                "@temp7": Spill,
                "a": Spill,
                "b": Register(
                    R8,
                ),
                "c": Register(
                    R8,
                ),
            }"#]],
    );
}

#[test]
fn test_program_2_starve_0() {
    register_allocator_check(
        PROGRAM_2,
        Set::new(),
        expect![[r#"
            {
                "@temp0": Spill,
                "@temp1": Spill,
                "@temp2": Spill,
                "@temp3": Spill,
                "@temp4": Spill,
                "@temp5": Spill,
                "@temp6": Spill,
                "@temp7": Spill,
                "a": Spill,
                "b": Spill,
                "c": Spill,
            }"#]],
    );
}
