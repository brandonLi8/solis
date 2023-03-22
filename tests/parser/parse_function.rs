// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Tests parsing function declarations.

use expect_test::expect;
use test_utils::parse_check;

#[test]
fn test_fib() {
    parse_check(
        "
        fun fib(n: int) : int {
          if n <= 1 {
            1
          }
          else {
            fib(n - 1) + fib(n - 2)
          }
        }

        let a: int = 2 - fib(5) * 3
        ",
        expect![[r#"
            Program {
                functions: [
                    Function {
                        id: "fib",
                        params: [
                            Param {
                                id: "n",
                                type_reference: Int,
                            },
                        ],
                        return_type: Int,
                        body: Block {
                            exprs: [
                                Expr {
                                    kind: If {
                                        condition: Expr {
                                            kind: BinaryExpr {
                                                kind: LessThanOrEquals,
                                                operand_1: Expr {
                                                    kind: Id {
                                                        value: "n",
                                                    },
                                                    position: 46..47,
                                                },
                                                operand_2: Expr {
                                                    kind: Int {
                                                        value: 1,
                                                    },
                                                    position: 51..52,
                                                },
                                            },
                                            position: 48..50,
                                        },
                                        then_block: Block {
                                            exprs: [
                                                Expr {
                                                    kind: Int {
                                                        value: 1,
                                                    },
                                                    position: 67..68,
                                                },
                                            ],
                                        },
                                        else_block: Some(
                                            Block {
                                                exprs: [
                                                    Expr {
                                                        kind: BinaryExpr {
                                                            kind: Plus,
                                                            operand_1: Expr {
                                                                kind: Call {
                                                                    id: "fib",
                                                                    args: [
                                                                        Expr {
                                                                            kind: BinaryExpr {
                                                                                kind: Minus,
                                                                                operand_1: Expr {
                                                                                    kind: Id {
                                                                                        value: "n",
                                                                                    },
                                                                                    position: 114..115,
                                                                                },
                                                                                operand_2: Expr {
                                                                                    kind: Int {
                                                                                        value: 1,
                                                                                    },
                                                                                    position: 118..119,
                                                                                },
                                                                            },
                                                                            position: 116..117,
                                                                        },
                                                                    ],
                                                                },
                                                                position: 110..113,
                                                            },
                                                            operand_2: Expr {
                                                                kind: Call {
                                                                    id: "fib",
                                                                    args: [
                                                                        Expr {
                                                                            kind: BinaryExpr {
                                                                                kind: Minus,
                                                                                operand_1: Expr {
                                                                                    kind: Id {
                                                                                        value: "n",
                                                                                    },
                                                                                    position: 127..128,
                                                                                },
                                                                                operand_2: Expr {
                                                                                    kind: Int {
                                                                                        value: 2,
                                                                                    },
                                                                                    position: 131..132,
                                                                                },
                                                                            },
                                                                            position: 129..130,
                                                                        },
                                                                    ],
                                                                },
                                                                position: 123..126,
                                                            },
                                                        },
                                                        position: 121..122,
                                                    },
                                                ],
                                            },
                                        ),
                                    },
                                    position: 43..45,
                                },
                            ],
                        },
                    },
                ],
                body: Block {
                    exprs: [
                        Expr {
                            kind: Let {
                                id: "a",
                                type_reference: Int,
                                init_expr: Expr {
                                    kind: BinaryExpr {
                                        kind: Minus,
                                        operand_1: Expr {
                                            kind: Int {
                                                value: 2,
                                            },
                                            position: 178..179,
                                        },
                                        operand_2: Expr {
                                            kind: BinaryExpr {
                                                kind: Times,
                                                operand_1: Expr {
                                                    kind: Call {
                                                        id: "fib",
                                                        args: [
                                                            Expr {
                                                                kind: Int {
                                                                    value: 5,
                                                                },
                                                                position: 186..187,
                                                            },
                                                        ],
                                                    },
                                                    position: 182..185,
                                                },
                                                operand_2: Expr {
                                                    kind: Int {
                                                        value: 3,
                                                    },
                                                    position: 191..192,
                                                },
                                            },
                                            position: 189..190,
                                        },
                                    },
                                    position: 180..181,
                                },
                            },
                            position: 172..175,
                        },
                    ],
                },
            }"#]],
    );
}

#[test]
fn test_function() {
    parse_check(
        "
        fun a(b: int, c: int,d:int) : int {
          a + b + c
        }

        a(1,2,   3)
        ",
        expect![[r#"
            Program {
                functions: [
                    Function {
                        id: "a",
                        params: [
                            Param {
                                id: "b",
                                type_reference: Int,
                            },
                            Param {
                                id: "c",
                                type_reference: Int,
                            },
                            Param {
                                id: "d",
                                type_reference: Int,
                            },
                        ],
                        return_type: Int,
                        body: Block {
                            exprs: [
                                Expr {
                                    kind: BinaryExpr {
                                        kind: Plus,
                                        operand_1: Expr {
                                            kind: BinaryExpr {
                                                kind: Plus,
                                                operand_1: Expr {
                                                    kind: Id {
                                                        value: "a",
                                                    },
                                                    position: 55..56,
                                                },
                                                operand_2: Expr {
                                                    kind: Id {
                                                        value: "b",
                                                    },
                                                    position: 59..60,
                                                },
                                            },
                                            position: 57..58,
                                        },
                                        operand_2: Expr {
                                            kind: Id {
                                                value: "c",
                                            },
                                            position: 63..64,
                                        },
                                    },
                                    position: 61..62,
                                },
                            ],
                        },
                    },
                ],
                body: Block {
                    exprs: [
                        Expr {
                            kind: Call {
                                id: "a",
                                args: [
                                    Expr {
                                        kind: Int {
                                            value: 1,
                                        },
                                        position: 86..87,
                                    },
                                    Expr {
                                        kind: Int {
                                            value: 2,
                                        },
                                        position: 88..89,
                                    },
                                    Expr {
                                        kind: Int {
                                            value: 3,
                                        },
                                        position: 93..94,
                                    },
                                ],
                            },
                            position: 84..85,
                        },
                    ],
                },
            }"#]],
    );
}
