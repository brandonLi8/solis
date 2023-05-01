// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Tests parser correctness of operand precedence.

use expect_test::expect;
use test_utils::parse_check;

#[test]
fn test_arithmetic_precedence() {
    parse_check(
        r"
          1 + 2 * 3
          1 * 2 + 3
          (1 + 2) * 3
          1 * (2 + 3)
          1 / 2 * 3
          let a: int = 1 - 2 % 3
          let b: int = 1 % 2 - 3
        ",
        expect![[r#"
            Program {
                functions: [],
                body: Block {
                    exprs: [
                        BinaryExpr {
                            kind: Plus,
                            operand_1: Int {
                                value: 1,
                            },
                            operand_2: BinaryExpr {
                                kind: Times,
                                operand_1: Int {
                                    value: 2,
                                },
                                operand_2: Int {
                                    value: 3,
                                },
                            },
                        },
                        BinaryExpr {
                            kind: Plus,
                            operand_1: BinaryExpr {
                                kind: Times,
                                operand_1: Int {
                                    value: 1,
                                },
                                operand_2: Int {
                                    value: 2,
                                },
                            },
                            operand_2: Int {
                                value: 3,
                            },
                        },
                        BinaryExpr {
                            kind: Times,
                            operand_1: BinaryExpr {
                                kind: Plus,
                                operand_1: Int {
                                    value: 1,
                                },
                                operand_2: Int {
                                    value: 2,
                                },
                            },
                            operand_2: Int {
                                value: 3,
                            },
                        },
                        BinaryExpr {
                            kind: Times,
                            operand_1: Int {
                                value: 1,
                            },
                            operand_2: BinaryExpr {
                                kind: Plus,
                                operand_1: Int {
                                    value: 2,
                                },
                                operand_2: Int {
                                    value: 3,
                                },
                            },
                        },
                        BinaryExpr {
                            kind: Times,
                            operand_1: BinaryExpr {
                                kind: Divide,
                                operand_1: Int {
                                    value: 1,
                                },
                                operand_2: Int {
                                    value: 2,
                                },
                            },
                            operand_2: Int {
                                value: 3,
                            },
                        },
                        Let {
                            id: "a",
                            id_position: 119..120,
                            type_reference: Int,
                            init_expr: BinaryExpr {
                                kind: Minus,
                                operand_1: Int {
                                    value: 1,
                                },
                                operand_2: BinaryExpr {
                                    kind: Mod,
                                    operand_1: Int {
                                        value: 2,
                                    },
                                    operand_2: Int {
                                        value: 3,
                                    },
                                },
                            },
                        },
                        Let {
                            id: "b",
                            id_position: 152..153,
                            type_reference: Int,
                            init_expr: BinaryExpr {
                                kind: Minus,
                                operand_1: BinaryExpr {
                                    kind: Mod,
                                    operand_1: Int {
                                        value: 1,
                                    },
                                    operand_2: Int {
                                        value: 2,
                                    },
                                },
                                operand_2: Int {
                                    value: 3,
                                },
                            },
                        },
                    ],
                },
            }"#]],
    );
}
