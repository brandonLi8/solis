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
                                operator_position: 50..51,
                            },
                        },
                        Id {
                            value: "a",
                            position: 65..66,
                        },
                        Id {
                            value: "b",
                            position: 77..78,
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
                                operator_position: 91..92,
                            },
                            operand_2: Int {
                                value: 45,
                            },
                            operator_position: 96..98,
                        },
                        BinaryExpr {
                            kind: EqualsEquals,
                            operand_1: Bool {
                                value: true,
                            },
                            operand_2: Bool {
                                value: false,
                            },
                            operator_position: 117..119,
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
                                operator_position: 40..41,
                            },
                        },
                        Id {
                            value: "a",
                            position: 55..56,
                        },
                        Id {
                            value: "b",
                            position: 57..58,
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
                                operator_position: 61..62,
                            },
                            operand_2: Int {
                                value: 45,
                            },
                            operator_position: 76..78,
                        },
                    ],
                },
            }"#]],
    );
}
