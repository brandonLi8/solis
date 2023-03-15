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
                body: Block {
                    exprs: [
                        Expr {
                            kind: Int {
                                value: 1,
                            },
                            position: 1..2,
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
                        Expr {
                            kind: UnaryExpr {
                                kind: Negative,
                                operand: Expr {
                                    kind: Int {
                                        value: 1,
                                    },
                                    position: 1..2,
                                },
                            },
                            position: 0..1,
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
                        Expr {
                            kind: Let {
                                id: "name",
                                type_reference: Int,
                                init_expr: Expr {
                                    kind: UnaryExpr {
                                        kind: Negative,
                                        operand: Expr {
                                            kind: UnaryExpr {
                                                kind: Negative,
                                                operand: Expr {
                                                    kind: UnaryExpr {
                                                        kind: Not,
                                                        operand: Expr {
                                                            kind: UnaryExpr {
                                                                kind: Negative,
                                                                operand: Expr {
                                                                    kind: UnaryExpr {
                                                                        kind: Negative,
                                                                        operand: Expr {
                                                                            kind: Int {
                                                                                value: 3,
                                                                            },
                                                                            position: 22..23,
                                                                        },
                                                                    },
                                                                    position: 21..22,
                                                                },
                                                            },
                                                            position: 19..20,
                                                        },
                                                    },
                                                    position: 18..19,
                                                },
                                            },
                                            position: 17..18,
                                        },
                                    },
                                    position: 16..17,
                                },
                            },
                            position: 10..13,
                        },
                    ],
                },
            }"#]],
    );

    parse_check(
        "let name: int = +2 - -3 - -+-+-4 + !4",
        expect![[r#"
            Program {
                body: Block {
                    exprs: [
                        Expr {
                            kind: Let {
                                id: "name",
                                type_reference: Int,
                                init_expr: Expr {
                                    kind: BinaryExpr {
                                        kind: Plus,
                                        operand_1: Expr {
                                            kind: BinaryExpr {
                                                kind: Minus,
                                                operand_1: Expr {
                                                    kind: BinaryExpr {
                                                        kind: Minus,
                                                        operand_1: Expr {
                                                            kind: Int {
                                                                value: 2,
                                                            },
                                                            position: 17..18,
                                                        },
                                                        operand_2: Expr {
                                                            kind: UnaryExpr {
                                                                kind: Negative,
                                                                operand: Expr {
                                                                    kind: Int {
                                                                        value: 3,
                                                                    },
                                                                    position: 22..23,
                                                                },
                                                            },
                                                            position: 21..22,
                                                        },
                                                    },
                                                    position: 19..20,
                                                },
                                                operand_2: Expr {
                                                    kind: UnaryExpr {
                                                        kind: Negative,
                                                        operand: Expr {
                                                            kind: UnaryExpr {
                                                                kind: Negative,
                                                                operand: Expr {
                                                                    kind: UnaryExpr {
                                                                        kind: Negative,
                                                                        operand: Expr {
                                                                            kind: Int {
                                                                                value: 4,
                                                                            },
                                                                            position: 31..32,
                                                                        },
                                                                    },
                                                                    position: 30..31,
                                                                },
                                                            },
                                                            position: 28..29,
                                                        },
                                                    },
                                                    position: 26..27,
                                                },
                                            },
                                            position: 24..25,
                                        },
                                        operand_2: Expr {
                                            kind: UnaryExpr {
                                                kind: Not,
                                                operand: Expr {
                                                    kind: Int {
                                                        value: 4,
                                                    },
                                                    position: 36..37,
                                                },
                                            },
                                            position: 35..36,
                                        },
                                    },
                                    position: 33..34,
                                },
                            },
                            position: 10..13,
                        },
                    ],
                },
            }"#]],
    );

    parse_check(
        "let name: bool = !!!!!!!!!!!true",
        expect![[r#"
            Program {
                body: Block {
                    exprs: [
                        Expr {
                            kind: Let {
                                id: "name",
                                type_reference: Bool,
                                init_expr: Expr {
                                    kind: UnaryExpr {
                                        kind: Not,
                                        operand: Expr {
                                            kind: UnaryExpr {
                                                kind: Not,
                                                operand: Expr {
                                                    kind: UnaryExpr {
                                                        kind: Not,
                                                        operand: Expr {
                                                            kind: UnaryExpr {
                                                                kind: Not,
                                                                operand: Expr {
                                                                    kind: UnaryExpr {
                                                                        kind: Not,
                                                                        operand: Expr {
                                                                            kind: UnaryExpr {
                                                                                kind: Not,
                                                                                operand: Expr {
                                                                                    kind: UnaryExpr {
                                                                                        kind: Not,
                                                                                        operand: Expr {
                                                                                            kind: UnaryExpr {
                                                                                                kind: Not,
                                                                                                operand: Expr {
                                                                                                    kind: UnaryExpr {
                                                                                                        kind: Not,
                                                                                                        operand: Expr {
                                                                                                            kind: UnaryExpr {
                                                                                                                kind: Not,
                                                                                                                operand: Expr {
                                                                                                                    kind: UnaryExpr {
                                                                                                                        kind: Not,
                                                                                                                        operand: Expr {
                                                                                                                            kind: Bool {
                                                                                                                                value: true,
                                                                                                                            },
                                                                                                                            position: 28..32,
                                                                                                                        },
                                                                                                                    },
                                                                                                                    position: 27..28,
                                                                                                                },
                                                                                                            },
                                                                                                            position: 26..27,
                                                                                                        },
                                                                                                    },
                                                                                                    position: 25..26,
                                                                                                },
                                                                                            },
                                                                                            position: 24..25,
                                                                                        },
                                                                                    },
                                                                                    position: 23..24,
                                                                                },
                                                                            },
                                                                            position: 22..23,
                                                                        },
                                                                    },
                                                                    position: 21..22,
                                                                },
                                                            },
                                                            position: 20..21,
                                                        },
                                                    },
                                                    position: 19..20,
                                                },
                                            },
                                            position: 18..19,
                                        },
                                    },
                                    position: 17..18,
                                },
                            },
                            position: 10..14,
                        },
                    ],
                },
            }"#]],
    );
}
