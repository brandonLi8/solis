// Copyright © 2022 Brandon Li. All rights reserved.

//! Basic tests for liveness analysis correctness.

use expect_test::expect;
use solis::register_allocation::register_allocator::{Map, Set};
use test_utils::liveness_analysis_check;

#[test]
fn test_empty_literals() {
    liveness_analysis_check(
        "let a: int = 1 + 2",
        Set::new(),
        Map::new(),
        expect!["{}"],
        expect![[r#"{"a": 0}"#]],
    );
}

#[test]
fn test_inherited_literals() {
    liveness_analysis_check(
        "let a: int = 1 + 2",
        Set::from([&String::from("a")]),
        Map::from([(&String::from("b"), 0_usize)]),
        expect!["{}"],
        expect![[r#"{"a": 0, "b": 0}"#]],
    );
}

#[test]
fn test_modify_inherited_literals() {
    liveness_analysis_check(
        "let a: int = 1 + 2",
        Set::from([&"a".to_string(), &"b".to_string()]),
        Map::from([(&"a".to_string(), 1_usize), (&"b".to_string(), 0_usize)]),
        expect![[r#"{"b"}"#]],
        expect![[r#"{"a": 1, "b": 0}"#]],
    );
}

#[test]
fn test_empty_ids() {
    liveness_analysis_check(
        "let a: int = d + 2",
        Set::new(),
        Map::new(),
        expect![[r#"{"d"}"#]],
        expect![[r#"{"a": 0, "d": 1}"#]],
    );
}

#[test]
fn test_modify_inherited_ids() {
    liveness_analysis_check(
        "let a: int = d + b",
        Set::from([&"a".to_string(), &"b".to_string()]),
        Map::from([(&"a".to_string(), 1_usize), (&"b".to_string(), 0_usize)]),
        expect![[r#"{"b", "d"}"#]],
        expect![[r#"{"a": 1, "b": 1, "d": 1}"#]],
    );
}

#[test]
fn test_keep_variable_alive() {
    liveness_analysis_check(
        "a + c",
        Set::from([&"a".to_string(), &"b".to_string()]),
        Map::from([(&"a".to_string(), 1_usize), (&"b".to_string(), 0_usize)]),
        expect![[r#"{"a", "b", "c"}"#]],
        expect![[r#"{"a": 2, "b": 0, "c": 1}"#]],
    );
}

#[test]
fn test_destroy_on_let() {
    liveness_analysis_check(
        "let a: int = b",
        Set::from([&"a".to_string(), &"b".to_string()]),
        Map::from([(&"a".to_string(), 1_usize), (&"b".to_string(), 0_usize)]),
        expect![[r#"{"b"}"#]],
        expect![[r#"{"a": 1, "b": 1}"#]],
    );
}
