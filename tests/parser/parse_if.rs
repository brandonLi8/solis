// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Tests for parsing if expressions

use expect_test::expect;
use test_utils::parse_check;

#[test]
fn parse_if_no_else() {
    parse_check(
        r"
          if a < b {
            1 + 2
            2 + 3
          }
        ",
        expect![[r#"
            Program {
                functions: [],
                body: Block {
                    exprs: [
                        If {
                            condition: BinaryExpr {
                                kind: LessThan,
                                operand_1: Id {
                                    value: "a",
                                    position: 14..15,
                                },
                                operand_2: Id {
                                    value: "b",
                                    position: 18..19,
                                },
                                operator_position: 16..17,
                            },
                            then_block: Block {
                                exprs: [
                                    BinaryExpr {
                                        kind: Plus,
                                        operand_1: Int {
                                            value: 1,
                                        },
                                        operand_2: Int {
                                            value: 2,
                                        },
                                        operator_position: 36..37,
                                    },
                                    BinaryExpr {
                                        kind: Plus,
                                        operand_1: Int {
                                            value: 2,
                                        },
                                        operand_2: Int {
                                            value: 3,
                                        },
                                        operator_position: 54..55,
                                    },
                                ],
                            },
                            else_block: None,
                            if_position: 11..13,
                        },
                    ],
                },
            }"#]],
    );
}

#[test]
fn parse_if_empty() {
    parse_check(
        r"
          if a < b {
          }
        ",
        expect![[r#"
            Program {
                functions: [],
                body: Block {
                    exprs: [
                        If {
                            condition: BinaryExpr {
                                kind: LessThan,
                                operand_1: Id {
                                    value: "a",
                                    position: 14..15,
                                },
                                operand_2: Id {
                                    value: "b",
                                    position: 18..19,
                                },
                                operator_position: 16..17,
                            },
                            then_block: Block {
                                exprs: [],
                            },
                            else_block: None,
                            if_position: 11..13,
                        },
                    ],
                },
            }"#]],
    );
}

#[test]
fn parse_if_else_basic() {
    parse_check(
        r"
          if a < b {
            1 + 2
            2 + 3
          }
          else {
            4 + 5
          }
        ",
        expect![[r#"
            Program {
                functions: [],
                body: Block {
                    exprs: [
                        If {
                            condition: BinaryExpr {
                                kind: LessThan,
                                operand_1: Id {
                                    value: "a",
                                    position: 14..15,
                                },
                                operand_2: Id {
                                    value: "b",
                                    position: 18..19,
                                },
                                operator_position: 16..17,
                            },
                            then_block: Block {
                                exprs: [
                                    BinaryExpr {
                                        kind: Plus,
                                        operand_1: Int {
                                            value: 1,
                                        },
                                        operand_2: Int {
                                            value: 2,
                                        },
                                        operator_position: 36..37,
                                    },
                                    BinaryExpr {
                                        kind: Plus,
                                        operand_1: Int {
                                            value: 2,
                                        },
                                        operand_2: Int {
                                            value: 3,
                                        },
                                        operator_position: 54..55,
                                    },
                                ],
                            },
                            else_block: Some(
                                Block {
                                    exprs: [
                                        BinaryExpr {
                                            kind: Plus,
                                            operand_1: Int {
                                                value: 4,
                                            },
                                            operand_2: Int {
                                                value: 5,
                                            },
                                            operator_position: 101..102,
                                        },
                                    ],
                                },
                            ),
                            if_position: 11..13,
                        },
                    ],
                },
            }"#]],
    );
}

#[test]
fn parse_if_else_chain() {
    parse_check(
        r"
          let a: int = if a < b {
            1 + 2;
            2 + 3
          }
          else if c {
            2 + 3
          }
          else if d {
            1 + 2
          }
          else if c {
            a
          }
          else {
            2 + 1
          }
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
                            init_expr: If {
                                condition: BinaryExpr {
                                    kind: LessThan,
                                    operand_1: Id {
                                        value: "a",
                                        position: 27..28,
                                    },
                                    operand_2: Id {
                                        value: "b",
                                        position: 31..32,
                                    },
                                    operator_position: 29..30,
                                },
                                then_block: Block {
                                    exprs: [
                                        BinaryExpr {
                                            kind: Plus,
                                            operand_1: Int {
                                                value: 1,
                                            },
                                            operand_2: Int {
                                                value: 2,
                                            },
                                            operator_position: 49..50,
                                        },
                                        BinaryExpr {
                                            kind: Plus,
                                            operand_1: Int {
                                                value: 2,
                                            },
                                            operand_2: Int {
                                                value: 3,
                                            },
                                            operator_position: 68..69,
                                        },
                                    ],
                                },
                                else_block: Some(
                                    Block {
                                        exprs: [
                                            If {
                                                condition: Id {
                                                    value: "c",
                                                    position: 102..103,
                                                },
                                                then_block: Block {
                                                    exprs: [
                                                        BinaryExpr {
                                                            kind: Plus,
                                                            operand_1: Int {
                                                                value: 2,
                                                            },
                                                            operand_2: Int {
                                                                value: 3,
                                                            },
                                                            operator_position: 120..121,
                                                        },
                                                    ],
                                                },
                                                else_block: Some(
                                                    Block {
                                                        exprs: [
                                                            If {
                                                                condition: Id {
                                                                    value: "d",
                                                                    position: 154..155,
                                                                },
                                                                then_block: Block {
                                                                    exprs: [
                                                                        BinaryExpr {
                                                                            kind: Plus,
                                                                            operand_1: Int {
                                                                                value: 1,
                                                                            },
                                                                            operand_2: Int {
                                                                                value: 2,
                                                                            },
                                                                            operator_position: 172..173,
                                                                        },
                                                                    ],
                                                                },
                                                                else_block: Some(
                                                                    Block {
                                                                        exprs: [
                                                                            If {
                                                                                condition: Id {
                                                                                    value: "c",
                                                                                    position: 206..207,
                                                                                },
                                                                                then_block: Block {
                                                                                    exprs: [
                                                                                        Id {
                                                                                            value: "a",
                                                                                            position: 222..223,
                                                                                        },
                                                                                    ],
                                                                                },
                                                                                else_block: Some(
                                                                                    Block {
                                                                                        exprs: [
                                                                                            BinaryExpr {
                                                                                                kind: Plus,
                                                                                                operand_1: Int {
                                                                                                    value: 2,
                                                                                                },
                                                                                                operand_2: Int {
                                                                                                    value: 1,
                                                                                                },
                                                                                                operator_position: 267..268,
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ),
                                                                                if_position: 203..205,
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                if_position: 151..153,
                                                            },
                                                        ],
                                                    },
                                                ),
                                                if_position: 99..101,
                                            },
                                        ],
                                    },
                                ),
                                if_position: 24..26,
                            },
                            init_expr_position: 24..282,
                        },
                    ],
                },
            }"#]],
    );
}
