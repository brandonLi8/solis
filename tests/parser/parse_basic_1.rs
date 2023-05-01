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
                functions: [],
                body: Block {
                    exprs: [],
                },
            }"#]],
    );
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
                functions: [],
                body: Block {
                    exprs: [
                        Let {
                            id: "varName",
                            id_position: 4..11,
                            type_reference: Int,
                            init_expr: Int {
                                value: 32,
                            },
                        },
                        Let {
                            id: "varName2",
                            id_position: 35..43,
                            type_reference: Bool,
                            init_expr: Bool {
                                value: true,
                            },
                        },
                        Let {
                            id: "varName3",
                            id_position: 70..78,
                            type_reference: Float,
                            init_expr: Float {
                                value: 2.0,
                            },
                        },
                    ],
                },
            }"#]],
    );
}
