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
                        Expr {
                            kind: BinaryExpr {
                                kind: LessThan,
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
                                kind: MoreThan,
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
                                kind: LessThanOrEquals,
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
                                    position: 62..63,
                                },
                            },
                            position: 59..61,
                        },
                        Expr {
                            kind: BinaryExpr {
                                kind: LessThanOrEquals,
                                operand_1: Expr {
                                    kind: Int {
                                        value: 1,
                                    },
                                    position: 74..75,
                                },
                                operand_2: Expr {
                                    kind: BinaryExpr {
                                        kind: Plus,
                                        operand_1: Expr {
                                            kind: Int {
                                                value: 2,
                                            },
                                            position: 80..81,
                                        },
                                        operand_2: Expr {
                                            kind: Int {
                                                value: 3,
                                            },
                                            position: 84..85,
                                        },
                                    },
                                    position: 82..83,
                                },
                            },
                            position: 76..78,
                        },
                        Expr {
                            kind: BinaryExpr {
                                kind: EqualsEquals,
                                operand_1: Expr {
                                    kind: BinaryExpr {
                                        kind: LessThan,
                                        operand_1: Expr {
                                            kind: Int {
                                                value: 1,
                                            },
                                            position: 97..98,
                                        },
                                        operand_2: Expr {
                                            kind: Int {
                                                value: 2,
                                            },
                                            position: 101..102,
                                        },
                                    },
                                    position: 99..100,
                                },
                                operand_2: Expr {
                                    kind: Bool {
                                        value: true,
                                    },
                                    position: 106..110,
                                },
                            },
                            position: 103..105,
                        },
                        Expr {
                            kind: BinaryExpr {
                                kind: EqualsEquals,
                                operand_1: Expr {
                                    kind: Bool {
                                        value: false,
                                    },
                                    position: 121..126,
                                },
                                operand_2: Expr {
                                    kind: BinaryExpr {
                                        kind: LessThan,
                                        operand_1: Expr {
                                            kind: Int {
                                                value: 1,
                                            },
                                            position: 131..132,
                                        },
                                        operand_2: Expr {
                                            kind: Int {
                                                value: 2,
                                            },
                                            position: 135..136,
                                        },
                                    },
                                    position: 133..134,
                                },
                            },
                            position: 127..129,
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
                        Expr {
                            kind: Let {
                                id: "a",
                                type_reference: Bool,
                                init_expr: Expr {
                                    kind: BinaryExpr {
                                        kind: LessThan,
                                        operand_1: Expr {
                                            kind: BinaryExpr {
                                                kind: Plus,
                                                operand_1: Expr {
                                                    kind: Id {
                                                        value: "z",
                                                    },
                                                    position: 25..26,
                                                },
                                                operand_2: Expr {
                                                    kind: Id {
                                                        value: "y",
                                                    },
                                                    position: 29..30,
                                                },
                                            },
                                            position: 27..28,
                                        },
                                        operand_2: Expr {
                                            kind: Id {
                                                value: "z",
                                            },
                                            position: 33..34,
                                        },
                                    },
                                    position: 31..32,
                                },
                            },
                            position: 18..22,
                        },
                        Expr {
                            kind: Let {
                                id: "b",
                                type_reference: Bool,
                                init_expr: Expr {
                                    kind: BinaryExpr {
                                        kind: NotEquals,
                                        operand_1: Expr {
                                            kind: Int {
                                                value: 1,
                                            },
                                            position: 59..60,
                                        },
                                        operand_2: Expr {
                                            kind: BinaryExpr {
                                                kind: Divide,
                                                operand_1: Expr {
                                                    kind: Int {
                                                        value: 2,
                                                    },
                                                    position: 64..65,
                                                },
                                                operand_2: Expr {
                                                    kind: Int {
                                                        value: 3,
                                                    },
                                                    position: 68..69,
                                                },
                                            },
                                            position: 66..67,
                                        },
                                    },
                                    position: 61..63,
                                },
                            },
                            position: 52..56,
                        },
                        Expr {
                            kind: Let {
                                id: "c",
                                type_reference: Unit,
                                init_expr: Expr {
                                    kind: BinaryExpr {
                                        kind: Plus,
                                        operand_1: Expr {
                                            kind: Int {
                                                value: 1,
                                            },
                                            position: 92..93,
                                        },
                                        operand_2: Expr {
                                            kind: BinaryExpr {
                                                kind: MoreThanOrEquals,
                                                operand_1: Expr {
                                                    kind: Int {
                                                        value: 2,
                                                    },
                                                    position: 97..98,
                                                },
                                                operand_2: Expr {
                                                    kind: Int {
                                                        value: 3,
                                                    },
                                                    position: 102..103,
                                                },
                                            },
                                            position: 99..101,
                                        },
                                    },
                                    position: 94..95,
                                },
                            },
                            position: 88..89,
                        },
                    ],
                },
            }"#]],
    );
}
