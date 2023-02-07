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
          true == false
        ",
        expect![[r#"
            Program {
                body: Block {
                    exprs: [
                        Expr {
                            kind: Let {
                                id: "a",
                                type_reference: "int",
                                init_expr: Expr {
                                    kind: Int {
                                        value: 32,
                                    },
                                    position: 24..26,
                                },
                            },
                            position: 18..21,
                        },
                        Expr {
                            kind: Let {
                                id: "b",
                                type_reference: "int",
                                init_expr: Expr {
                                    kind: UnaryExpr {
                                        kind: Negative,
                                        operand: Expr {
                                            kind: Int {
                                                value: 123,
                                            },
                                            position: 51..54,
                                        },
                                    },
                                    position: 50..51,
                                },
                            },
                            position: 44..47,
                        },
                        Expr {
                            kind: Id {
                                value: "a",
                            },
                            position: 65..66,
                        },
                        Expr {
                            kind: Id {
                                value: "b",
                            },
                            position: 77..78,
                        },
                        Expr {
                            kind: BinaryExpr {
                                kind: EqualsEquals,
                                operand_1: Expr {
                                    kind: BinaryExpr {
                                        kind: Plus,
                                        operand_1: Expr {
                                            kind: Int {
                                                value: 2,
                                            },
                                            position: 89..90,
                                        },
                                        operand_2: Expr {
                                            kind: Int {
                                                value: 43,
                                            },
                                            position: 93..95,
                                        },
                                    },
                                    position: 91..92,
                                },
                                operand_2: Expr {
                                    kind: Int {
                                        value: 45,
                                    },
                                    position: 99..101,
                                },
                            },
                            position: 96..98,
                        },
                        Expr {
                            kind: BinaryExpr {
                                kind: EqualsEquals,
                                operand_1: Expr {
                                    kind: Bool {
                                        value: true,
                                    },
                                    position: 112..116,
                                },
                                operand_2: Expr {
                                    kind: Bool {
                                        value: false,
                                    },
                                    position: 120..125,
                                },
                            },
                            position: 117..119,
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
                        Expr {
                            kind: Let {
                                id: "a",
                                type_reference: "int",
                                init_expr: Expr {
                                    kind: Int {
                                        value: 32,
                                    },
                                    position: 24..26,
                                },
                            },
                            position: 18..21,
                        },
                        Expr {
                            kind: Let {
                                id: "b",
                                type_reference: "int",
                                init_expr: Expr {
                                    kind: UnaryExpr {
                                        kind: Negative,
                                        operand: Expr {
                                            kind: Int {
                                                value: 123,
                                            },
                                            position: 41..44,
                                        },
                                    },
                                    position: 40..41,
                                },
                            },
                            position: 34..37,
                        },
                        Expr {
                            kind: Id {
                                value: "a",
                            },
                            position: 55..56,
                        },
                        Expr {
                            kind: Id {
                                value: "b",
                            },
                            position: 57..58,
                        },
                        Expr {
                            kind: BinaryExpr {
                                kind: EqualsEquals,
                                operand_1: Expr {
                                    kind: BinaryExpr {
                                        kind: Plus,
                                        operand_1: Expr {
                                            kind: Int {
                                                value: 2,
                                            },
                                            position: 59..60,
                                        },
                                        operand_2: Expr {
                                            kind: Int {
                                                value: 43,
                                            },
                                            position: 73..75,
                                        },
                                    },
                                    position: 61..62,
                                },
                                operand_2: Expr {
                                    kind: Int {
                                        value: 45,
                                    },
                                    position: 89..91,
                                },
                            },
                            position: 76..78,
                        },
                    ],
                },
            }"#]],
    )
}
