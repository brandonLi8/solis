// Copyright © 2022 Brandon Li. All rights reserved.

//! Tests translator for programs with nested let statements.

use expect_test::expect;
use test_utils::translate_check;

#[test]
fn test_nested_let() {
    translate_check(
        "let a: int = let b: int = !(1 < 2 < 3)",
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
                            },
                        },
                        Let {
                            id: "@temp1",
                            init_expr: BinaryExpr {
                                kind: LessThan,
                                operand_1: Id {
                                    value: "@temp0",
                                },
                                operand_2: Int {
                                    value: 3,
                                },
                            },
                        },
                        Let {
                            id: "a",
                            init_expr: Let {
                                id: "b",
                                init_expr: UnaryExpr {
                                    kind: Not,
                                    operand: Id {
                                        value: "@temp1",
                                    },
                                },
                            },
                        },
                    ],
                },
            }"#]],
    )
}

#[test]
fn test_nested_let_1() {
    translate_check(
        "let a: int = (let b: int = !(1 < 2 < 3)) + 2",
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
                            },
                        },
                        Let {
                            id: "@temp1",
                            init_expr: BinaryExpr {
                                kind: LessThan,
                                operand_1: Id {
                                    value: "@temp0",
                                },
                                operand_2: Int {
                                    value: 3,
                                },
                            },
                        },
                        Let {
                            id: "@temp2",
                            init_expr: Let {
                                id: "b",
                                init_expr: UnaryExpr {
                                    kind: Not,
                                    operand: Id {
                                        value: "@temp1",
                                    },
                                },
                            },
                        },
                        Let {
                            id: "a",
                            init_expr: BinaryExpr {
                                kind: Plus,
                                operand_1: Id {
                                    value: "@temp2",
                                },
                                operand_2: Int {
                                    value: 2,
                                },
                            },
                        },
                    ],
                },
            }"#]],
    )
}

#[test]
fn test_nested_let_2() {
    translate_check(
        "let a: int = (let b: int = let a: int = 1 + 2 + 3) + 2",
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
                            },
                        },
                        Let {
                            id: "@temp1",
                            init_expr: Let {
                                id: "b",
                                init_expr: Let {
                                    id: "a",
                                    init_expr: BinaryExpr {
                                        kind: Plus,
                                        operand_1: Id {
                                            value: "@temp0",
                                        },
                                        operand_2: Int {
                                            value: 3,
                                        },
                                    },
                                },
                            },
                        },
                        Let {
                            id: "a",
                            init_expr: BinaryExpr {
                                kind: Plus,
                                operand_1: Id {
                                    value: "@temp1",
                                },
                                operand_2: Int {
                                    value: 2,
                                },
                            },
                        },
                    ],
                },
            }"#]],
    )
}
