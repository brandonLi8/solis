// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Tests parser correctness of prefix expressions.

use expect_test::expect;
use test_utils::parse_check;

#[test]
fn test_prefix() {
    parse_check(
        "+1",
        expect![[r#"
            Program {
                functions: [],
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
                functions: [],
                body: Block {
                    exprs: [
                        UnaryExpr {
                            kind: Negative,
                            operand: Int {
                                value: 1,
                            },
                            operator_position: 0..1,
                        },
                    ],
                },
            }"#]],
    );

    parse_check(
        "let name: int = --!-+-3 # incorrect semantics",
        expect![[r#"
            Program {
                functions: [],
                body: Block {
                    exprs: [
                        Let {
                            id: "name",
                            id_position: 4..8,
                            type_reference: Int,
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
                                                operator_position: 21..22,
                                            },
                                            operator_position: 19..20,
                                        },
                                        operator_position: 18..19,
                                    },
                                    operator_position: 17..18,
                                },
                                operator_position: 16..17,
                            },
                        },
                    ],
                },
            }"#]],
    );

    parse_check(
        "let name: int = +2 - -3 - -+-+-4 + !4",
        expect![[r#"
            Program {
                functions: [],
                body: Block {
                    exprs: [
                        Let {
                            id: "name",
                            id_position: 4..8,
                            type_reference: Int,
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
                                            operator_position: 21..22,
                                        },
                                        operator_position: 19..20,
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
                                                operator_position: 30..31,
                                            },
                                            operator_position: 28..29,
                                        },
                                        operator_position: 26..27,
                                    },
                                    operator_position: 24..25,
                                },
                                operand_2: UnaryExpr {
                                    kind: Not,
                                    operand: Int {
                                        value: 4,
                                    },
                                    operator_position: 35..36,
                                },
                                operator_position: 33..34,
                            },
                        },
                    ],
                },
            }"#]],
    );

    parse_check(
        "let name: bool = !!!!!!!!!!!true",
        expect![[r#"
            Program {
                functions: [],
                body: Block {
                    exprs: [
                        Let {
                            id: "name",
                            id_position: 4..8,
                            type_reference: Bool,
                            init_expr: UnaryExpr {
                                kind: Not,
                                operand: UnaryExpr {
                                    kind: Not,
                                    operand: UnaryExpr {
                                        kind: Not,
                                        operand: UnaryExpr {
                                            kind: Not,
                                            operand: UnaryExpr {
                                                kind: Not,
                                                operand: UnaryExpr {
                                                    kind: Not,
                                                    operand: UnaryExpr {
                                                        kind: Not,
                                                        operand: UnaryExpr {
                                                            kind: Not,
                                                            operand: UnaryExpr {
                                                                kind: Not,
                                                                operand: UnaryExpr {
                                                                    kind: Not,
                                                                    operand: UnaryExpr {
                                                                        kind: Not,
                                                                        operand: Bool {
                                                                            value: true,
                                                                        },
                                                                        operator_position: 27..28,
                                                                    },
                                                                    operator_position: 26..27,
                                                                },
                                                                operator_position: 25..26,
                                                            },
                                                            operator_position: 24..25,
                                                        },
                                                        operator_position: 23..24,
                                                    },
                                                    operator_position: 22..23,
                                                },
                                                operator_position: 21..22,
                                            },
                                            operator_position: 20..21,
                                        },
                                        operator_position: 19..20,
                                    },
                                    operator_position: 18..19,
                                },
                                operator_position: 17..18,
                            },
                        },
                    ],
                },
            }"#]],
    );
}
