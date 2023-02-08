// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Basic tests for conflict analysis correctness.

use expect_test::expect;
use test_utils::conflict_analysis_check;

#[test]
fn test_empty() {
    conflict_analysis_check(
        "",
        expect![[r#"
            (
                InterferenceGraph {
                    nodes: {},
                    removed_nodes: {},
                },
                {},
            )"#]],
    );
}

#[test]
fn test_no_variables() {
    conflict_analysis_check(
        "1 + 2
         1 - 2
         (1 + 2)
         1 < 3
         1 / 2
         1 >= 2
         1 % 2",
        expect![[r#"
            (
                InterferenceGraph {
                    nodes: {},
                    removed_nodes: {},
                },
                {},
            )"#]],
    );
}

#[test]
fn test_only_temporaries() {
    conflict_analysis_check(
        "1 + 2 + 3  # @temp0 = 1 + 2; @temp0 + 1
         1 < 2 < 3  # @temp1 = 1 < 2; @temp1 + 1",
        expect![[r#"
            (
                InterferenceGraph {
                    nodes: {
                        "@temp0": {},
                        "@temp1": {},
                    },
                    removed_nodes: {},
                },
                {
                    "@temp0": 1,
                    "@temp1": 1,
                },
            )"#]],
    );
}

#[test]
fn test_let_sequence_1() {
    conflict_analysis_check(
        "let a: int = 1 + 2
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
         ##",
        expect![[r#"
            (
                InterferenceGraph {
                    nodes: {
                        "@temp0": {
                            "a",
                            "b",
                        },
                        "@temp1": {
                            "a",
                        },
                        "a": {
                            "@temp0",
                            "@temp1",
                            "b",
                            "c",
                        },
                        "b": {
                            "@temp0",
                            "a",
                        },
                        "c": {
                            "a",
                        },
                        "d": {},
                    },
                    removed_nodes: {},
                },
                {
                    "@temp0": 1,
                    "@temp1": 1,
                    "a": 2,
                    "b": 1,
                    "c": 1,
                    "d": 0,
                },
            )"#]],
    );
}

#[test]
fn test_let_sequence_2() {
    conflict_analysis_check(
        "let a: int = 1 + 2 * 3 * (2 + 3)
         let b: int = -2 < --+-4 * 1
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
         ##",
        expect![[r#"
            (
                InterferenceGraph {
                    nodes: {
                        "@temp0": {
                            "@temp1",
                        },
                        "@temp1": {
                            "@temp0",
                        },
                        "@temp2": {},
                        "@temp3": {
                            "@temp4",
                            "@temp5",
                            "@temp6",
                            "@temp7",
                            "a",
                        },
                        "@temp4": {
                            "@temp3",
                            "a",
                        },
                        "@temp5": {
                            "@temp3",
                            "a",
                        },
                        "@temp6": {
                            "@temp3",
                            "a",
                        },
                        "@temp7": {
                            "@temp3",
                            "a",
                        },
                        "a": {
                            "@temp3",
                            "@temp4",
                            "@temp5",
                            "@temp6",
                            "@temp7",
                            "b",
                            "c",
                        },
                        "b": {
                            "a",
                            "c",
                        },
                        "c": {
                            "a",
                            "b",
                        },
                    },
                    removed_nodes: {},
                },
                {
                    "@temp0": 1,
                    "@temp1": 1,
                    "@temp2": 1,
                    "@temp3": 1,
                    "@temp4": 1,
                    "@temp5": 1,
                    "@temp6": 1,
                    "@temp7": 1,
                    "a": 1,
                    "b": 1,
                    "c": 1,
                },
            )"#]],
    );
}
