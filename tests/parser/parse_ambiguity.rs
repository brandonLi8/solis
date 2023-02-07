// Copyright © 2022 Brandon Li. All rights reserved.

//! Tests parser correctness for https://github.com/brandonLi8/solis/issues/28

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
                body: Block {
                    exprs: [
                        Expr {
                            kind: Let {
                                id: "a",
                                type_reference: "int",
                                init_expr: Expr {
                                    kind: BinaryExpr {
                                        kind: Minus,
                                        operand_1: Expr {
                                            kind: Int {
                                                value: 1,
                                            },
                                            position: 24..25,
                                        },
                                        operand_2: Expr {
                                            kind: Int {
                                                value: 2,
                                            },
                                            position: 37..38,
                                        },
                                    },
                                    position: 36..37,
                                },
                            },
                            position: 18..21,
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
                body: Block {
                    exprs: [
                        Expr {
                            kind: Let {
                                id: "a",
                                type_reference: "int",
                                init_expr: Expr {
                                    kind: Int {
                                        value: 1,
                                    },
                                    position: 24..25,
                                },
                            },
                            position: 18..21,
                        },
                        Expr {
                            kind: UnaryExpr {
                                kind: Negative,
                                operand: Expr {
                                    kind: Int {
                                        value: 2,
                                    },
                                    position: 38..39,
                                },
                            },
                            position: 37..38,
                        },
                    ],
                },
            }"#]],
    )
}
