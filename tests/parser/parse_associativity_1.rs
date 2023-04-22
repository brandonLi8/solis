// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Tests parser correctness of operand associativity.

use expect_test::expect;
use test_utils::parse_check;

#[test]
fn test_arithmetic_left_associative_1() {
    parse_check(
        r"
          1 + 2 + 3
          1 - 2 - 3
          (1 + 2) + 3
          1 - (2 + 3)
          1 / 2 * 3
          1 * 2 % 3
          1 % 2 / 3
        ",
        expect![[r#"
            Program {
                functions: [],
                body: Block {
                    exprs: [
                        BinaryExpr {
                            kind: Plus,
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
                            kind: Minus,
                            operand_1: BinaryExpr {
                                kind: Minus,
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
                            kind: Plus,
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
                            kind: Minus,
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
                        BinaryExpr {
                            kind: Mod,
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
                            kind: Divide,
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
                    ],
                },
            }"#]],
    );
}

#[test]
fn test_arithmetic_left_associative_2() {
    parse_check(
        "let a: int = 32 - 2 * (3. + ((4))) / 5 - 3 % 2.",
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
                                operand_1: BinaryExpr {
                                    kind: Minus,
                                    operand_1: Int {
                                        value: 32,
                                    },
                                    operand_2: BinaryExpr {
                                        kind: Divide,
                                        operand_1: BinaryExpr {
                                            kind: Times,
                                            operand_1: Int {
                                                value: 2,
                                            },
                                            operand_2: BinaryExpr {
                                                kind: Plus,
                                                operand_1: Float {
                                                    value: 3.0,
                                                },
                                                operand_2: Int {
                                                    value: 4,
                                                },
                                            },
                                        },
                                        operand_2: Int {
                                            value: 5,
                                        },
                                    },
                                },
                                operand_2: BinaryExpr {
                                    kind: Mod,
                                    operand_1: Int {
                                        value: 3,
                                    },
                                    operand_2: Float {
                                        value: 2.0,
                                    },
                                },
                            },
                        },
                    ],
                },
            }"#]],
    );
}
