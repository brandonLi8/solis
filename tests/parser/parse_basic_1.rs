// Copyright © 2022 Brandon Li. All rights reserved.

//! Basic tests for the parser.

use expect_test::expect;
use test_utils::parse_check;

#[test]
fn test_empty() {
    parse_check(
        "",
        expect![[r#"
            Program {
                body: Block {
                    exprs: [],
                },
            }"#]],
    )
}

#[test]
fn test_basic() {
    parse_check(
        "let varName: int = 32",
        expect![[r#"
            Program {
                body: Block {
                    exprs: [
                        Let {
                            id: "varName",
                            type_reference: "int",
                            init_expr: Int {
                                value: 32,
                            },
                        },
                    ],
                },
            }"#]],
    )
}
