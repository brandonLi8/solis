// Copyright © 2022 Brandon Li. All rights reserved.

//! Basic tests for the parser.

use expect_test::expect;
use test_utils::parse_check;

#[test]
fn test_multiple_expressions() {
    parse_check(
        r"
          let a: int = 32
          let b: int = -123
          a
          b
          2 + 43 == 45
        ",
        expect![[r#"
            Program {
                body: Block {
                    exprs: [
                        Let {
                            id: "a",
                            type_reference: "int",
                            init_expr: Int {
                                value: 32,
                            },
                        },
                        Let {
                            id: "b",
                            type_reference: "int",
                            init_expr: UnaryExpr {
                                kind: Negative,
                                operand: Int {
                                    value: 123,
                                },
                            },
                        },
                        Id {
                            value: "a",
                        },
                        Id {
                            value: "b",
                        },
                        BinaryExpr {
                            kind: EqualsEquals,
                            operand_1: BinaryExpr {
                                kind: Plus,
                                operand_1: Int {
                                    value: 2,
                                },
                                operand_2: Int {
                                    value: 43,
                                },
                            },
                            operand_2: Int {
                                value: 45,
                            },
                        },
                    ],
                },
            }"#]],
    );

    parse_check(
        r"
          let a: int = 32 let b: int = -123
          a b 2 +
          43 ==
          45
        ",
        expect![[r#"
            Program {
                body: Block {
                    exprs: [
                        Let {
                            id: "a",
                            type_reference: "int",
                            init_expr: Int {
                                value: 32,
                            },
                        },
                        Let {
                            id: "b",
                            type_reference: "int",
                            init_expr: UnaryExpr {
                                kind: Negative,
                                operand: Int {
                                    value: 123,
                                },
                            },
                        },
                        Id {
                            value: "a",
                        },
                        Id {
                            value: "b",
                        },
                        BinaryExpr {
                            kind: EqualsEquals,
                            operand_1: BinaryExpr {
                                kind: Plus,
                                operand_1: Int {
                                    value: 2,
                                },
                                operand_2: Int {
                                    value: 43,
                                },
                            },
                            operand_2: Int {
                                value: 45,
                            },
                        },
                    ],
                },
            }"#]],
    )
}
