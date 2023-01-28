// Copyright © 2022 Brandon Li. All rights reserved.

//! Tests parser correctness of prefix expressions.

use expect_test::expect;
use test_utils::parse_check;

#[test]
fn test_prefix() {
    parse_check(
        "+1",
        expect![[r#"
            Program {
                body: Block {
                    exprs: [
                        Int {
                            value: 1,
                        },
                    ],
                },
            }"#]],
    );

    parse_check(
        "-1",
        expect![[r#"
            Program {
                body: Block {
                    exprs: [
                        UnaryExpr {
                            kind: Negative,
                            operand: Int {
                                value: 1,
                            },
                        },
                    ],
                },
            }"#]],
    );

    parse_check(
        "let name: int = --!-+-3 # incorrect semantics",
        expect![[r#"
            Program {
                body: Block {
                    exprs: [
                        Let {
                            id: "name",
                            type_reference: "int",
                            init_expr: UnaryExpr {
                                kind: Negative,
                                operand: UnaryExpr {
                                    kind: Negative,
                                    operand: UnaryExpr {
                                        kind: Not,
                                        operand: UnaryExpr {
                                            kind: Negative,
                                            operand: UnaryExpr {
                                                kind: Negative,
                                                operand: Int {
                                                    value: 3,
                                                },
                                            },
                                        },
                                    },
                                },
                            },
                        },
                    ],
                },
            }"#]],
    );

    parse_check(
        "let name: unknown = +2 - -3 - -+-+-4 + !4",
        expect![[r#"
            Program {
                body: Block {
                    exprs: [
                        Let {
                            id: "name",
                            type_reference: "unknown",
                            init_expr: BinaryExpr {
                                kind: Plus,
                                operand_1: BinaryExpr {
                                    kind: Minus,
                                    operand_1: BinaryExpr {
                                        kind: Minus,
                                        operand_1: Int {
                                            value: 2,
                                        },
                                        operand_2: UnaryExpr {
                                            kind: Negative,
                                            operand: Int {
                                                value: 3,
                                            },
                                        },
                                    },
                                    operand_2: UnaryExpr {
                                        kind: Negative,
                                        operand: UnaryExpr {
                                            kind: Negative,
                                            operand: UnaryExpr {
                                                kind: Negative,
                                                operand: Int {
                                                    value: 4,
                                                },
                                            },
                                        },
                                    },
                                },
                                operand_2: UnaryExpr {
                                    kind: Not,
                                    operand: Int {
                                        value: 4,
                                    },
                                },
                            },
                        },
                    ],
                },
            }"#]],
    );
}
