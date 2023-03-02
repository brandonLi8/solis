// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Basic tests for liveness analysis correctness.

use expect_test::expect;
use solis::ir::type_checker::SolisType;
use solis::register_allocation::register_allocator::Map;
use test_utils::liveness_analysis_check;

#[test]
fn test_empty_literals() {
    liveness_analysis_check(
        "let a: int = 1 + 2",
        Map::new(),
        Map::new(),
        expect![[r#"{"a": Int}"#]],
        expect![[r#"{"a": 1}"#]],
    );
}

#[test]
fn test_inherited_literals() {
    liveness_analysis_check(
        "let a: int = 1 + 2",
        Map::from([(&"a".to_string(), &SolisType::Int)]),
        Map::from([(&"b".to_string(), 0_usize)]),
        expect![[r#"{"a": Int}"#]],
        expect![[r#"{"a": 1, "b": 0}"#]],
    );
}

#[test]
fn test_modify_inherited_literals() {
    liveness_analysis_check(
        "let a: int = 1 + 2",
        Map::from([(&"a".to_string(), &SolisType::Int), (&"b".to_string(), &SolisType::Int)]),
        Map::from([(&"a".to_string(), 1_usize), (&"b".to_string(), 0_usize)]),
        expect![[r#"{"a": Int, "b": Int}"#]],
        expect![[r#"{"a": 2, "b": 0}"#]],
    );
}

#[test]
fn test_empty_ids() {
    liveness_analysis_check(
        "let d: int = -1;
         let a: int = d + 2",
        Map::new(),
        Map::new(),
        expect![[r#"{"a": Int}"#]],
        expect![[r#"{"a": 1}"#]],
    );
}

#[test]
fn test_modify_inherited_ids() {
    liveness_analysis_check(
        "let d: int = -1; let b: int = -1;
         let a: int = d + b",
        Map::from([(&"a".to_string(), &SolisType::Int), (&"b".to_string(), &SolisType::Int)]),
        Map::from([(&"a".to_string(), 1_usize), (&"b".to_string(), 0_usize)]),
        expect![[r#"{"a": Int, "b": Int}"#]],
        expect![[r#"{"a": 2, "b": 0}"#]],
    );
}

#[test]
fn test_keep_variable_alive() {
    liveness_analysis_check(
        "let a: int = -1; let c: int = -1;
         a + c",
        Map::from([(&"a".to_string(), &SolisType::Int), (&"b".to_string(), &SolisType::Int)]),
        Map::from([(&"a".to_string(), 1_usize), (&"b".to_string(), 0_usize)]),
        expect![[r#"{"a": Int, "b": Int, "c": Int}"#]],
        expect![[r#"{"a": 2, "b": 0, "c": 1}"#]],
    );
}

#[test]
fn test_destroy_on_let() {
    liveness_analysis_check(
        "let b: int = -1;
         let a: int = b",
        Map::from([(&"a".to_string(), &SolisType::Int), (&"b".to_string(), &SolisType::Int)]),
        Map::from([(&"a".to_string(), 1_usize), (&"b".to_string(), 0_usize)]),
        expect![[r#"{"a": Int, "b": Int}"#]],
        expect![[r#"{"a": 2, "b": 0}"#]],
    );
}
