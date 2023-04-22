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
                                If {
                                    condition: BinaryExpr {
                                        kind: LessThanOrEquals,
                                        operand_1: Id {
                                            value: "n",
                                        },
                                        operand_2: Int {
                                            value: 1,
                                        },
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
                                                        args: [
                                                            BinaryExpr {
                                                                kind: Minus,
                                                                operand_1: Id {
                                                                    value: "n",
                                                                },
                                                                operand_2: Int {
                                                                    value: 1,
                                                                },
                                                            },
                                                        ],
                                                    },
                                                    operand_2: Call {
                                                        id: "fib",
                                                        args: [
                                                            BinaryExpr {
                                                                kind: Minus,
                                                                operand_1: Id {
                                                                    value: "n",
                                                                },
                                                                operand_2: Int {
                                                                    value: 2,
                                                                },
                                                            },
                                                        ],
                                                    },
                                                },
                                            ],
                                        },
                                    ),
                                },
                            ],
                        },
                    },
                ],
                body: Block {
                    exprs: [
                        Let {
                            id: "a",
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
                                        args: [
                                            Int {
                                                value: 5,
                                            },
                                        ],
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
                                BinaryExpr {
                                    kind: Plus,
                                    operand_1: BinaryExpr {
                                        kind: Plus,
                                        operand_1: Id {
                                            value: "a",
                                        },
                                        operand_2: Id {
                                            value: "b",
                                        },
                                    },
                                    operand_2: Id {
                                        value: "c",
                                    },
                                },
                            ],
                        },
                    },
                ],
                body: Block {
                    exprs: [
                        Call {
                            id: "a",
                            args: [
                                Int {
                                    value: 1,
                                },
                                Int {
                                    value: 2,
                                },
                                Int {
                                    value: 3,
                                },
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
                        params: [],
                        return_type: Unit,
                        body: Block {
                            exprs: [
                                Let {
                                    id: "a",
                                    type_reference: Bool,
                                    init_expr: Bool {
                                        value: false,
                                    },
                                },
                            ],
                        },
                    },
                ],
                body: Block {
                    exprs: [
                        Call {
                            id: "a",
                            args: [
                                Int {
                                    value: 1,
                                },
                                Int {
                                    value: 2,
                                },
                                Int {
                                    value: 3,
                                },
                            ],
                        },
                    ],
                },
            }"#]],
    );
}
