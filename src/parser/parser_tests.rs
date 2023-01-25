// Copyright © 2022 Brandon Li. All rights reserved.

//! Unit tests for the parser. Note that the semantics of some tests may not be correct, as we are only testing parsing.

use expect_test::{expect, Expect};
use parser::parser::parse;
use tokenizer::tokenizer::tokenize;
use File;

#[test]
fn test_empty() {
    parse_check(
        "",
        expect![[r#"
            Program {
                body: Block {
                    exprs: [],
                },
            }"#]],
    )
}

#[test]
fn test_basic() {
    parse_check(
        "let varName: int = 32",
        expect![[r#"
            Program {
                body: Block {
                    exprs: [
                        Let {
                            id: "varName",
                            type_reference: "int",
                            init_expr: Int {
                                value: 32,
                            },
                        },
                    ],
                },
            }"#]],
    )
}

#[test]
fn test_multiple_expressions() {
    parse_check(
        "let a: int = 32\n let b: int = -123\n a\n b 2 + 43 == 45",
        expect![[r#"
            Program {
                body: Block {
                    exprs: [
                        Let {
                            id: "a",
                            type_reference: "int",
                            init_expr: Int {
                                value: 32,
                            },
                        },
                        Let {
                            id: "b",
                            type_reference: "int",
                            init_expr: UnaryExpr {
                                kind: Negative,
                                operand: Int {
                                    value: 123,
                                },
                            },
                        },
                        Id {
                            value: "a",
                        },
                        Id {
                            value: "b",
                        },
                        BinaryExpr {
                            kind: EqualsEquals,
                            operand_1: BinaryExpr {
                                kind: Plus,
                                operand_1: Int {
                                    value: 2,
                                },
                                operand_2: Int {
                                    value: 43,
                                },
                            },
                            operand_2: Int {
                                value: 45,
                            },
                        },
                    ],
                },
            }"#]],
    );
}

#[test]
fn test_arithmetic_precedence() {
    parse_check(
        "let a: int = 1 + 2 * 3 \n let b: int = 1 / 2 - 3",
        expect![[r#"
            Program {
                body: Block {
                    exprs: [
                        Let {
                            id: "a",
                            type_reference: "int",
                            init_expr: BinaryExpr {
                                kind: Plus,
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
                                },
                            },
                        },
                        Let {
                            id: "b",
                            type_reference: "int",
                            init_expr: BinaryExpr {
                                kind: Minus,
                                operand_1: BinaryExpr {
                                    kind: Divide,
                                    operand_1: Int {
                                        value: 1,
                                    },
                                    operand_2: Int {
                                        value: 2,
                                    },
                                },
                                operand_2: Int {
                                    value: 3,
                                },
                            },
                        },
                    ],
                },
            }"#]],
    );
}

#[test]
fn test_comparison_precedence() {
    parse_check(
        "let a: bool = z + y < z\n let b: bool = 1 != 2 / 3 \n let c: invalid = 1 + (2 >= 3) # semantics wrong",
        expect![[r#"
            Program {
                body: Block {
                    exprs: [
                        Let {
                            id: "a",
                            type_reference: "bool",
                            init_expr: BinaryExpr {
                                kind: LessThan,
                                operand_1: BinaryExpr {
                                    kind: Plus,
                                    operand_1: Id {
                                        value: "z",
                                    },
                                    operand_2: Id {
                                        value: "y",
                                    },
                                },
                                operand_2: Id {
                                    value: "z",
                                },
                            },
                        },
                        Let {
                            id: "b",
                            type_reference: "bool",
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
                                },
                            },
                        },
                        Let {
                            id: "c",
                            type_reference: "invalid",
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
                                },
                            },
                        },
                    ],
                },
            }"#]],
    )
}

#[test]
fn test_comparison_left_associative() {
    parse_check(
        "let a: bool = 32 < 2 <= (3 > ((4))) / 5 >= 3 != 2 == 2",
        expect![[r#"
            Program {
                body: Block {
                    exprs: [
                        Let {
                            id: "a",
                            type_reference: "bool",
                            init_expr: BinaryExpr {
                                kind: EqualsEquals,
                                operand_1: BinaryExpr {
                                    kind: NotEquals,
                                    operand_1: BinaryExpr {
                                        kind: MoreThanOrEquals,
                                        operand_1: BinaryExpr {
                                            kind: LessThanOrEquals,
                                            operand_1: BinaryExpr {
                                                kind: LessThan,
                                                operand_1: Int {
                                                    value: 32,
                                                },
                                                operand_2: Int {
                                                    value: 2,
                                                },
                                            },
                                            operand_2: BinaryExpr {
                                                kind: Divide,
                                                operand_1: BinaryExpr {
                                                    kind: MoreThan,
                                                    operand_1: Int {
                                                        value: 3,
                                                    },
                                                    operand_2: Int {
                                                        value: 4,
                                                    },
                                                },
                                                operand_2: Int {
                                                    value: 5,
                                                },
                                            },
                                        },
                                        operand_2: Int {
                                            value: 3,
                                        },
                                    },
                                    operand_2: Int {
                                        value: 2,
                                    },
                                },
                                operand_2: Int {
                                    value: 2,
                                },
                            },
                        },
                    ],
                },
            }"#]],
    );
}

#[test]
fn test_arithmetic_left_associative() {
    parse_check(
        "let a: int = 32 - 2 * (3 + ((4))) / 5 - 3 * 2",
        expect![[r#"
            Program {
                body: Block {
                    exprs: [
                        Let {
                            id: "a",
                            type_reference: "int",
                            init_expr: BinaryExpr {
                                kind: Minus,
                                operand_1: BinaryExpr {
                                    kind: Minus,
                                    operand_1: Int {
                                        value: 32,
                                    },
                                    operand_2: BinaryExpr {
                                        kind: Divide,
                                        operand_1: BinaryExpr {
                                            kind: Times,
                                            operand_1: Int {
                                                value: 2,
                                            },
                                            operand_2: BinaryExpr {
                                                kind: Plus,
                                                operand_1: Int {
                                                    value: 3,
                                                },
                                                operand_2: Int {
                                                    value: 4,
                                                },
                                            },
                                        },
                                        operand_2: Int {
                                            value: 5,
                                        },
                                    },
                                },
                                operand_2: BinaryExpr {
                                    kind: Times,
                                    operand_1: Int {
                                        value: 3,
                                    },
                                    operand_2: Int {
                                        value: 2,
                                    },
                                },
                            },
                        },
                    ],
                },
            }"#]],
    )
}

#[test]
fn test_prefix() {
    parse_check(
        "let name: unknown = +2 - -3 - -+-+-4 + !4",
        expect![[r#"
            Program {
                body: Block {
                    exprs: [
                        Let {
                            id: "name",
                            type_reference: "unknown",
                            init_expr: BinaryExpr {
                                kind: Plus,
                                operand_1: BinaryExpr {
                                    kind: Minus,
                                    operand_1: BinaryExpr {
                                        kind: Minus,
                                        operand_1: Int {
                                            value: 2,
                                        },
                                        operand_2: UnaryExpr {
                                            kind: Negative,
                                            operand: Int {
                                                value: 3,
                                            },
                                        },
                                    },
                                    operand_2: UnaryExpr {
                                        kind: Negative,
                                        operand: UnaryExpr {
                                            kind: Negative,
                                            operand: UnaryExpr {
                                                kind: Negative,
                                                operand: Int {
                                                    value: 4,
                                                },
                                            },
                                        },
                                    },
                                },
                                operand_2: UnaryExpr {
                                    kind: Not,
                                    operand: Int {
                                        value: 4,
                                    },
                                },
                            },
                        },
                    ],
                },
            }"#]],
    );
}

#[test]
#[should_panic(expected = "Syntax Error: unexpected token at 13..14")]
fn test_parse_terminal_unexpected_token() {
    parse_check("let a: int = * * 2", expect![[]]);
}

#[test]
#[should_panic(expected = "Syntax Error: unexpected end of file at 15..16")]
fn test_parse_terminal_unexpected_end_of_file() {
    parse_check("let a: int = 2 +", expect![[]]);
}

#[test]
#[should_panic(expected = "Syntax Error: unexpected end of file at 13..14")]
fn test_parse_terminal_unexpected_end_of_file_2() {
    parse_check("let a: int = +", expect![[]]);
}

#[test]
#[should_panic(expected = "Syntax Error: unexpected end of file at 22..23")]
fn test_parse_factor_consume_token() {
    parse_check("let a: int = 2 + (2 + 1", expect![[]]);
}

#[test]
#[should_panic(expected = "Syntax Error: expected CloseParen at 22..23")]
fn test_parse_factor_consume_token_end_of_file() {
    parse_check("let a: int = 2 + (2 + 1 \nlet b: int = 2", expect![[]]);
}

/// A helper function to test parsing a program, where the filename does not matter and only the contents matter.
fn parse_check(program: &str, expect: Expect) {
    let file = File { name: String::new(), contents: program.to_string() };

    expect.assert_eq(&format!("{:#?}", parse(&file, tokenize(&file))))
}
