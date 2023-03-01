// Copyright © 2022-2023 Brandon Li. All rights reserved.

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
                    identifier_types: {},
                },
            }"#]],
    )
}

#[test]
fn test_direct() {
    translate_check(
        "2",
        expect![[r#"
            Program {
                body: Block {
                    exprs: [
                        Direct {
                            expr: Int {
                                value: 2,
                            },
                        },
                    ],
                    identifier_types: {},
                },
            }"#]],
    )
}

#[test]
fn test_direct_2() {
    translate_check(
        "let a: float = 2.; a",
        expect![[r#"
            Program {
                body: Block {
                    exprs: [
                        Let {
                            id: "@temp0",
                            init_expr: Direct {
                                expr: Float {
                                    value: 2.0,
                                },
                            },
                        },
                        Let {
                            id: "a",
                            init_expr: Direct {
                                expr: Id {
                                    value: "@temp0",
                                },
                            },
                        },
                        Direct {
                            expr: Id {
                                value: "a",
                            },
                        },
                    ],
                    identifier_types: {
                        "@temp0": Float,
                        "a": Float,
                    },
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
                                operand_type: Int,
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
                                operand_type: Int,
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
                                operand_type: Int,
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
                                operand_type: Int,
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
                                operand_type: Int,
                            },
                        },
                        Direct {
                            expr: Id {
                                value: "b",
                            },
                        },
                    ],
                    identifier_types: {
                        "@temp0": Int,
                        "@temp1": Int,
                        "@temp2": Int,
                        "a": Int,
                        "b": Int,
                    },
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
                                operand_type: Int,
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
                                operand_type: Int,
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
                                operand_type: Int,
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
                                operand_type: Int,
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
                                operand_type: Int,
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
                            operand_type: Int,
                        },
                    ],
                    identifier_types: {
                        "@temp3": Int,
                        "@temp4": Int,
                        "@temp5": Int,
                        "@temp6": Int,
                        "@temp7": Int,
                    },
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
                                operand_type: Bool,
                            },
                        },
                        Let {
                            id: "@temp9",
                            init_expr: UnaryExpr {
                                kind: Not,
                                operand: Id {
                                    value: "@temp8",
                                },
                                operand_type: Bool,
                            },
                        },
                        Let {
                            id: "@temp10",
                            init_expr: UnaryExpr {
                                kind: Not,
                                operand: Id {
                                    value: "@temp9",
                                },
                                operand_type: Bool,
                            },
                        },
                        UnaryExpr {
                            kind: Not,
                            operand: Id {
                                value: "@temp10",
                            },
                            operand_type: Bool,
                        },
                    ],
                    identifier_types: {
                        "@temp10": Bool,
                        "@temp8": Bool,
                        "@temp9": Bool,
                    },
                },
            }"#]],
    )
}
