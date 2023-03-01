// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Tests translator for programs with nested let statements.

use expect_test::expect;
use test_utils::translate_check;

#[test]
fn test_nested_let() {
    translate_check(
        "let a: bool = let b: bool = !(1 < 2 == false)",
        expect![[r#"
            Program {
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
                                },
                                operand_type: Bool,
                            },
                        },
                        Let {
                            id: "a",
                            init_expr: Direct {
                                expr: Id {
                                    value: "b",
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
                        "@temp0": Bool,
                        "@temp1": Bool,
                        "a": Bool,
                        "b": Bool,
                    },
                },
            }"#]],
    )
}

#[test]
fn test_nested_let_1() {
    translate_check(
        "let a: bool = (let b: int = -(2 - 4 * 4)) == 2",
        expect![[r#"
            Program {
                body: Block {
                    exprs: [
                        Let {
                            id: "@temp0",
                            init_expr: BinaryExpr {
                                kind: Times,
                                operand_1: Int {
                                    value: 4,
                                },
                                operand_2: Int {
                                    value: 4,
                                },
                                operand_type: Int,
                            },
                        },
                        Let {
                            id: "@temp1",
                            init_expr: BinaryExpr {
                                kind: Minus,
                                operand_1: Int {
                                    value: 2,
                                },
                                operand_2: Id {
                                    value: "@temp0",
                                },
                                operand_type: Int,
                            },
                        },
                        Let {
                            id: "b",
                            init_expr: UnaryExpr {
                                kind: Negative,
                                operand: Id {
                                    value: "@temp1",
                                },
                                operand_type: Int,
                            },
                        },
                        Let {
                            id: "a",
                            init_expr: BinaryExpr {
                                kind: EqualsEquals,
                                operand_1: Id {
                                    value: "b",
                                },
                                operand_2: Int {
                                    value: 2,
                                },
                                operand_type: Int,
                            },
                        },
                        Direct {
                            expr: Id {
                                value: "a",
                            },
                        },
                    ],
                    identifier_types: {
                        "@temp0": Int,
                        "@temp1": Int,
                        "a": Bool,
                        "b": Int,
                    },
                },
            }"#]],
    )
}

#[test]
fn test_nested_let_2() {
    translate_check(
        "let a: int = (let b: int = let c: int = 1 + 2 + 3) + 2",
        expect![[r#"
            Program {
                body: Block {
                    exprs: [
                        Let {
                            id: "@temp0",
                            init_expr: BinaryExpr {
                                kind: Plus,
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
                            id: "c",
                            init_expr: BinaryExpr {
                                kind: Plus,
                                operand_1: Id {
                                    value: "@temp0",
                                },
                                operand_2: Int {
                                    value: 3,
                                },
                                operand_type: Int,
                            },
                        },
                        Let {
                            id: "b",
                            init_expr: Direct {
                                expr: Id {
                                    value: "c",
                                },
                            },
                        },
                        Let {
                            id: "a",
                            init_expr: BinaryExpr {
                                kind: Plus,
                                operand_1: Id {
                                    value: "b",
                                },
                                operand_2: Int {
                                    value: 2,
                                },
                                operand_type: Int,
                            },
                        },
                        Direct {
                            expr: Id {
                                value: "a",
                            },
                        },
                    ],
                    identifier_types: {
                        "@temp0": Int,
                        "a": Int,
                        "b": Int,
                        "c": Int,
                    },
                },
            }"#]],
    )
}
