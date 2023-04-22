// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Tests parser correctness for `https://github.com/brandonLi8/solis/issues/28`

use expect_test::expect;
use test_utils::parse_check;

#[test]
fn test_ambiguity_unary() {
    parse_check(
        r"
          let a: int = 1
          -2
        ",
        expect![[r#"
            Program {
                functions: [],
                body: Block {
                    exprs: [
                        Let {
                            id: "a",
                            type_reference: Int,
                            init_expr: BinaryExpr {
                                kind: Minus,
                                operand_1: Int {
                                    value: 1,
                                },
                                operand_2: Int {
                                    value: 2,
                                },
                            },
                        },
                    ],
                },
            }"#]],
    );

    parse_check(
        r"
          let a: int = 1;
          -2
        ",
        expect![[r#"
            Program {
                functions: [],
                body: Block {
                    exprs: [
                        Let {
                            id: "a",
                            type_reference: Int,
                            init_expr: Int {
                                value: 1,
                            },
                        },
                        UnaryExpr {
                            kind: Negative,
                            operand: Int {
                                value: 2,
                            },
                        },
                    ],
                },
            }"#]],
    );
}
