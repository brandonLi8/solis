// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Tests parser correctness of operand associativity.

use expect_test::expect;
use test_utils::parse_check;

#[test]
fn test_arithmetic_left_associative_1() {
    parse_check(
        r"
          1 + 2 + 3
          1 - 2 - 3
          (1 + 2) + 3
          1 - (2 + 3)
          1 / 2 * 3
          1 * 2 % 3
          1 % 2 / 3
        ",
        expect![[r#"
            Program {
                body: Block {
                    exprs: [
                        Expr {
                            kind: BinaryExpr {
                                kind: Plus,
                                operand_1: Expr {
                                    kind: BinaryExpr {
                                        kind: Plus,
                                        operand_1: Expr {
                                            kind: Int {
                                                value: 1,
                                            },
                                            position: 11..12,
                                        },
                                        operand_2: Expr {
                                            kind: Int {
                                                value: 2,
                                            },
                                            position: 15..16,
                                        },
                                    },
                                    position: 13..14,
                                },
                                operand_2: Expr {
                                    kind: Int {
                                        value: 3,
                                    },
                                    position: 19..20,
                                },
                            },
                            position: 17..18,
                        },
                        Expr {
                            kind: BinaryExpr {
                                kind: Minus,
                                operand_1: Expr {
                                    kind: BinaryExpr {
                                        kind: Minus,
                                        operand_1: Expr {
                                            kind: Int {
                                                value: 1,
                                            },
                                            position: 31..32,
                                        },
                                        operand_2: Expr {
                                            kind: Int {
                                                value: 2,
                                            },
                                            position: 35..36,
                                        },
                                    },
                                    position: 33..34,
                                },
                                operand_2: Expr {
                                    kind: Int {
                                        value: 3,
                                    },
                                    position: 39..40,
                                },
                            },
                            position: 37..38,
                        },
                        Expr {
                            kind: BinaryExpr {
                                kind: Plus,
                                operand_1: Expr {
                                    kind: BinaryExpr {
                                        kind: Plus,
                                        operand_1: Expr {
                                            kind: Int {
                                                value: 1,
                                            },
                                            position: 52..53,
                                        },
                                        operand_2: Expr {
                                            kind: Int {
                                                value: 2,
                                            },
                                            position: 56..57,
                                        },
                                    },
                                    position: 54..55,
                                },
                                operand_2: Expr {
                                    kind: Int {
                                        value: 3,
                                    },
                                    position: 61..62,
                                },
                            },
                            position: 59..60,
                        },
                        Expr {
                            kind: BinaryExpr {
                                kind: Minus,
                                operand_1: Expr {
                                    kind: Int {
                                        value: 1,
                                    },
                                    position: 73..74,
                                },
                                operand_2: Expr {
                                    kind: BinaryExpr {
                                        kind: Plus,
                                        operand_1: Expr {
                                            kind: Int {
                                                value: 2,
                                            },
                                            position: 78..79,
                                        },
                                        operand_2: Expr {
                                            kind: Int {
                                                value: 3,
                                            },
                                            position: 82..83,
                                        },
                                    },
                                    position: 80..81,
                                },
                            },
                            position: 75..76,
                        },
                        Expr {
                            kind: BinaryExpr {
                                kind: Times,
                                operand_1: Expr {
                                    kind: BinaryExpr {
                                        kind: Divide,
                                        operand_1: Expr {
                                            kind: Int {
                                                value: 1,
                                            },
                                            position: 95..96,
                                        },
                                        operand_2: Expr {
                                            kind: Int {
                                                value: 2,
                                            },
                                            position: 99..100,
                                        },
                                    },
                                    position: 97..98,
                                },
                                operand_2: Expr {
                                    kind: Int {
                                        value: 3,
                                    },
                                    position: 103..104,
                                },
                            },
                            position: 101..102,
                        },
                        Expr {
                            kind: BinaryExpr {
                                kind: Mod,
                                operand_1: Expr {
                                    kind: BinaryExpr {
                                        kind: Times,
                                        operand_1: Expr {
                                            kind: Int {
                                                value: 1,
                                            },
                                            position: 115..116,
                                        },
                                        operand_2: Expr {
                                            kind: Int {
                                                value: 2,
                                            },
                                            position: 119..120,
                                        },
                                    },
                                    position: 117..118,
                                },
                                operand_2: Expr {
                                    kind: Int {
                                        value: 3,
                                    },
                                    position: 123..124,
                                },
                            },
                            position: 121..122,
                        },
                        Expr {
                            kind: BinaryExpr {
                                kind: Divide,
                                operand_1: Expr {
                                    kind: BinaryExpr {
                                        kind: Mod,
                                        operand_1: Expr {
                                            kind: Int {
                                                value: 1,
                                            },
                                            position: 135..136,
                                        },
                                        operand_2: Expr {
                                            kind: Int {
                                                value: 2,
                                            },
                                            position: 139..140,
                                        },
                                    },
                                    position: 137..138,
                                },
                                operand_2: Expr {
                                    kind: Int {
                                        value: 3,
                                    },
                                    position: 143..144,
                                },
                            },
                            position: 141..142,
                        },
                    ],
                },
            }"#]],
    );
}

#[test]
fn test_arithmetic_left_associative_2() {
    parse_check(
        "let a: int = 32 - 2 * (3. + ((4))) / 5 - 3 % 2.",
        expect![[r#"
            Program {
                body: Block {
                    exprs: [
                        Expr {
                            kind: Let {
                                id: "a",
                                type_reference: Int,
                                init_expr: Expr {
                                    kind: BinaryExpr {
                                        kind: Minus,
                                        operand_1: Expr {
                                            kind: BinaryExpr {
                                                kind: Minus,
                                                operand_1: Expr {
                                                    kind: Int {
                                                        value: 32,
                                                    },
                                                    position: 13..15,
                                                },
                                                operand_2: Expr {
                                                    kind: BinaryExpr {
                                                        kind: Divide,
                                                        operand_1: Expr {
                                                            kind: BinaryExpr {
                                                                kind: Times,
                                                                operand_1: Expr {
                                                                    kind: Int {
                                                                        value: 2,
                                                                    },
                                                                    position: 18..19,
                                                                },
                                                                operand_2: Expr {
                                                                    kind: BinaryExpr {
                                                                        kind: Plus,
                                                                        operand_1: Expr {
                                                                            kind: Float {
                                                                                value: 3.0,
                                                                            },
                                                                            position: 23..25,
                                                                        },
                                                                        operand_2: Expr {
                                                                            kind: Int {
                                                                                value: 4,
                                                                            },
                                                                            position: 30..31,
                                                                        },
                                                                    },
                                                                    position: 26..27,
                                                                },
                                                            },
                                                            position: 20..21,
                                                        },
                                                        operand_2: Expr {
                                                            kind: Int {
                                                                value: 5,
                                                            },
                                                            position: 37..38,
                                                        },
                                                    },
                                                    position: 35..36,
                                                },
                                            },
                                            position: 16..17,
                                        },
                                        operand_2: Expr {
                                            kind: BinaryExpr {
                                                kind: Mod,
                                                operand_1: Expr {
                                                    kind: Int {
                                                        value: 3,
                                                    },
                                                    position: 41..42,
                                                },
                                                operand_2: Expr {
                                                    kind: Float {
                                                        value: 2.0,
                                                    },
                                                    position: 45..47,
                                                },
                                            },
                                            position: 43..44,
                                        },
                                    },
                                    position: 39..40,
                                },
                            },
                            position: 7..10,
                        },
                    ],
                },
            }"#]],
    );
}
