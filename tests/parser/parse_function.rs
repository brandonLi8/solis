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
                        id_position: 13..16,
                        params: [
                            Param {
                                id: "n",
                                type_reference: Int,
                            },
                        ],
                        return_type: Int,
                        body: Block {
                            exprs: [
                                If {
                                    condition: BinaryExpr {
                                        kind: LessThanOrEquals,
                                        operand_1: Id {
                                            value: "n",
                                            position: 46..47,
                                        },
                                        operand_2: Int {
                                            value: 1,
                                        },
                                        operator_position: 48..50,
                                    },
                                    then_block: Block {
                                        exprs: [
                                            Int {
                                                value: 1,
                                            },
                                        ],
                                    },
                                    else_block: Some(
                                        Block {
                                            exprs: [
                                                BinaryExpr {
                                                    kind: Plus,
                                                    operand_1: Call {
                                                        id: "fib",
                                                        id_position: 110..113,
                                                        args: [
                                                            (
                                                                BinaryExpr {
                                                                    kind: Minus,
                                                                    operand_1: Id {
                                                                        value: "n",
                                                                        position: 114..115,
                                                                    },
                                                                    operand_2: Int {
                                                                        value: 1,
                                                                    },
                                                                    operator_position: 116..117,
                                                                },
                                                                114..119,
                                                            ),
                                                        ],
                                                    },
                                                    operand_2: Call {
                                                        id: "fib",
                                                        id_position: 123..126,
                                                        args: [
                                                            (
                                                                BinaryExpr {
                                                                    kind: Minus,
                                                                    operand_1: Id {
                                                                        value: "n",
                                                                        position: 127..128,
                                                                    },
                                                                    operand_2: Int {
                                                                        value: 2,
                                                                    },
                                                                    operator_position: 129..130,
                                                                },
                                                                127..132,
                                                            ),
                                                        ],
                                                    },
                                                    operator_position: 121..122,
                                                },
                                            ],
                                        },
                                    ),
                                    if_position: 43..45,
                                },
                            ],
                        },
                    },
                ],
                body: Block {
                    exprs: [
                        Let {
                            id: "a",
                            id_position: 169..170,
                            type_reference: Int,
                            init_expr: BinaryExpr {
                                kind: Minus,
                                operand_1: Int {
                                    value: 2,
                                },
                                operand_2: BinaryExpr {
                                    kind: Times,
                                    operand_1: Call {
                                        id: "fib",
                                        id_position: 182..185,
                                        args: [
                                            (
                                                Int {
                                                    value: 5,
                                                },
                                                186..187,
                                            ),
                                        ],
                                    },
                                    operand_2: Int {
                                        value: 3,
                                    },
                                    operator_position: 189..190,
                                },
                                operator_position: 180..181,
                            },
                            init_expr_position: 178..192,
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
                        id_position: 13..14,
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
                                BinaryExpr {
                                    kind: Plus,
                                    operand_1: BinaryExpr {
                                        kind: Plus,
                                        operand_1: Id {
                                            value: "a",
                                            position: 55..56,
                                        },
                                        operand_2: Id {
                                            value: "b",
                                            position: 59..60,
                                        },
                                        operator_position: 57..58,
                                    },
                                    operand_2: Id {
                                        value: "c",
                                        position: 63..64,
                                    },
                                    operator_position: 61..62,
                                },
                            ],
                        },
                    },
                ],
                body: Block {
                    exprs: [
                        Call {
                            id: "a",
                            id_position: 84..85,
                            args: [
                                (
                                    Int {
                                        value: 1,
                                    },
                                    86..87,
                                ),
                                (
                                    Int {
                                        value: 2,
                                    },
                                    88..89,
                                ),
                                (
                                    Int {
                                        value: 3,
                                    },
                                    93..94,
                                ),
                            ],
                        },
                    ],
                },
            }"#]],
    );
}

#[test]
fn test_multi_functions() {
    parse_check(
        "
        fun a() : int {
          2
        }

        fun a() : () {
          let a: bool = false
        }

        a(1,2,   3)
        ",
        expect![[r#"
            Program {
                functions: [
                    Function {
                        id: "a",
                        id_position: 13..14,
                        params: [],
                        return_type: Int,
                        body: Block {
                            exprs: [
                                Int {
                                    value: 2,
                                },
                            ],
                        },
                    },
                    Function {
                        id: "a",
                        id_position: 60..61,
                        params: [],
                        return_type: Unit,
                        body: Block {
                            exprs: [
                                Let {
                                    id: "a",
                                    id_position: 85..86,
                                    type_reference: Bool,
                                    init_expr: Bool {
                                        value: false,
                                    },
                                    init_expr_position: 95..100,
                                },
                            ],
                        },
                    },
                ],
                body: Block {
                    exprs: [
                        Call {
                            id: "a",
                            id_position: 120..121,
                            args: [
                                (
                                    Int {
                                        value: 1,
                                    },
                                    122..123,
                                ),
                                (
                                    Int {
                                        value: 2,
                                    },
                                    124..125,
                                ),
                                (
                                    Int {
                                        value: 3,
                                    },
                                    129..130,
                                ),
                            ],
                        },
                    ],
                },
            }"#]],
    );
}
