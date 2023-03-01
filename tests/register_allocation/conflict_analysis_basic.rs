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
         1 / 2.2
         1 >= 2
         1 % 2",
        expect![[r#"
            (
                InterferenceGraph {
                    nodes: {},
                    removed_nodes: {},
                },
                InterferenceGraph {
                    nodes: {
                        "@temp0": {
                            "@temp1",
                        },
                        "@temp1": {
                            "@temp0",
                        },
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
fn test_only_temporaries() {
    conflict_analysis_check(
        "1 + 2 + 3      # @temp0 = 1 + 2; @temp0 + 1
         1 < 2 != true  # @temp1 = 1 < 2; @temp1 != true",
        expect![[r#"
            (
                InterferenceGraph {
                    nodes: {
                        "@temp0": {},
                        "@temp1": {},
                    },
                    removed_nodes: {},
                },
                InterferenceGraph {
                    nodes: {},
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
fn test_only_float_temporaries() {
    conflict_analysis_check(
        "1 + 2 + 3.0
        # @temp0 = 1 + 2;
        # @temp1 = 3.0;
        # @temp0 + temp1
        ",
        expect![[r#"
            (
                InterferenceGraph {
                    nodes: {
                        "@temp1": {},
                    },
                    removed_nodes: {},
                },
                InterferenceGraph {
                    nodes: {
                        "@temp0": {
                            "@temp2",
                        },
                        "@temp2": {
                            "@temp0",
                        },
                    },
                    removed_nodes: {},
                },
                {
                    "@temp0": 1,
                    "@temp1": 1,
                    "@temp2": 1,
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
                InterferenceGraph {
                    nodes: {},
                    removed_nodes: {},
                },
                {
                    "@temp0": 1,
                    "@temp1": 1,
                    "a": 2,
                    "b": 1,
                    "c": 1,
                    "d": 1,
                },
            )"#]],
    );
}

#[test]
fn test_let_sequence_2() {
    conflict_analysis_check(
        "let a: int = 1 + 2 * 3 * (2 + 3)
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
                            "a",
                        },
                        "@temp4": {
                            "a",
                        },
                        "@temp5": {
                            "@temp6",
                            "a",
                        },
                        "@temp6": {
                            "@temp5",
                            "@temp7",
                            "a",
                        },
                        "@temp7": {
                            "@temp6",
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
                InterferenceGraph {
                    nodes: {},
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

#[test]
fn test_let_sequence_3() {
    conflict_analysis_check(
        "
         let a: int = 1 + 2
         let b: int = 6
         let c: int = 8 + a + b + 7
         let d: int = a + c
         let e: float = a + b + c + d + 0.0

         ##
          a = 1 + 2           {}
          b = 6               {a}
          temp0 = 8 + a       {a, b}
          temp1 = temp0 + b   {a, b, temp0}
          c = temp1 + 7       {a, b, temp1}
          d = a + c           {a, b, c}
          temp2 = a + b       {a, b, c, d}
          temp3 = temp2 + c   {temp2, c, d}
          temp4 = 0.0         {temp3, d}
          temp5 = temp3 + d   {temp4, temp3, d}
          e = temp5 + temp4   {temp5, temp4}
          e                   {e}
          identifier_types: {
              d: Int,
              c: Int,
              @temp2: Int,
              @temp1: Int,
              @temp3: Int,
              @temp5: Int,
              @temp0: Int,
              a: Int,
              b: Int,
              @temp4: Float,
              e: Float,
          }
         ##
        ",
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
                            "b",
                        },
                        "@temp2": {
                            "c",
                            "d",
                        },
                        "@temp3": {
                            "d",
                        },
                        "@temp5": {},
                        "a": {
                            "@temp0",
                            "@temp1",
                            "b",
                            "c",
                            "d",
                        },
                        "b": {
                            "@temp0",
                            "@temp1",
                            "a",
                            "c",
                            "d",
                        },
                        "c": {
                            "@temp2",
                            "a",
                            "b",
                            "d",
                        },
                        "d": {
                            "@temp2",
                            "@temp3",
                            "a",
                            "b",
                            "c",
                        },
                    },
                    removed_nodes: {},
                },
                InterferenceGraph {
                    nodes: {
                        "@temp4": {
                            "@temp6",
                        },
                        "@temp6": {
                            "@temp4",
                        },
                        "e": {},
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
                    "a": 3,
                    "b": 2,
                    "c": 2,
                    "d": 1,
                    "e": 1,
                },
            )"#]],
    )
}

#[test]
fn test_let_sequence_4() {
    conflict_analysis_check(
        "
         let a: float = 1 + 2 + 3.
         let b: int = 34 / 2
         let c: float = a * b + 2
         let d: float = a + a + a + a + a + a + a + b + c
         ##
          @temp0 = 3.           {}
          @temp1 = 1 + 2        {temp1}
          a = temp0 + temp1     {temp0, temp1}

          b = 34 / 2            {a}
          @temp2 = a * b        {a, b}
          c = temp2 + 2         {temp2, a}
          @temp3 = a + a        {c, a}
          @temp4 = @temp3 + a   {temp3, c, a}
          @temp5 = @temp4 + a   {temp4, c, a}
          @temp6 = @temp5 + a   {temp5, c, a}
          @temp7 = @temp6 + a   {temp8, c, a}
          @temp8 = @temp7 + a   {temp7, c, a}
          @temp9 = @temp8 + b   {temp8, c, b}
          d = @temp9 + c        {temp9, c}
          d                     {d}

          identifier_types: {
            @temp1: Int,
            @temp8: Float,
            @temp2: Float,
            @temp4: Float,
            @temp3: Float,
            a: Float,
            @temp9: Float,
            c: Float,
            @temp5: Float,
            @temp6: Float,
            @temp0: Float,
            @temp7: Float,
            d: Float,
            b: Int,
        }
         ##",
        expect![[r#"
            (
                InterferenceGraph {
                    nodes: {
                        "@temp1": {},
                        "b": {},
                    },
                    removed_nodes: {},
                },
                InterferenceGraph {
                    nodes: {
                        "@temp0": {
                            "@temp2",
                        },
                        "@temp10": {
                            "a",
                            "c",
                        },
                        "@temp11": {
                            "@temp12",
                            "c",
                        },
                        "@temp12": {
                            "@temp11",
                            "c",
                        },
                        "@temp13": {
                            "c",
                        },
                        "@temp2": {
                            "@temp0",
                        },
                        "@temp3": {
                            "a",
                        },
                        "@temp4": {
                            "@temp5",
                            "a",
                        },
                        "@temp5": {
                            "@temp4",
                            "a",
                        },
                        "@temp6": {
                            "a",
                            "c",
                        },
                        "@temp7": {
                            "a",
                            "c",
                        },
                        "@temp8": {
                            "a",
                            "c",
                        },
                        "@temp9": {
                            "a",
                            "c",
                        },
                        "a": {
                            "@temp10",
                            "@temp3",
                            "@temp4",
                            "@temp5",
                            "@temp6",
                            "@temp7",
                            "@temp8",
                            "@temp9",
                            "c",
                        },
                        "c": {
                            "@temp10",
                            "@temp11",
                            "@temp12",
                            "@temp13",
                            "@temp6",
                            "@temp7",
                            "@temp8",
                            "@temp9",
                            "a",
                        },
                        "d": {},
                    },
                    removed_nodes: {},
                },
                {
                    "@temp0": 1,
                    "@temp1": 1,
                    "@temp10": 1,
                    "@temp11": 1,
                    "@temp12": 1,
                    "@temp13": 1,
                    "@temp2": 1,
                    "@temp3": 1,
                    "@temp4": 1,
                    "@temp5": 1,
                    "@temp6": 1,
                    "@temp7": 1,
                    "@temp8": 1,
                    "@temp9": 1,
                    "a": 8,
                    "b": 2,
                    "c": 1,
                    "d": 1,
                },
            )"#]],
    );
}
