// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Tests translator for programs with floating point, specifically expressions that involve floats.

use expect_test::expect;
use test_utils::translate_check;

#[test]
fn test_float_plus_int_basic() {
    translate_check(
        "let a: float = 2.3 + 1
         let b: float = 2 + 1.2
         let c: float = 2.1 + 3.14",
        expect![[r#"
            Program {
                body: Block {
                    exprs: [
                        Let {
                            id: "@temp0",
                            init_expr: Direct {
                                expr: Float {
                                    value: 2.3,
                                },
                            },
                        },
                        Let {
                            id: "@temp1",
                            init_expr: TypeCoercion {
                                expr: Int {
                                    value: 1,
                                },
                                from_type: Int,
                                to_type: Float,
                            },
                        },
                        Let {
                            id: "a",
                            init_expr: BinaryExpr {
                                kind: Plus,
                                operand_1: Id {
                                    value: "@temp0",
                                },
                                operand_2: Id {
                                    value: "@temp1",
                                },
                                operand_type: Float,
                            },
                        },
                        Let {
                            id: "@temp2",
                            init_expr: Direct {
                                expr: Float {
                                    value: 1.2,
                                },
                            },
                        },
                        Let {
                            id: "@temp3",
                            init_expr: TypeCoercion {
                                expr: Int {
                                    value: 2,
                                },
                                from_type: Int,
                                to_type: Float,
                            },
                        },
                        Let {
                            id: "b",
                            init_expr: BinaryExpr {
                                kind: Plus,
                                operand_1: Id {
                                    value: "@temp3",
                                },
                                operand_2: Id {
                                    value: "@temp2",
                                },
                                operand_type: Float,
                            },
                        },
                        Let {
                            id: "@temp4",
                            init_expr: Direct {
                                expr: Float {
                                    value: 2.1,
                                },
                            },
                        },
                        Let {
                            id: "@temp5",
                            init_expr: Direct {
                                expr: Float {
                                    value: 3.14,
                                },
                            },
                        },
                        Let {
                            id: "c",
                            init_expr: BinaryExpr {
                                kind: Plus,
                                operand_1: Id {
                                    value: "@temp4",
                                },
                                operand_2: Id {
                                    value: "@temp5",
                                },
                                operand_type: Float,
                            },
                        },
                        Direct {
                            expr: Id {
                                value: "c",
                            },
                        },
                    ],
                    identifier_types: {
                        "@temp0": Float,
                        "@temp1": Float,
                        "@temp2": Float,
                        "@temp3": Float,
                        "@temp4": Float,
                        "@temp5": Float,
                        "a": Float,
                        "b": Float,
                        "c": Float,
                    },
                },
            }"#]],
    )
}

#[test]
fn test_float_other_2() {
    translate_check(
        "let a: bool = (2.3 < 3) == true
         let d: float = -2.3
         let b: float = 2 + 1.2 + 3 + 1
         let c: float = 2 + 1 + 3 + 1.2",
        expect![[r#"
            Program {
                body: Block {
                    exprs: [
                        Let {
                            id: "@temp0",
                            init_expr: Direct {
                                expr: Float {
                                    value: 2.3,
                                },
                            },
                        },
                        Let {
                            id: "@temp1",
                            init_expr: TypeCoercion {
                                expr: Int {
                                    value: 3,
                                },
                                from_type: Int,
                                to_type: Float,
                            },
                        },
                        Let {
                            id: "@temp2",
                            init_expr: BinaryExpr {
                                kind: LessThan,
                                operand_1: Id {
                                    value: "@temp0",
                                },
                                operand_2: Id {
                                    value: "@temp1",
                                },
                                operand_type: Float,
                            },
                        },
                        Let {
                            id: "a",
                            init_expr: BinaryExpr {
                                kind: EqualsEquals,
                                operand_1: Id {
                                    value: "@temp2",
                                },
                                operand_2: Bool {
                                    value: true,
                                },
                                operand_type: Bool,
                            },
                        },
                        Let {
                            id: "@temp3",
                            init_expr: Direct {
                                expr: Float {
                                    value: 2.3,
                                },
                            },
                        },
                        Let {
                            id: "d",
                            init_expr: UnaryExpr {
                                kind: Negative,
                                operand: Id {
                                    value: "@temp3",
                                },
                                operand_type: Float,
                            },
                        },
                        Let {
                            id: "@temp4",
                            init_expr: Direct {
                                expr: Float {
                                    value: 1.2,
                                },
                            },
                        },
                        Let {
                            id: "@temp5",
                            init_expr: TypeCoercion {
                                expr: Int {
                                    value: 2,
                                },
                                from_type: Int,
                                to_type: Float,
                            },
                        },
                        Let {
                            id: "@temp6",
                            init_expr: BinaryExpr {
                                kind: Plus,
                                operand_1: Id {
                                    value: "@temp5",
                                },
                                operand_2: Id {
                                    value: "@temp4",
                                },
                                operand_type: Float,
                            },
                        },
                        Let {
                            id: "@temp7",
                            init_expr: TypeCoercion {
                                expr: Int {
                                    value: 3,
                                },
                                from_type: Int,
                                to_type: Float,
                            },
                        },
                        Let {
                            id: "@temp8",
                            init_expr: BinaryExpr {
                                kind: Plus,
                                operand_1: Id {
                                    value: "@temp6",
                                },
                                operand_2: Id {
                                    value: "@temp7",
                                },
                                operand_type: Float,
                            },
                        },
                        Let {
                            id: "@temp9",
                            init_expr: TypeCoercion {
                                expr: Int {
                                    value: 1,
                                },
                                from_type: Int,
                                to_type: Float,
                            },
                        },
                        Let {
                            id: "b",
                            init_expr: BinaryExpr {
                                kind: Plus,
                                operand_1: Id {
                                    value: "@temp8",
                                },
                                operand_2: Id {
                                    value: "@temp9",
                                },
                                operand_type: Float,
                            },
                        },
                        Let {
                            id: "@temp10",
                            init_expr: BinaryExpr {
                                kind: Plus,
                                operand_1: Int {
                                    value: 2,
                                },
                                operand_2: Int {
                                    value: 1,
                                },
                                operand_type: Int,
                            },
                        },
                        Let {
                            id: "@temp11",
                            init_expr: Direct {
                                expr: Float {
                                    value: 1.2,
                                },
                            },
                        },
                        Let {
                            id: "@temp12",
                            init_expr: BinaryExpr {
                                kind: Plus,
                                operand_1: Id {
                                    value: "@temp10",
                                },
                                operand_2: Int {
                                    value: 3,
                                },
                                operand_type: Int,
                            },
                        },
                        Let {
                            id: "@temp13",
                            init_expr: TypeCoercion {
                                expr: Id {
                                    value: "@temp12",
                                },
                                from_type: Int,
                                to_type: Float,
                            },
                        },
                        Let {
                            id: "c",
                            init_expr: BinaryExpr {
                                kind: Plus,
                                operand_1: Id {
                                    value: "@temp13",
                                },
                                operand_2: Id {
                                    value: "@temp11",
                                },
                                operand_type: Float,
                            },
                        },
                        Direct {
                            expr: Id {
                                value: "c",
                            },
                        },
                    ],
                    identifier_types: {
                        "@temp0": Float,
                        "@temp1": Float,
                        "@temp10": Int,
                        "@temp11": Float,
                        "@temp12": Int,
                        "@temp13": Float,
                        "@temp2": Bool,
                        "@temp3": Float,
                        "@temp4": Float,
                        "@temp5": Float,
                        "@temp6": Float,
                        "@temp7": Float,
                        "@temp8": Float,
                        "@temp9": Float,
                        "a": Bool,
                        "b": Float,
                        "c": Float,
                        "d": Float,
                    },
                },
            }"#]],
    )
}
