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
                                },
                                operand_2: Id {
                                    value: "b",
                                },
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
                                    },
                                    BinaryExpr {
                                        kind: Plus,
                                        operand_1: Int {
                                            value: 2,
                                        },
                                        operand_2: Int {
                                            value: 3,
                                        },
                                    },
                                ],
                            },
                            else_block: None,
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
                                },
                                operand_2: Id {
                                    value: "b",
                                },
                            },
                            then_block: Block {
                                exprs: [],
                            },
                            else_block: None,
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
                                },
                                operand_2: Id {
                                    value: "b",
                                },
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
                                    },
                                    BinaryExpr {
                                        kind: Plus,
                                        operand_1: Int {
                                            value: 2,
                                        },
                                        operand_2: Int {
                                            value: 3,
                                        },
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
                                        },
                                    ],
                                },
                            ),
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
                            type_reference: Int,
                            init_expr: If {
                                condition: BinaryExpr {
                                    kind: LessThan,
                                    operand_1: Id {
                                        value: "a",
                                    },
                                    operand_2: Id {
                                        value: "b",
                                    },
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
                                        },
                                        BinaryExpr {
                                            kind: Plus,
                                            operand_1: Int {
                                                value: 2,
                                            },
                                            operand_2: Int {
                                                value: 3,
                                            },
                                        },
                                    ],
                                },
                                else_block: Some(
                                    Block {
                                        exprs: [
                                            If {
                                                condition: Id {
                                                    value: "c",
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
                                                        },
                                                    ],
                                                },
                                                else_block: Some(
                                                    Block {
                                                        exprs: [
                                                            If {
                                                                condition: Id {
                                                                    value: "d",
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
                                                                        },
                                                                    ],
                                                                },
                                                                else_block: Some(
                                                                    Block {
                                                                        exprs: [
                                                                            If {
                                                                                condition: Id {
                                                                                    value: "c",
                                                                                },
                                                                                then_block: Block {
                                                                                    exprs: [
                                                                                        Id {
                                                                                            value: "a",
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
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ),
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                ),
                                            },
                                        ],
                                    },
                                ),
                            },
                        },
                    ],
                },
            }"#]],
    );
}
