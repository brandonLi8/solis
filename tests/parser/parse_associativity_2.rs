// Copyright © 2022 Brandon Li. All rights reserved.

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
          1 <= (2 != 3)
        ",
        expect![[r#"
            Program {
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
                            },
                            operand_2: Int {
                                value: 3,
                            },
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
                            },
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
                            },
                            operand_2: Int {
                                value: 3,
                            },
                        },
                        BinaryExpr {
                            kind: LessThanOrEquals,
                            operand_1: Int {
                                value: 1,
                            },
                            operand_2: BinaryExpr {
                                kind: NotEquals,
                                operand_1: Int {
                                    value: 2,
                                },
                                operand_2: Int {
                                    value: 3,
                                },
                            },
                        },
                    ],
                },
            }"#]],
    )
}

#[test]
fn test_comparison_left_associative_2() {
    parse_check(
        "let a: bool = 32 < 2 <= (3 > ((4))) / 5 >= 3 != 2 == 2",
        expect![[r#"
            Program {
                body: Block {
                    exprs: [
                        Let {
                            id: "a",
                            type_reference: "bool",
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
                                                },
                                                operand_2: Int {
                                                    value: 5,
                                                },
                                            },
                                        },
                                        operand_2: Int {
                                            value: 3,
                                        },
                                    },
                                    operand_2: Int {
                                        value: 2,
                                    },
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
}
