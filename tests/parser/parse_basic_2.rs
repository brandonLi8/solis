// Copyright © 2022-2023 Brandon Li. All rights reserved.

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
          true == false
        ",
        expect![[r#"
            Program {
                functions: [],
                body: Block {
                    exprs: [
                        Let {
                            id: "a",
                            id_position: 15..16,
                            type_reference: Int,
                            init_expr: Int {
                                value: 32,
                            },
                        },
                        Let {
                            id: "b",
                            id_position: 41..42,
                            type_reference: Int,
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
                        BinaryExpr {
                            kind: EqualsEquals,
                            operand_1: Bool {
                                value: true,
                            },
                            operand_2: Bool {
                                value: false,
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
                functions: [],
                body: Block {
                    exprs: [
                        Let {
                            id: "a",
                            id_position: 15..16,
                            type_reference: Int,
                            init_expr: Int {
                                value: 32,
                            },
                        },
                        Let {
                            id: "b",
                            id_position: 31..32,
                            type_reference: Int,
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
}
