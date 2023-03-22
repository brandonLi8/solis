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
                        Expr {
                            kind: If {
                                condition: Expr {
                                    kind: BinaryExpr {
                                        kind: LessThan,
                                        operand_1: Expr {
                                            kind: Id {
                                                value: "a",
                                            },
                                            position: 14..15,
                                        },
                                        operand_2: Expr {
                                            kind: Id {
                                                value: "b",
                                            },
                                            position: 18..19,
                                        },
                                    },
                                    position: 16..17,
                                },
                                then_block: Block {
                                    exprs: [
                                        Expr {
                                            kind: BinaryExpr {
                                                kind: Plus,
                                                operand_1: Expr {
                                                    kind: Int {
                                                        value: 1,
                                                    },
                                                    position: 34..35,
                                                },
                                                operand_2: Expr {
                                                    kind: Int {
                                                        value: 2,
                                                    },
                                                    position: 38..39,
                                                },
                                            },
                                            position: 36..37,
                                        },
                                        Expr {
                                            kind: BinaryExpr {
                                                kind: Plus,
                                                operand_1: Expr {
                                                    kind: Int {
                                                        value: 2,
                                                    },
                                                    position: 52..53,
                                                },
                                                operand_2: Expr {
                                                    kind: Int {
                                                        value: 3,
                                                    },
                                                    position: 56..57,
                                                },
                                            },
                                            position: 54..55,
                                        },
                                    ],
                                },
                                else_block: None,
                            },
                            position: 11..13,
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
                        Expr {
                            kind: If {
                                condition: Expr {
                                    kind: BinaryExpr {
                                        kind: LessThan,
                                        operand_1: Expr {
                                            kind: Id {
                                                value: "a",
                                            },
                                            position: 14..15,
                                        },
                                        operand_2: Expr {
                                            kind: Id {
                                                value: "b",
                                            },
                                            position: 18..19,
                                        },
                                    },
                                    position: 16..17,
                                },
                                then_block: Block {
                                    exprs: [],
                                },
                                else_block: None,
                            },
                            position: 11..13,
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
                        Expr {
                            kind: If {
                                condition: Expr {
                                    kind: BinaryExpr {
                                        kind: LessThan,
                                        operand_1: Expr {
                                            kind: Id {
                                                value: "a",
                                            },
                                            position: 14..15,
                                        },
                                        operand_2: Expr {
                                            kind: Id {
                                                value: "b",
                                            },
                                            position: 18..19,
                                        },
                                    },
                                    position: 16..17,
                                },
                                then_block: Block {
                                    exprs: [
                                        Expr {
                                            kind: BinaryExpr {
                                                kind: Plus,
                                                operand_1: Expr {
                                                    kind: Int {
                                                        value: 1,
                                                    },
                                                    position: 34..35,
                                                },
                                                operand_2: Expr {
                                                    kind: Int {
                                                        value: 2,
                                                    },
                                                    position: 38..39,
                                                },
                                            },
                                            position: 36..37,
                                        },
                                        Expr {
                                            kind: BinaryExpr {
                                                kind: Plus,
                                                operand_1: Expr {
                                                    kind: Int {
                                                        value: 2,
                                                    },
                                                    position: 52..53,
                                                },
                                                operand_2: Expr {
                                                    kind: Int {
                                                        value: 3,
                                                    },
                                                    position: 56..57,
                                                },
                                            },
                                            position: 54..55,
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
                                                        kind: Int {
                                                            value: 4,
                                                        },
                                                        position: 99..100,
                                                    },
                                                    operand_2: Expr {
                                                        kind: Int {
                                                            value: 5,
                                                        },
                                                        position: 103..104,
                                                    },
                                                },
                                                position: 101..102,
                                            },
                                        ],
                                    },
                                ),
                            },
                            position: 11..13,
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
                        Expr {
                            kind: Let {
                                id: "a",
                                type_reference: Int,
                                init_expr: Expr {
                                    kind: If {
                                        condition: Expr {
                                            kind: BinaryExpr {
                                                kind: LessThan,
                                                operand_1: Expr {
                                                    kind: Id {
                                                        value: "a",
                                                    },
                                                    position: 27..28,
                                                },
                                                operand_2: Expr {
                                                    kind: Id {
                                                        value: "b",
                                                    },
                                                    position: 31..32,
                                                },
                                            },
                                            position: 29..30,
                                        },
                                        then_block: Block {
                                            exprs: [
                                                Expr {
                                                    kind: BinaryExpr {
                                                        kind: Plus,
                                                        operand_1: Expr {
                                                            kind: Int {
                                                                value: 1,
                                                            },
                                                            position: 47..48,
                                                        },
                                                        operand_2: Expr {
                                                            kind: Int {
                                                                value: 2,
                                                            },
                                                            position: 51..52,
                                                        },
                                                    },
                                                    position: 49..50,
                                                },
                                                Expr {
                                                    kind: BinaryExpr {
                                                        kind: Plus,
                                                        operand_1: Expr {
                                                            kind: Int {
                                                                value: 2,
                                                            },
                                                            position: 66..67,
                                                        },
                                                        operand_2: Expr {
                                                            kind: Int {
                                                                value: 3,
                                                            },
                                                            position: 70..71,
                                                        },
                                                    },
                                                    position: 68..69,
                                                },
                                            ],
                                        },
                                        else_block: Some(
                                            Block {
                                                exprs: [
                                                    Expr {
                                                        kind: If {
                                                            condition: Expr {
                                                                kind: Id {
                                                                    value: "c",
                                                                },
                                                                position: 102..103,
                                                            },
                                                            then_block: Block {
                                                                exprs: [
                                                                    Expr {
                                                                        kind: BinaryExpr {
                                                                            kind: Plus,
                                                                            operand_1: Expr {
                                                                                kind: Int {
                                                                                    value: 2,
                                                                                },
                                                                                position: 118..119,
                                                                            },
                                                                            operand_2: Expr {
                                                                                kind: Int {
                                                                                    value: 3,
                                                                                },
                                                                                position: 122..123,
                                                                            },
                                                                        },
                                                                        position: 120..121,
                                                                    },
                                                                ],
                                                            },
                                                            else_block: Some(
                                                                Block {
                                                                    exprs: [
                                                                        Expr {
                                                                            kind: If {
                                                                                condition: Expr {
                                                                                    kind: Id {
                                                                                        value: "d",
                                                                                    },
                                                                                    position: 154..155,
                                                                                },
                                                                                then_block: Block {
                                                                                    exprs: [
                                                                                        Expr {
                                                                                            kind: BinaryExpr {
                                                                                                kind: Plus,
                                                                                                operand_1: Expr {
                                                                                                    kind: Int {
                                                                                                        value: 1,
                                                                                                    },
                                                                                                    position: 170..171,
                                                                                                },
                                                                                                operand_2: Expr {
                                                                                                    kind: Int {
                                                                                                        value: 2,
                                                                                                    },
                                                                                                    position: 174..175,
                                                                                                },
                                                                                            },
                                                                                            position: 172..173,
                                                                                        },
                                                                                    ],
                                                                                },
                                                                                else_block: Some(
                                                                                    Block {
                                                                                        exprs: [
                                                                                            Expr {
                                                                                                kind: If {
                                                                                                    condition: Expr {
                                                                                                        kind: Id {
                                                                                                            value: "c",
                                                                                                        },
                                                                                                        position: 206..207,
                                                                                                    },
                                                                                                    then_block: Block {
                                                                                                        exprs: [
                                                                                                            Expr {
                                                                                                                kind: Id {
                                                                                                                    value: "a",
                                                                                                                },
                                                                                                                position: 222..223,
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
                                                                                                                            kind: Int {
                                                                                                                                value: 2,
                                                                                                                            },
                                                                                                                            position: 265..266,
                                                                                                                        },
                                                                                                                        operand_2: Expr {
                                                                                                                            kind: Int {
                                                                                                                                value: 1,
                                                                                                                            },
                                                                                                                            position: 269..270,
                                                                                                                        },
                                                                                                                    },
                                                                                                                    position: 267..268,
                                                                                                                },
                                                                                                            ],
                                                                                                        },
                                                                                                    ),
                                                                                                },
                                                                                                position: 203..205,
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ),
                                                                            },
                                                                            position: 151..153,
                                                                        },
                                                                    ],
                                                                },
                                                            ),
                                                        },
                                                        position: 99..101,
                                                    },
                                                ],
                                            },
                                        ),
                                    },
                                    position: 24..26,
                                },
                            },
                            position: 18..21,
                        },
                    ],
                },
            }"#]],
    );
}
