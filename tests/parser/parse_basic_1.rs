// Copyright © 2022-2023 Brandon Li. All rights reserved.

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
        "let varName: int = 32
         let varName2: bool = true
         let varName3: float = 2.
        ",
        expect![[r#"
            Program {
                body: Block {
                    exprs: [
                        Expr {
                            kind: Let {
                                id: "varName",
                                type_reference: "int",
                                init_expr: Expr {
                                    kind: Int {
                                        value: 32,
                                    },
                                    position: 19..21,
                                },
                            },
                            position: 13..16,
                        },
                        Expr {
                            kind: Let {
                                id: "varName2",
                                type_reference: "bool",
                                init_expr: Expr {
                                    kind: Bool {
                                        value: true,
                                    },
                                    position: 52..56,
                                },
                            },
                            position: 45..49,
                        },
                        Expr {
                            kind: Let {
                                id: "varName3",
                                type_reference: "float",
                                init_expr: Expr {
                                    kind: Float {
                                        value: 2.0,
                                    },
                                    position: 88..90,
                                },
                            },
                            position: 80..85,
                        },
                    ],
                },
            }"#]],
    )
}
