// Copyright © 2022 Brandon Li. All rights reserved.

//! Tests parser correctness of operand precedence.

use expect_test::expect;
use test_utils::parse_check;

#[test]
fn test_arithmetic_precedence() {
    parse_check(
        r"
          1 + 2 * 3
          1 * 2 + 3
          (1 + 2) * 3
          1 * (2 + 3)
          1 / 2 * 3
          let a: int = 1 - 2 % 3
          let b: int = 1 % 2 - 3
        ",
        expect![[r#"
            Program {
                body: Block {
                    exprs: [
                        Expr {
                            kind: BinaryExpr {
                                kind: Plus,
                                operand_1: Expr {
                                    kind: Int {
                                        value: 1,
                                    },
                                    position: 11..12,
                                },
                                operand_2: Expr {
                                    kind: BinaryExpr {
                                        kind: Times,
                                        operand_1: Expr {
                                            kind: Int {
                                                value: 2,
                                            },
                                            position: 15..16,
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
                            },
                            position: 13..14,
                        },
                        Expr {
                            kind: BinaryExpr {
                                kind: Plus,
                                operand_1: Expr {
                                    kind: BinaryExpr {
                                        kind: Times,
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
                                kind: Times,
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
                                kind: Times,
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
                            kind: Let {
                                id: "a",
                                type_reference: "int",
                                init_expr: Expr {
                                    kind: BinaryExpr {
                                        kind: Minus,
                                        operand_1: Expr {
                                            kind: Int {
                                                value: 1,
                                            },
                                            position: 128..129,
                                        },
                                        operand_2: Expr {
                                            kind: BinaryExpr {
                                                kind: Mod,
                                                operand_1: Expr {
                                                    kind: Int {
                                                        value: 2,
                                                    },
                                                    position: 132..133,
                                                },
                                                operand_2: Expr {
                                                    kind: Int {
                                                        value: 3,
                                                    },
                                                    position: 136..137,
                                                },
                                            },
                                            position: 134..135,
                                        },
                                    },
                                    position: 130..131,
                                },
                            },
                            position: 122..125,
                        },
                        Expr {
                            kind: Let {
                                id: "b",
                                type_reference: "int",
                                init_expr: Expr {
                                    kind: BinaryExpr {
                                        kind: Minus,
                                        operand_1: Expr {
                                            kind: BinaryExpr {
                                                kind: Mod,
                                                operand_1: Expr {
                                                    kind: Int {
                                                        value: 1,
                                                    },
                                                    position: 161..162,
                                                },
                                                operand_2: Expr {
                                                    kind: Int {
                                                        value: 2,
                                                    },
                                                    position: 165..166,
                                                },
                                            },
                                            position: 163..164,
                                        },
                                        operand_2: Expr {
                                            kind: Int {
                                                value: 3,
                                            },
                                            position: 169..170,
                                        },
                                    },
                                    position: 167..168,
                                },
                            },
                            position: 155..158,
                        },
                    ],
                },
            }"#]],
    )
}
