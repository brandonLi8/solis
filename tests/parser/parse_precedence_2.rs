// Copyright © 2022 Brandon Li. All rights reserved.

//! Tests parser correctness of operand precedence.

use expect_test::expect;
use test_utils::parse_check;

#[test]
fn test_comparison_precedence_1() {
    parse_check(
        r"
          1 < 2 * 3
          1 * 2 > 3
          (1 + 2) <= 3
          1 <= (2 + 3)
        ",
        expect![[r#"
            Program {
                body: Block {
                    exprs: [
                        BinaryExpr {
                            kind: LessThan,
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
                            kind: MoreThan,
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
                            kind: LessThanOrEquals,
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
                            kind: LessThanOrEquals,
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
                    ],
                },
            }"#]],
    )
}

#[test]
fn test_comparison_precedence_2() {
    parse_check(
        r"
          let a: bool = z + y < z
          let b: bool = 1 != 2 / 3
          let c: invalid = 1 + (2 >= 3) # semantics wrong
        ",
        expect![[r#"
            Program {
                body: Block {
                    exprs: [
                        Let {
                            id: "a",
                            type_reference: "bool",
                            init_expr: BinaryExpr {
                                kind: LessThan,
                                operand_1: BinaryExpr {
                                    kind: Plus,
                                    operand_1: Id {
                                        value: "z",
                                    },
                                    operand_2: Id {
                                        value: "y",
                                    },
                                },
                                operand_2: Id {
                                    value: "z",
                                },
                            },
                        },
                        Let {
                            id: "b",
                            type_reference: "bool",
                            init_expr: BinaryExpr {
                                kind: NotEquals,
                                operand_1: Int {
                                    value: 1,
                                },
                                operand_2: BinaryExpr {
                                    kind: Divide,
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
                            id: "c",
                            type_reference: "invalid",
                            init_expr: BinaryExpr {
                                kind: Plus,
                                operand_1: Int {
                                    value: 1,
                                },
                                operand_2: BinaryExpr {
                                    kind: MoreThanOrEquals,
                                    operand_1: Int {
                                        value: 2,
                                    },
                                    operand_2: Int {
                                        value: 3,
                                    },
                                },
                            },
                        },
                    ],
                },
            }"#]],
    )
}
