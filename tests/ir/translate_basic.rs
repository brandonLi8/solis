// Copyright © 2022 Brandon Li. All rights reserved.

//! Tests basic flattening functionality of the translator.

use expect_test::expect;
use test_utils::translate_check;

#[test]
fn test_empty() {
    translate_check(
        "",
        expect![[r#"
            Program {
                body: Block {
                    exprs: [],
                },
            }"#]],
    )
}

#[test]
fn test_basic() {
    translate_check(
        "let a: int = 2 + 3
         let b: int = a + 1 - 2 + 3 * 4",
        expect![[r#"
            Program {
                body: Block {
                    exprs: [
                        Let {
                            id: "a",
                            init_expr: BinaryExpr {
                                kind: Plus,
                                operand_1: Int {
                                    value: 2,
                                },
                                operand_2: Int {
                                    value: 3,
                                },
                            },
                        },
                        Let {
                            id: "@temp0",
                            init_expr: BinaryExpr {
                                kind: Plus,
                                operand_1: Id {
                                    value: "a",
                                },
                                operand_2: Int {
                                    value: 1,
                                },
                            },
                        },
                        Let {
                            id: "@temp1",
                            init_expr: BinaryExpr {
                                kind: Minus,
                                operand_1: Id {
                                    value: "@temp0",
                                },
                                operand_2: Int {
                                    value: 2,
                                },
                            },
                        },
                        Let {
                            id: "@temp2",
                            init_expr: BinaryExpr {
                                kind: Times,
                                operand_1: Int {
                                    value: 3,
                                },
                                operand_2: Int {
                                    value: 4,
                                },
                            },
                        },
                        Let {
                            id: "b",
                            init_expr: BinaryExpr {
                                kind: Plus,
                                operand_1: Id {
                                    value: "@temp1",
                                },
                                operand_2: Id {
                                    value: "@temp2",
                                },
                            },
                        },
                    ],
                },
            }"#]],
    );

    translate_check(
        "32 - 2 * (3 + ((4))) / 5 == 3 * 2",
        expect![[r#"
            Program {
                body: Block {
                    exprs: [
                        Let {
                            id: "@temp3",
                            init_expr: BinaryExpr {
                                kind: Plus,
                                operand_1: Int {
                                    value: 3,
                                },
                                operand_2: Int {
                                    value: 4,
                                },
                            },
                        },
                        Let {
                            id: "@temp4",
                            init_expr: BinaryExpr {
                                kind: Times,
                                operand_1: Int {
                                    value: 2,
                                },
                                operand_2: Id {
                                    value: "@temp3",
                                },
                            },
                        },
                        Let {
                            id: "@temp5",
                            init_expr: BinaryExpr {
                                kind: Divide,
                                operand_1: Id {
                                    value: "@temp4",
                                },
                                operand_2: Int {
                                    value: 5,
                                },
                            },
                        },
                        Let {
                            id: "@temp6",
                            init_expr: BinaryExpr {
                                kind: Minus,
                                operand_1: Int {
                                    value: 32,
                                },
                                operand_2: Id {
                                    value: "@temp5",
                                },
                            },
                        },
                        Let {
                            id: "@temp7",
                            init_expr: BinaryExpr {
                                kind: Times,
                                operand_1: Int {
                                    value: 3,
                                },
                                operand_2: Int {
                                    value: 2,
                                },
                            },
                        },
                        BinaryExpr {
                            kind: EqualsEquals,
                            operand_1: Id {
                                value: "@temp6",
                            },
                            operand_2: Id {
                                value: "@temp7",
                            },
                        },
                    ],
                },
            }"#]],
    );

    translate_check(
        "!!!(true == false)",
        expect![[r#"
            Program {
                body: Block {
                    exprs: [
                        Let {
                            id: "@temp8",
                            init_expr: BinaryExpr {
                                kind: EqualsEquals,
                                operand_1: Bool {
                                    value: true,
                                },
                                operand_2: Bool {
                                    value: false,
                                },
                            },
                        },
                        Let {
                            id: "@temp9",
                            init_expr: UnaryExpr {
                                kind: Not,
                                operand: Id {
                                    value: "@temp8",
                                },
                            },
                        },
                        Let {
                            id: "@temp10",
                            init_expr: UnaryExpr {
                                kind: Not,
                                operand: Id {
                                    value: "@temp9",
                                },
                            },
                        },
                        UnaryExpr {
                            kind: Not,
                            operand: Id {
                                value: "@temp10",
                            },
                        },
                    ],
                },
            }"#]],
    )
}
