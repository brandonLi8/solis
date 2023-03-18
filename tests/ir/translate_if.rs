// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Tests for translating if expressions.

use expect_test::expect;
use test_utils::{translate_check, translate_error_check};

#[test]
fn test_translate_if_no_else() {
    translate_check(
        "
        let a: int = 0;
        let b: int = a;
        if a < b {
          1 + 2
          2 + 3
        }
        ",
        expect![[r#"
            Program {
                body: Block {
                    exprs: [
                        Let {
                            id: "a",
                            init_expr: Direct {
                                expr: Int {
                                    value: 0,
                                },
                            },
                        },
                        Let {
                            id: "b",
                            init_expr: Direct {
                                expr: Id {
                                    value: "a",
                                    id_type: Int,
                                },
                            },
                        },
                        Let {
                            id: "@temp0",
                            init_expr: BinaryExpr {
                                kind: LessThan,
                                operand_1: Id {
                                    value: "a",
                                    id_type: Int,
                                },
                                operand_2: Id {
                                    value: "b",
                                    id_type: Int,
                                },
                                operand_type: Int,
                            },
                        },
                        If {
                            condition: Id {
                                value: "@temp0",
                                id_type: Bool,
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
                                        operand_type: Int,
                                    },
                                    BinaryExpr {
                                        kind: Plus,
                                        operand_1: Int {
                                            value: 2,
                                        },
                                        operand_2: Int {
                                            value: 3,
                                        },
                                        operand_type: Int,
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
fn test_translate_if_empty() {
    translate_check(
        "
        let a: int = 0;
        let b: int = a;
          if a < b {
          }
        ",
        expect![[r#"
            Program {
                body: Block {
                    exprs: [
                        Let {
                            id: "a",
                            init_expr: Direct {
                                expr: Int {
                                    value: 0,
                                },
                            },
                        },
                        Let {
                            id: "b",
                            init_expr: Direct {
                                expr: Id {
                                    value: "a",
                                    id_type: Int,
                                },
                            },
                        },
                        Let {
                            id: "@temp0",
                            init_expr: BinaryExpr {
                                kind: LessThan,
                                operand_1: Id {
                                    value: "a",
                                    id_type: Int,
                                },
                                operand_2: Id {
                                    value: "b",
                                    id_type: Int,
                                },
                                operand_type: Int,
                            },
                        },
                        If {
                            condition: Id {
                                value: "@temp0",
                                id_type: Bool,
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
fn test_translate_if_else_basic() {
    translate_check(
        "
        let a: int = 0;
        let b: int = a;
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
                body: Block {
                    exprs: [
                        Let {
                            id: "a",
                            init_expr: Direct {
                                expr: Int {
                                    value: 0,
                                },
                            },
                        },
                        Let {
                            id: "b",
                            init_expr: Direct {
                                expr: Id {
                                    value: "a",
                                    id_type: Int,
                                },
                            },
                        },
                        Let {
                            id: "@temp0",
                            init_expr: BinaryExpr {
                                kind: LessThan,
                                operand_1: Id {
                                    value: "a",
                                    id_type: Int,
                                },
                                operand_2: Id {
                                    value: "b",
                                    id_type: Int,
                                },
                                operand_type: Int,
                            },
                        },
                        If {
                            condition: Id {
                                value: "@temp0",
                                id_type: Bool,
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
                                        operand_type: Int,
                                    },
                                    BinaryExpr {
                                        kind: Plus,
                                        operand_1: Int {
                                            value: 2,
                                        },
                                        operand_2: Int {
                                            value: 3,
                                        },
                                        operand_type: Int,
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
                                            operand_type: Int,
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
fn test_translate_if_else_chain() {
    translate_check(
        "
        let a: int = 0;
        let b: int = a;
        let d: bool = false;
        let c: bool = d;

        let e: int = if a < b {
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
                body: Block {
                    exprs: [
                        Let {
                            id: "a",
                            init_expr: Direct {
                                expr: Int {
                                    value: 0,
                                },
                            },
                        },
                        Let {
                            id: "b",
                            init_expr: Direct {
                                expr: Id {
                                    value: "a",
                                    id_type: Int,
                                },
                            },
                        },
                        Let {
                            id: "d",
                            init_expr: Direct {
                                expr: Bool {
                                    value: false,
                                },
                            },
                        },
                        Let {
                            id: "c",
                            init_expr: Direct {
                                expr: Id {
                                    value: "d",
                                    id_type: Bool,
                                },
                            },
                        },
                        Let {
                            id: "@temp0",
                            init_expr: BinaryExpr {
                                kind: LessThan,
                                operand_1: Id {
                                    value: "a",
                                    id_type: Int,
                                },
                                operand_2: Id {
                                    value: "b",
                                    id_type: Int,
                                },
                                operand_type: Int,
                            },
                        },
                        Let {
                            id: "e",
                            init_expr: If {
                                condition: Id {
                                    value: "@temp0",
                                    id_type: Bool,
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
                                            operand_type: Int,
                                        },
                                        BinaryExpr {
                                            kind: Plus,
                                            operand_1: Int {
                                                value: 2,
                                            },
                                            operand_2: Int {
                                                value: 3,
                                            },
                                            operand_type: Int,
                                        },
                                    ],
                                },
                                else_block: Some(
                                    Block {
                                        exprs: [
                                            Let {
                                                id: "@temp1",
                                                init_expr: Direct {
                                                    expr: Id {
                                                        value: "c",
                                                        id_type: Bool,
                                                    },
                                                },
                                            },
                                            If {
                                                condition: Id {
                                                    value: "@temp1",
                                                    id_type: Bool,
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
                                                            operand_type: Int,
                                                        },
                                                    ],
                                                },
                                                else_block: Some(
                                                    Block {
                                                        exprs: [
                                                            Let {
                                                                id: "@temp2",
                                                                init_expr: Direct {
                                                                    expr: Id {
                                                                        value: "d",
                                                                        id_type: Bool,
                                                                    },
                                                                },
                                                            },
                                                            If {
                                                                condition: Id {
                                                                    value: "@temp2",
                                                                    id_type: Bool,
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
                                                                            operand_type: Int,
                                                                        },
                                                                    ],
                                                                },
                                                                else_block: Some(
                                                                    Block {
                                                                        exprs: [
                                                                            Let {
                                                                                id: "@temp3",
                                                                                init_expr: Direct {
                                                                                    expr: Id {
                                                                                        value: "c",
                                                                                        id_type: Bool,
                                                                                    },
                                                                                },
                                                                            },
                                                                            If {
                                                                                condition: Id {
                                                                                    value: "@temp3",
                                                                                    id_type: Bool,
                                                                                },
                                                                                then_block: Block {
                                                                                    exprs: [
                                                                                        Direct {
                                                                                            expr: Id {
                                                                                                value: "a",
                                                                                                id_type: Int,
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
                                                                                                    value: 2,
                                                                                                },
                                                                                                operand_2: Int {
                                                                                                    value: 1,
                                                                                                },
                                                                                                operand_type: Int,
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
                        Direct {
                            expr: Id {
                                value: "e",
                                id_type: Int,
                            },
                        },
                    ],
                },
            }"#]],
    );
}

#[test]
fn test_translate_nested() {
    translate_check(
        "
        let a: int = 0
        let b: int = 0
        let c: int = 0

        let d: int = if (if true { c <= 0 } else { 1 + 2 + 3 < 6 }) {
          a + b
        }
        else if (if false { false } else { true }) {
          2
        }
        else {
          let e: int = b + c + 2
          e
        }
        ",
        expect![[r#"
            Program {
                body: Block {
                    exprs: [
                        Let {
                            id: "a",
                            init_expr: Direct {
                                expr: Int {
                                    value: 0,
                                },
                            },
                        },
                        Let {
                            id: "b",
                            init_expr: Direct {
                                expr: Int {
                                    value: 0,
                                },
                            },
                        },
                        Let {
                            id: "c",
                            init_expr: Direct {
                                expr: Int {
                                    value: 0,
                                },
                            },
                        },
                        Let {
                            id: "@temp0",
                            init_expr: Direct {
                                expr: Bool {
                                    value: true,
                                },
                            },
                        },
                        Let {
                            id: "@temp3",
                            init_expr: If {
                                condition: Id {
                                    value: "@temp0",
                                    id_type: Bool,
                                },
                                then_block: Block {
                                    exprs: [
                                        BinaryExpr {
                                            kind: LessThanOrEquals,
                                            operand_1: Id {
                                                value: "c",
                                                id_type: Int,
                                            },
                                            operand_2: Int {
                                                value: 0,
                                            },
                                            operand_type: Int,
                                        },
                                    ],
                                },
                                else_block: Some(
                                    Block {
                                        exprs: [
                                            Let {
                                                id: "@temp1",
                                                init_expr: BinaryExpr {
                                                    kind: Plus,
                                                    operand_1: Int {
                                                        value: 1,
                                                    },
                                                    operand_2: Int {
                                                        value: 2,
                                                    },
                                                    operand_type: Int,
                                                },
                                            },
                                            Let {
                                                id: "@temp2",
                                                init_expr: BinaryExpr {
                                                    kind: Plus,
                                                    operand_1: Id {
                                                        value: "@temp1",
                                                        id_type: Int,
                                                    },
                                                    operand_2: Int {
                                                        value: 3,
                                                    },
                                                    operand_type: Int,
                                                },
                                            },
                                            BinaryExpr {
                                                kind: LessThan,
                                                operand_1: Id {
                                                    value: "@temp2",
                                                    id_type: Int,
                                                },
                                                operand_2: Int {
                                                    value: 6,
                                                },
                                                operand_type: Int,
                                            },
                                        ],
                                    },
                                ),
                            },
                        },
                        Let {
                            id: "d",
                            init_expr: If {
                                condition: Id {
                                    value: "@temp3",
                                    id_type: Bool,
                                },
                                then_block: Block {
                                    exprs: [
                                        BinaryExpr {
                                            kind: Plus,
                                            operand_1: Id {
                                                value: "a",
                                                id_type: Int,
                                            },
                                            operand_2: Id {
                                                value: "b",
                                                id_type: Int,
                                            },
                                            operand_type: Int,
                                        },
                                    ],
                                },
                                else_block: Some(
                                    Block {
                                        exprs: [
                                            Let {
                                                id: "@temp4",
                                                init_expr: Direct {
                                                    expr: Bool {
                                                        value: false,
                                                    },
                                                },
                                            },
                                            Let {
                                                id: "@temp5",
                                                init_expr: If {
                                                    condition: Id {
                                                        value: "@temp4",
                                                        id_type: Bool,
                                                    },
                                                    then_block: Block {
                                                        exprs: [
                                                            Direct {
                                                                expr: Bool {
                                                                    value: false,
                                                                },
                                                            },
                                                        ],
                                                    },
                                                    else_block: Some(
                                                        Block {
                                                            exprs: [
                                                                Direct {
                                                                    expr: Bool {
                                                                        value: true,
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                    ),
                                                },
                                            },
                                            If {
                                                condition: Id {
                                                    value: "@temp5",
                                                    id_type: Bool,
                                                },
                                                then_block: Block {
                                                    exprs: [
                                                        Direct {
                                                            expr: Int {
                                                                value: 2,
                                                            },
                                                        },
                                                    ],
                                                },
                                                else_block: Some(
                                                    Block {
                                                        exprs: [
                                                            Let {
                                                                id: "@temp6",
                                                                init_expr: BinaryExpr {
                                                                    kind: Plus,
                                                                    operand_1: Id {
                                                                        value: "b",
                                                                        id_type: Int,
                                                                    },
                                                                    operand_2: Id {
                                                                        value: "c",
                                                                        id_type: Int,
                                                                    },
                                                                    operand_type: Int,
                                                                },
                                                            },
                                                            Let {
                                                                id: "e",
                                                                init_expr: BinaryExpr {
                                                                    kind: Plus,
                                                                    operand_1: Id {
                                                                        value: "@temp6",
                                                                        id_type: Int,
                                                                    },
                                                                    operand_2: Int {
                                                                        value: 2,
                                                                    },
                                                                    operand_type: Int,
                                                                },
                                                            },
                                                            Direct {
                                                                expr: Id {
                                                                    value: "e",
                                                                    id_type: Int,
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
                        },
                        Direct {
                            expr: Id {
                                value: "d",
                                id_type: Int,
                            },
                        },
                    ],
                },
            }"#]],
    );
}

#[test]
fn test_translate_branch_type_consistency() {
    translate_error_check(
        "
        let a: int = 0
        let b: int = 0
        let c: int = 0

        if (if true { c <= 0 } else { 1 + 2 + 3 < 6 }) {
          a + b
        }
        else if (if false { false } else { true }) {
          2
        }
        else if false {
          false
        }
        else {
          let d: int = b + c + 2
          d
        }
        ",
        expect![[r#"
            Error: Mismatched types on `if` branches, `bool` and `int`
             --> :12:13
               |
            12 |         else if false {
               |              ^^
        "#]],
    );
}

#[test]
fn test_translate_use_declared_inside_branch() {
    translate_error_check(
        "
        let a: int = 0
        let b: int = 0
        let c: int = 0

        if (if true { c <= 0 } else { 1 + 2 + 3 < 6 }) {
          a + b
        }
        else if (if false { false } else { true }) {
          2
        }
        else if false {
          2
        }
        else {
          let d: int = 3
          d
        }
        d
        ",
        expect![[r#"
            Error: Undeclared variable `d`
             --> :19:8
               |
            19 |         d
               |         ^
        "#]],
    );
}

#[test]
fn test_translate_use_before_declare() {
    translate_error_check(
        "
        let a: int = if true { a } else { 2 }
        ",
        expect![[r#"
            Error: Undeclared variable `a`
             --> :2:31
              |
            2 |         let a: int = if true { a } else { 2 }
              |                                ^
        "#]],
    );
}

#[test]
fn test_translate_use_before_declare_2() {
    translate_error_check(
        "
        let a: int = if a < 2 { 2 } else { 2 }
        ",
        expect![[r#"
            Error: Undeclared variable `a`
             --> :2:24
              |
            2 |         let a: int = if a < 2 { 2 } else { 2 }
              |                         ^
        "#]],
    );
}

#[test]
fn test_translate_redeclare_before_declare() {
    translate_error_check(
        "
        let a: int = if true { let a: int = 0 } else { 0 }
        ",
        expect![[r#"
            Error: Variable `a` is already declared in this scope
             --> :2:38
              |
            2 |         let a: int = if true { let a: int = 0 } else { 0 }
              |                                       ^^^
        "#]],
    );
}

#[test]
fn test_translate_no_else_type_mismatch() {
    translate_error_check(
        "
        let a: int = if false { 5 }
        ",
        expect![[r#"
            Error: Mismatched types, expected `<unit>`, but found `int`
             --> :2:15
              |
            2 |         let a: int = if false { 5 }
              |                ^^^
        "#]],
    );
}
