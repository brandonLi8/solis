// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Tests translator for programs with floating point, specifically that there is no floating point immediates.

use expect_test::expect;
use test_utils::translate_check;

#[test]
fn test_float_basic() {
    translate_check(
        "let a: float = 2.3 + 1.2; 2.6",
        expect![[r#"
            Program {
                functions: [],
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
                            init_expr: Direct {
                                expr: Float {
                                    value: 1.2,
                                },
                            },
                        },
                        Let {
                            id: "a",
                            init_expr: BinaryExpr {
                                kind: Plus,
                                operand_1: Id {
                                    value: "@temp0",
                                    id_type: Float,
                                },
                                operand_2: Id {
                                    value: "@temp1",
                                    id_type: Float,
                                },
                                operand_type: Float,
                            },
                        },
                        Let {
                            id: "@temp2",
                            init_expr: Direct {
                                expr: Float {
                                    value: 2.6,
                                },
                            },
                        },
                        Direct {
                            expr: Id {
                                value: "@temp2",
                                id_type: Float,
                            },
                        },
                    ],
                },
            }"#]],
    );
}

#[test]
fn test_float_complex() {
    translate_check(
        "
        let b: bool = !(1 < 2.3 == false)
        let a: bool = b
        ",
        expect![[r#"
            Program {
                functions: [],
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
                            id: "@temp2",
                            init_expr: BinaryExpr {
                                kind: LessThan,
                                operand_1: Id {
                                    value: "@temp1",
                                    id_type: Float,
                                },
                                operand_2: Id {
                                    value: "@temp0",
                                    id_type: Float,
                                },
                                operand_type: Float,
                            },
                        },
                        Let {
                            id: "@temp3",
                            init_expr: BinaryExpr {
                                kind: EqualsEquals,
                                operand_1: Id {
                                    value: "@temp2",
                                    id_type: Bool,
                                },
                                operand_2: Bool {
                                    value: false,
                                },
                                operand_type: Bool,
                            },
                        },
                        Let {
                            id: "b",
                            init_expr: UnaryExpr {
                                kind: Not,
                                operand: Id {
                                    value: "@temp3",
                                    id_type: Bool,
                                },
                                operand_type: Bool,
                            },
                        },
                        Let {
                            id: "a",
                            init_expr: Direct {
                                expr: Id {
                                    value: "b",
                                    id_type: Bool,
                                },
                            },
                        },
                        Direct {
                            expr: Id {
                                value: "a",
                                id_type: Bool,
                            },
                        },
                    ],
                },
            }"#]],
    );
}
