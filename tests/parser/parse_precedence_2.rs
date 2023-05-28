// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Tests parser correctness of operand precedence.

use expect_test::expect;
use test_utils::parse_check;

#[test]
fn test_comparison_precedence_1() {
    parse_check(
        r"
          1 < 2 * 3
          1 * 2 > 3
          (1 + 2) <= 3
          1 <= (2 + 3)
          1 < 2 == true
          false == (1 < 2)
        ",
        expect![[r#"
            Program {
                functions: [],
                body: Block {
                    exprs: [
                        BinaryExpr {
                            kind: LessThan,
                            operand_1: Int {
                                value: 1,
                            },
                            operand_2: BinaryExpr {
                                kind: Times,
                                operand_1: Int {
                                    value: 2,
                                },
                                operand_2: Int {
                                    value: 3,
                                },
                                operator_position: 17..18,
                            },
                            operator_position: 13..14,
                        },
                        BinaryExpr {
                            kind: MoreThan,
                            operand_1: BinaryExpr {
                                kind: Times,
                                operand_1: Int {
                                    value: 1,
                                },
                                operand_2: Int {
                                    value: 2,
                                },
                                operator_position: 33..34,
                            },
                            operand_2: Int {
                                value: 3,
                            },
                            operator_position: 37..38,
                        },
                        BinaryExpr {
                            kind: LessThanOrEquals,
                            operand_1: BinaryExpr {
                                kind: Plus,
                                operand_1: Int {
                                    value: 1,
                                },
                                operand_2: Int {
                                    value: 2,
                                },
                                operator_position: 54..55,
                            },
                            operand_2: Int {
                                value: 3,
                            },
                            operator_position: 59..61,
                        },
                        BinaryExpr {
                            kind: LessThanOrEquals,
                            operand_1: Int {
                                value: 1,
                            },
                            operand_2: BinaryExpr {
                                kind: Plus,
                                operand_1: Int {
                                    value: 2,
                                },
                                operand_2: Int {
                                    value: 3,
                                },
                                operator_position: 82..83,
                            },
                            operator_position: 76..78,
                        },
                        BinaryExpr {
                            kind: EqualsEquals,
                            operand_1: BinaryExpr {
                                kind: LessThan,
                                operand_1: Int {
                                    value: 1,
                                },
                                operand_2: Int {
                                    value: 2,
                                },
                                operator_position: 99..100,
                            },
                            operand_2: Bool {
                                value: true,
                            },
                            operator_position: 103..105,
                        },
                        BinaryExpr {
                            kind: EqualsEquals,
                            operand_1: Bool {
                                value: false,
                            },
                            operand_2: BinaryExpr {
                                kind: LessThan,
                                operand_1: Int {
                                    value: 1,
                                },
                                operand_2: Int {
                                    value: 2,
                                },
                                operator_position: 133..134,
                            },
                            operator_position: 127..129,
                        },
                    ],
                },
            }"#]],
    );
}

#[test]
fn test_comparison_precedence_2() {
    parse_check(
        r"
          let a: bool = z + y < z
          let b: bool = 1 != 2 / 3
          let c: () = 1 + (2 >= 3) # semantics wrong
        ",
        expect![[r#"
            Program {
                functions: [],
                body: Block {
                    exprs: [
                        Let {
                            id: "a",
                            id_position: 15..16,
                            type_reference: Bool,
                            init_expr: BinaryExpr {
                                kind: LessThan,
                                operand_1: BinaryExpr {
                                    kind: Plus,
                                    operand_1: Id {
                                        value: "z",
                                        position: 25..26,
                                    },
                                    operand_2: Id {
                                        value: "y",
                                        position: 29..30,
                                    },
                                    operator_position: 27..28,
                                },
                                operand_2: Id {
                                    value: "z",
                                    position: 33..34,
                                },
                                operator_position: 31..32,
                            },
                        },
                        Let {
                            id: "b",
                            id_position: 49..50,
                            type_reference: Bool,
                            init_expr: BinaryExpr {
                                kind: NotEquals,
                                operand_1: Int {
                                    value: 1,
                                },
                                operand_2: BinaryExpr {
                                    kind: Divide,
                                    operand_1: Int {
                                        value: 2,
                                    },
                                    operand_2: Int {
                                        value: 3,
                                    },
                                    operator_position: 66..67,
                                },
                                operator_position: 61..63,
                            },
                        },
                        Let {
                            id: "c",
                            id_position: 84..85,
                            type_reference: Unit,
                            init_expr: BinaryExpr {
                                kind: Plus,
                                operand_1: Int {
                                    value: 1,
                                },
                                operand_2: BinaryExpr {
                                    kind: MoreThanOrEquals,
                                    operand_1: Int {
                                        value: 2,
                                    },
                                    operand_2: Int {
                                        value: 3,
                                    },
                                    operator_position: 99..101,
                                },
                                operator_position: 94..95,
                            },
                        },
                    ],
                },
            }"#]],
    );
}
