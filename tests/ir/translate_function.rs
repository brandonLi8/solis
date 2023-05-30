// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Tests translating function declarations.

use expect_test::expect;
use test_utils::translate_check;

#[test]
fn test_fib() {
    translate_check(
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
                            "n",
                        ],
                        body: Block {
                            exprs: [
                                Let {
                                    id: "@temp0",
                                    init_expr: BinaryExpr {
                                        kind: LessThanOrEquals,
                                        operand_1: Id {
                                            value: "n",
                                            id_type: Int,
                                        },
                                        operand_2: Int {
                                            value: 1,
                                        },
                                        operand_type: Int,
                                    },
                                },
                                If {
                                    condition: Id {
                                        value: "@temp0",
                                        id_type: Bool,
                                    },
                                    then_block: Block {
                                        exprs: [
                                            Direct {
                                                expr: Int {
                                                    value: 1,
                                                },
                                            },
                                        ],
                                    },
                                    else_block: Some(
                                        Block {
                                            exprs: [
                                                Let {
                                                    id: "@temp1",
                                                    init_expr: BinaryExpr {
                                                        kind: Minus,
                                                        operand_1: Id {
                                                            value: "n",
                                                            id_type: Int,
                                                        },
                                                        operand_2: Int {
                                                            value: 1,
                                                        },
                                                        operand_type: Int,
                                                    },
                                                },
                                                Let {
                                                    id: "@temp2",
                                                    init_expr: BinaryExpr {
                                                        kind: Minus,
                                                        operand_1: Id {
                                                            value: "n",
                                                            id_type: Int,
                                                        },
                                                        operand_2: Int {
                                                            value: 2,
                                                        },
                                                        operand_type: Int,
                                                    },
                                                },
                                                Let {
                                                    id: "@temp3",
                                                    init_expr: Call {
                                                        id: "fib",
                                                        args: [
                                                            Id {
                                                                value: "@temp1",
                                                                id_type: Int,
                                                            },
                                                        ],
                                                        live_variables: {},
                                                    },
                                                },
                                                Let {
                                                    id: "@temp4",
                                                    init_expr: Call {
                                                        id: "fib",
                                                        args: [
                                                            Id {
                                                                value: "@temp2",
                                                                id_type: Int,
                                                            },
                                                        ],
                                                        live_variables: {},
                                                    },
                                                },
                                                BinaryExpr {
                                                    kind: Plus,
                                                    operand_1: Id {
                                                        value: "@temp3",
                                                        id_type: Int,
                                                    },
                                                    operand_2: Id {
                                                        value: "@temp4",
                                                        id_type: Int,
                                                    },
                                                    operand_type: Int,
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
                            id: "@temp5",
                            init_expr: Call {
                                id: "fib",
                                args: [
                                    Int {
                                        value: 5,
                                    },
                                ],
                                live_variables: {},
                            },
                        },
                        Let {
                            id: "@temp6",
                            init_expr: BinaryExpr {
                                kind: Times,
                                operand_1: Id {
                                    value: "@temp5",
                                    id_type: Int,
                                },
                                operand_2: Int {
                                    value: 3,
                                },
                                operand_type: Int,
                            },
                        },
                        Let {
                            id: "a",
                            init_expr: BinaryExpr {
                                kind: Minus,
                                operand_1: Int {
                                    value: 2,
                                },
                                operand_2: Id {
                                    value: "@temp6",
                                    id_type: Int,
                                },
                                operand_type: Int,
                            },
                        },
                        Direct {
                            expr: Id {
                                value: "a",
                                id_type: Int,
                            },
                        },
                    ],
                },
            }"#]],
    );
}

#[test]
fn test_function_1() {
    translate_check(
        "
        fun a(b: int, c: int, d: int) : int {
          b + c + d
        }

        a(1,2,   3)
        ",
        expect![[r#"
            Program {
                functions: [
                    Function {
                        id: "a",
                        params: [
                            "b",
                            "c",
                            "d",
                        ],
                        body: Block {
                            exprs: [
                                Let {
                                    id: "@temp0",
                                    init_expr: BinaryExpr {
                                        kind: Plus,
                                        operand_1: Id {
                                            value: "b",
                                            id_type: Int,
                                        },
                                        operand_2: Id {
                                            value: "c",
                                            id_type: Int,
                                        },
                                        operand_type: Int,
                                    },
                                },
                                BinaryExpr {
                                    kind: Plus,
                                    operand_1: Id {
                                        value: "@temp0",
                                        id_type: Int,
                                    },
                                    operand_2: Id {
                                        value: "d",
                                        id_type: Int,
                                    },
                                    operand_type: Int,
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
                            live_variables: {},
                        },
                    ],
                },
            }"#]],
    );
}

#[test]
fn test_multi_functions() {
    translate_check(
        "
        fun a(b: int, c: int, d: int) : int {
          b + c + d
        }

        fun b() : bool {
          true
        }

        a(1, 2, 3)
        ",
        expect![[r#"
            Program {
                functions: [
                    Function {
                        id: "a",
                        params: [
                            "b",
                            "c",
                            "d",
                        ],
                        body: Block {
                            exprs: [
                                Let {
                                    id: "@temp0",
                                    init_expr: BinaryExpr {
                                        kind: Plus,
                                        operand_1: Id {
                                            value: "b",
                                            id_type: Int,
                                        },
                                        operand_2: Id {
                                            value: "c",
                                            id_type: Int,
                                        },
                                        operand_type: Int,
                                    },
                                },
                                BinaryExpr {
                                    kind: Plus,
                                    operand_1: Id {
                                        value: "@temp0",
                                        id_type: Int,
                                    },
                                    operand_2: Id {
                                        value: "d",
                                        id_type: Int,
                                    },
                                    operand_type: Int,
                                },
                            ],
                        },
                    },
                    Function {
                        id: "b",
                        params: [],
                        body: Block {
                            exprs: [
                                Direct {
                                    expr: Bool {
                                        value: true,
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
                            live_variables: {},
                        },
                    ],
                },
            }"#]],
    );
}

#[test]
fn test_unit_return() {
    translate_check(
        "
        fun a() : () {
          let a: bool = false
        }
        ",
        expect![[r#"
            Program {
                functions: [
                    Function {
                        id: "a",
                        params: [],
                        body: Block {
                            exprs: [
                                Let {
                                    id: "a",
                                    init_expr: Direct {
                                        expr: Bool {
                                            value: false,
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
                    },
                ],
                body: Block {
                    exprs: [],
                },
            }"#]],
    );
}
