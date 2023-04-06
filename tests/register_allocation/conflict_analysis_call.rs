// Copyright © 2022-2023 Brandon Li. All rights reserved.

// Tests for conflict analysis correctness for programs that call other functions.

use expect_test::expect;
use test_utils::conflict_analysis_ir_check;

#[test]
fn test_no_params() {
    conflict_analysis_ir_check(
        "
        fun function() : int {
          2
        }

        let a: int = 0
        let b: int = 0
        let c: int = 0

        function()
        a + b # keep a and b live, but not c
        ",
        expect![[r#"
            Block {
                exprs: [
                    Let {
                        id: "a",
                        init_expr: Direct {
                            expr: Int {
                                value: 0,
                            },
                        },
                    },
                    Let {
                        id: "b",
                        init_expr: Direct {
                            expr: Int {
                                value: 0,
                            },
                        },
                    },
                    Let {
                        id: "c",
                        init_expr: Direct {
                            expr: Int {
                                value: 0,
                            },
                        },
                    },
                    Call {
                        id: "function",
                        args: [],
                        live_variables: RefCell {
                            value: {
                                "a",
                                "b",
                            },
                        },
                    },
                    BinaryExpr {
                        kind: Plus,
                        operand_1: Id {
                            value: "a",
                            id_type: Int,
                        },
                        operand_2: Id {
                            value: "b",
                            id_type: Int,
                        },
                        operand_type: Int,
                    },
                ],
            }"#]],
    );
}

#[test]
fn test_basic() {
    conflict_analysis_ir_check(
        "
        fun function(j: int, k: int) : int {
          j + k
        }

        let a: int = 0
        let b: int = 0
        let c: int = 0

        let d: int = function(a, b)
        d
        ",
        expect![[r#"
            Block {
                exprs: [
                    Let {
                        id: "a",
                        init_expr: Direct {
                            expr: Int {
                                value: 0,
                            },
                        },
                    },
                    Let {
                        id: "b",
                        init_expr: Direct {
                            expr: Int {
                                value: 0,
                            },
                        },
                    },
                    Let {
                        id: "c",
                        init_expr: Direct {
                            expr: Int {
                                value: 0,
                            },
                        },
                    },
                    Let {
                        id: "d",
                        init_expr: Call {
                            id: "function",
                            args: [
                                Id {
                                    value: "a",
                                    id_type: Int,
                                },
                                Id {
                                    value: "b",
                                    id_type: Int,
                                },
                            ],
                            live_variables: RefCell {
                                value: {
                                    "a",
                                    "b",
                                },
                            },
                        },
                    },
                    Direct {
                        expr: Id {
                            value: "d",
                            id_type: Int,
                        },
                    },
                ],
            }"#]],
    );
}
