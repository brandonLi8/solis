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
                                operator_position: 13..14,
                            },
                            operand_2: Int {
                                value: 3,
                            },
                            operator_position: 17..18,
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
                                operator_position: 33..34,
                            },
                            operand_2: Int {
                                value: 3,
                            },
                            operator_position: 37..38,
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
                                operator_position: 54..55,
                            },
                            operand_2: Int {
                                value: 3,
                            },
                            operator_position: 59..60,
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
                                operator_position: 80..81,
                            },
                            operator_position: 75..76,
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
                                operator_position: 97..98,
                            },
                            operand_2: Int {
                                value: 3,
                            },
                            operator_position: 101..102,
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
                                operator_position: 117..118,
                            },
                            operand_2: Int {
                                value: 3,
                            },
                            operator_position: 121..122,
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
                                operator_position: 137..138,
                            },
                            operand_2: Int {
                                value: 3,
                            },
                            operator_position: 141..142,
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
                            id_position: 4..5,
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
                                                operator_position: 26..27,
                                            },
                                            operator_position: 20..21,
                                        },
                                        operand_2: Int {
                                            value: 5,
                                        },
                                        operator_position: 35..36,
                                    },
                                    operator_position: 16..17,
                                },
                                operand_2: BinaryExpr {
                                    kind: Mod,
                                    operand_1: Int {
                                        value: 3,
                                    },
                                    operand_2: Float {
                                        value: 2.0,
                                    },
                                    operator_position: 43..44,
                                },
                                operator_position: 39..40,
                            },
                            init_expr_position: 13..47,
                        },
                    ],
                },
            }"#]],
    );
}
