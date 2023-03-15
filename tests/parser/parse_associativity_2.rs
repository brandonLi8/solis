// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Tests parser correctness of operand associativity.

use expect_test::expect;
use test_utils::parse_check;

#[test]
fn test_comparison_left_associative_1() {
    parse_check(
        r"
          1 < 2 < 3
          1 < (2 < 3)
          1 == 2 > 3
          1. <= (2.3 != 3)
        ",
        expect![[r#"
            Program {
                body: Block {
                    exprs: [
                        Expr {
                            kind: BinaryExpr {
                                kind: LessThan,
                                operand_1: Expr {
                                    kind: BinaryExpr {
                                        kind: LessThan,
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
                                kind: LessThan,
                                operand_1: Expr {
                                    kind: Int {
                                        value: 1,
                                    },
                                    position: 31..32,
                                },
                                operand_2: Expr {
                                    kind: BinaryExpr {
                                        kind: LessThan,
                                        operand_1: Expr {
                                            kind: Int {
                                                value: 2,
                                            },
                                            position: 36..37,
                                        },
                                        operand_2: Expr {
                                            kind: Int {
                                                value: 3,
                                            },
                                            position: 40..41,
                                        },
                                    },
                                    position: 38..39,
                                },
                            },
                            position: 33..34,
                        },
                        Expr {
                            kind: BinaryExpr {
                                kind: MoreThan,
                                operand_1: Expr {
                                    kind: BinaryExpr {
                                        kind: EqualsEquals,
                                        operand_1: Expr {
                                            kind: Int {
                                                value: 1,
                                            },
                                            position: 53..54,
                                        },
                                        operand_2: Expr {
                                            kind: Int {
                                                value: 2,
                                            },
                                            position: 58..59,
                                        },
                                    },
                                    position: 55..57,
                                },
                                operand_2: Expr {
                                    kind: Int {
                                        value: 3,
                                    },
                                    position: 62..63,
                                },
                            },
                            position: 60..61,
                        },
                        Expr {
                            kind: BinaryExpr {
                                kind: LessThanOrEquals,
                                operand_1: Expr {
                                    kind: Float {
                                        value: 1.0,
                                    },
                                    position: 74..76,
                                },
                                operand_2: Expr {
                                    kind: BinaryExpr {
                                        kind: NotEquals,
                                        operand_1: Expr {
                                            kind: Float {
                                                value: 2.3,
                                            },
                                            position: 81..84,
                                        },
                                        operand_2: Expr {
                                            kind: Int {
                                                value: 3,
                                            },
                                            position: 88..89,
                                        },
                                    },
                                    position: 85..87,
                                },
                            },
                            position: 77..79,
                        },
                    ],
                },
            }"#]],
    );
}

#[test]
fn test_comparison_left_associative_2() {
    parse_check(
        "let a: bool = 32 < 2 <= (3 > ((4))) / 5 >= 3 != 2 == 2",
        expect![[r#"
            Program {
                body: Block {
                    exprs: [
                        Expr {
                            kind: Let {
                                id: "a",
                                type_reference: Bool,
                                init_expr: Expr {
                                    kind: BinaryExpr {
                                        kind: EqualsEquals,
                                        operand_1: Expr {
                                            kind: BinaryExpr {
                                                kind: NotEquals,
                                                operand_1: Expr {
                                                    kind: BinaryExpr {
                                                        kind: MoreThanOrEquals,
                                                        operand_1: Expr {
                                                            kind: BinaryExpr {
                                                                kind: LessThanOrEquals,
                                                                operand_1: Expr {
                                                                    kind: BinaryExpr {
                                                                        kind: LessThan,
                                                                        operand_1: Expr {
                                                                            kind: Int {
                                                                                value: 32,
                                                                            },
                                                                            position: 14..16,
                                                                        },
                                                                        operand_2: Expr {
                                                                            kind: Int {
                                                                                value: 2,
                                                                            },
                                                                            position: 19..20,
                                                                        },
                                                                    },
                                                                    position: 17..18,
                                                                },
                                                                operand_2: Expr {
                                                                    kind: BinaryExpr {
                                                                        kind: Divide,
                                                                        operand_1: Expr {
                                                                            kind: BinaryExpr {
                                                                                kind: MoreThan,
                                                                                operand_1: Expr {
                                                                                    kind: Int {
                                                                                        value: 3,
                                                                                    },
                                                                                    position: 25..26,
                                                                                },
                                                                                operand_2: Expr {
                                                                                    kind: Int {
                                                                                        value: 4,
                                                                                    },
                                                                                    position: 31..32,
                                                                                },
                                                                            },
                                                                            position: 27..28,
                                                                        },
                                                                        operand_2: Expr {
                                                                            kind: Int {
                                                                                value: 5,
                                                                            },
                                                                            position: 38..39,
                                                                        },
                                                                    },
                                                                    position: 36..37,
                                                                },
                                                            },
                                                            position: 21..23,
                                                        },
                                                        operand_2: Expr {
                                                            kind: Int {
                                                                value: 3,
                                                            },
                                                            position: 43..44,
                                                        },
                                                    },
                                                    position: 40..42,
                                                },
                                                operand_2: Expr {
                                                    kind: Int {
                                                        value: 2,
                                                    },
                                                    position: 48..49,
                                                },
                                            },
                                            position: 45..47,
                                        },
                                        operand_2: Expr {
                                            kind: Int {
                                                value: 2,
                                            },
                                            position: 53..54,
                                        },
                                    },
                                    position: 50..52,
                                },
                            },
                            position: 7..11,
                        },
                    ],
                },
            }"#]],
    );
}
