// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Tests parser correctness of operand associativity.

use expect_test::expect;
use test_utils::parse_check;

#[test]
fn test_comparison_left_associative_1() {
    parse_check(
        r"
          1 < 2 < 3
          1 < (2 < 3)
          1 == 2 > 3
          1. <= (2.3 != 3)
        ",
        expect![[r#"
            Program {
                functions: [],
                body: Block {
                    exprs: [
                        BinaryExpr {
                            kind: LessThan,
                            operand_1: BinaryExpr {
                                kind: LessThan,
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
                            kind: LessThan,
                            operand_1: Int {
                                value: 1,
                            },
                            operand_2: BinaryExpr {
                                kind: LessThan,
                                operand_1: Int {
                                    value: 2,
                                },
                                operand_2: Int {
                                    value: 3,
                                },
                                operator_position: 38..39,
                            },
                            operator_position: 33..34,
                        },
                        BinaryExpr {
                            kind: MoreThan,
                            operand_1: BinaryExpr {
                                kind: EqualsEquals,
                                operand_1: Int {
                                    value: 1,
                                },
                                operand_2: Int {
                                    value: 2,
                                },
                                operator_position: 55..57,
                            },
                            operand_2: Int {
                                value: 3,
                            },
                            operator_position: 60..61,
                        },
                        BinaryExpr {
                            kind: LessThanOrEquals,
                            operand_1: Float {
                                value: 1.0,
                            },
                            operand_2: BinaryExpr {
                                kind: NotEquals,
                                operand_1: Float {
                                    value: 2.3,
                                },
                                operand_2: Int {
                                    value: 3,
                                },
                                operator_position: 85..87,
                            },
                            operator_position: 77..79,
                        },
                    ],
                },
            }"#]],
    );
}

#[test]
fn test_comparison_left_associative_2() {
    parse_check(
        "let a: bool = 32 < 2 <= (3 > ((4))) / 5 >= 3 != 2 == 2",
        expect![[r#"
            Program {
                functions: [],
                body: Block {
                    exprs: [
                        Let {
                            id: "a",
                            id_position: 4..5,
                            type_reference: Bool,
                            init_expr: BinaryExpr {
                                kind: EqualsEquals,
                                operand_1: BinaryExpr {
                                    kind: NotEquals,
                                    operand_1: BinaryExpr {
                                        kind: MoreThanOrEquals,
                                        operand_1: BinaryExpr {
                                            kind: LessThanOrEquals,
                                            operand_1: BinaryExpr {
                                                kind: LessThan,
                                                operand_1: Int {
                                                    value: 32,
                                                },
                                                operand_2: Int {
                                                    value: 2,
                                                },
                                                operator_position: 17..18,
                                            },
                                            operand_2: BinaryExpr {
                                                kind: Divide,
                                                operand_1: BinaryExpr {
                                                    kind: MoreThan,
                                                    operand_1: Int {
                                                        value: 3,
                                                    },
                                                    operand_2: Int {
                                                        value: 4,
                                                    },
                                                    operator_position: 27..28,
                                                },
                                                operand_2: Int {
                                                    value: 5,
                                                },
                                                operator_position: 36..37,
                                            },
                                            operator_position: 21..23,
                                        },
                                        operand_2: Int {
                                            value: 3,
                                        },
                                        operator_position: 40..42,
                                    },
                                    operand_2: Int {
                                        value: 2,
                                    },
                                    operator_position: 45..47,
                                },
                                operand_2: Int {
                                    value: 2,
                                },
                                operator_position: 50..52,
                            },
                        },
                    ],
                },
            }"#]],
    );
}
