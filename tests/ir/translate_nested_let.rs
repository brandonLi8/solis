// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Tests translator for programs with nested let statements.

use expect_test::expect;
use test_utils::translate_check;

#[test]
fn test_nested_let() {
    translate_check(
        "let a: () = let b: bool = !(1 < 2 == false)",
        expect![[r#"
            Program {
                functions: [],
                body: Block {
                    exprs: [
                        Let {
                            id: "@temp0",
                            init_expr: BinaryExpr {
                                kind: LessThan,
                                operand_1: Int {
                                    value: 1,
                                },
                                operand_2: Int {
                                    value: 2,
                                },
                                operand_type: Int,
                            },
                        },
                        Let {
                            id: "@temp1",
                            init_expr: BinaryExpr {
                                kind: EqualsEquals,
                                operand_1: Id {
                                    value: "@temp0",
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
                                    value: "@temp1",
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
                                id_type: Unit,
                            },
                        },
                    ],
                },
            }"#]],
    );
}
