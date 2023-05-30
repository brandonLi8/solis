// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Tests translating function declarations that have errors.

use expect_test::expect;
use test_utils::translate_error_check;

#[test]
fn test_declaration_return_type_mismatch_1() {
    translate_error_check(
        "
        fun fib(n: int) : int {
          false
        }
        ",
        expect![[r#"
            Error: Mismatched return types, expected `int`, but found `bool`
             --> :2:12
              |
            2 |         fun fib(n: int) : int {
              |             ^^^
        "#]],
    );
}

#[test]
fn test_declaration_return_type_mismatch_2() {
    translate_error_check(
        "
        fun a(n: int) : bool {
          b(n)
        }

        fun b(n: int) : int {
          n
        }
        ",
        expect![[r#"
            Error: Mismatched return types, expected `bool`, but found `int`
             --> :2:12
              |
            2 |         fun a(n: int) : bool {
              |             ^
        "#]],
    );
}

#[test]
fn test_call_return_type_mismatch() {
    translate_error_check(
        "
        fun fib(n: int) : int {
          n
        }

        let a: bool = fib(2)
        ",
        expect![[r#"
            Error: Mismatched types, expected `bool`, but found `int`
             --> :6:22
              |
            6 |         let a: bool = fib(2)
              |                       ^^^^^^
        "#]],
    );
}

#[test]
fn test_call_bad_arity_1() {
    translate_error_check(
        "
        fun a(b: int, c: int, d: int) : int {
          b + c + d
        }

        fun main() : () {
          a(1, 2, 3, 4)
        }
        ",
        expect![[r#"
            Error: This function takes 3 arguments but 4 were supplied
             --> :7:10
              |
            7 |           a(1, 2, 3, 4)
              |           ^
        "#]],
    );
}

#[test]
fn test_call_bad_arity_2() {
    translate_error_check(
        "
        fun a(b: int, c: int, d: int) : int {
          b + c + d
        }

        fun main() : () {
          a(1, 2)
        }
        ",
        expect![[r#"
            Error: This function takes 3 arguments but 2 were supplied
             --> :7:10
              |
            7 |           a(1, 2)
              |           ^
        "#]],
    );
}

#[test]
fn test_declaration_param_type() {
    translate_error_check(
        "
        fun a(b: int) : bool {
          b
        }
        ",
        expect![[r#"
            Error: Mismatched return types, expected `bool`, but found `int`
             --> :2:12
              |
            2 |         fun a(b: int) : bool {
              |             ^
        "#]],
    );
}

#[test]
fn test_call_param_type() {
    translate_error_check(
        "
        fun a(b: int, c: bool, d: ()) : bool {
          c
        }

        a(1, 2
          + 4
        , 3)
        ",
        expect![[r#"
            Error: Expected argument type `bool`, found int
             --> :6:13
              |
            6 |         a(1, 2
              |              ^
            7 |           + 4
              | ^^^^^^^^^^^^^
        "#]],
    );
}

#[test]
fn test_declared_twice() {
    translate_error_check(
        "
        fun a(b: int, c: bool, d: ()) : bool {
          b
        }

        fun a(b: int, c: bool, d: ()) : bool {
          b
        }
        ",
        expect![[r#"
            Error: Function`a` has already been declared
             --> :6:12
              |
            6 |         fun a(b: int, c: bool, d: ()) : bool {
              |             ^
        "#]],
    );
}

#[test]
fn test_unknown_function() {
    translate_error_check(
        "
        let a: int = fib(2)
        ",
        expect![[r#"
            Error: Unknown function `fib`
             --> :2:21
              |
            2 |         let a: int = fib(2)
              |                      ^^^
        "#]],
    );
}
