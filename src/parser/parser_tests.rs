// Copyright © 2022 Brandon Li. All rights reserved.

//! Unit tests for the parser.

use parser::ast::*;
use parser::parser::*;
use parser::tokenizer::*;

#[test]
fn test_empty() {
    assert_eq!(
        parse_program(&tokenize("".to_string())),
        Program {
            body: Expr::Do { exprs: vec![] }
        }
    );
}

#[test]
fn test_basic() {
    assert_eq!(
        parse_program(&tokenize("let varName: int = 32".to_string())),
        Program {
            body: Expr::Do {
                exprs: vec![Expr::Let {
                    id: "varName".to_string(),
                    static_type: "int".to_string(),
                    expr: Box::new(Expr::Int { value: 32 }),
                }],
            },
        }
    );
}

#[test]
fn test_multiple_expressions() {
    assert_eq!(
        parse_program(&tokenize(
            "let a: int = 32\n let b: int = -123\n a\n b".to_string()
        )),
        Program {
            body: Expr::Do {
                exprs: vec![
                    Expr::Let {
                        id: "a".to_string(),
                        static_type: "int".to_string(),
                        expr: Box::new(Expr::Int { value: 32 }),
                    },
                    Expr::Let {
                        id: "b".to_string(),
                        static_type: "int".to_string(),
                        expr: Box::new(Expr::Int { value: -123 }),
                    },
                    Expr::Id {
                        value: "a".to_string()
                    },
                    Expr::Id {
                        value: "b".to_string()
                    },
                ],
            },
        }
    );
}

#[test]
fn test_math_precedence() {
    assert_eq!(
        parse_program(&tokenize(
            "let a: int = 1 + 2 * 3 \n let b: int = 1 / 2 - 3".to_string()
        )),
        Program {
            body: Expr::Do {
                exprs: vec![
                    Expr::Let {
                        id: "a".to_string(),
                        static_type: "int".to_string(),
                        expr: Box::new(Expr::Plus {
                            operand1: Box::new(Expr::Int { value: 1 }),
                            operand2: Box::new(Expr::Times {
                                operand1: Box::new(Expr::Int { value: 2 }),
                                operand2: Box::new(Expr::Int { value: 3 }),
                            }),
                        }),
                    },
                    Expr::Let {
                        id: "b".to_string(),
                        static_type: "int".to_string(),
                        expr: Box::new(Expr::Minus {
                            operand1: Box::new(Expr::Divide {
                                operand1: Box::new(Expr::Int { value: 1 }),
                                operand2: Box::new(Expr::Int { value: 2 }),
                            }),
                            operand2: Box::new(Expr::Int { value: 3 }),
                        }),
                    },
                ],
            },
        }
    );
}

#[test]
fn test_left_associative() {
    assert_eq!(
        parse_program(&tokenize(
            "let a: int = 32 - 2 * (3 + 4) / 5 - 3 * 2".to_string()
        )),
        Program {
            body: Expr::Do {
                exprs: vec![Expr::Let {
                    id: "a".to_string(),
                    static_type: "int".to_string(),
                    expr: Box::new(Expr::Minus {
                        operand1: Box::new(Expr::Minus {
                            operand1: Box::new(Expr::Int { value: 32 }),
                            operand2: Box::new(Expr::Divide {
                                operand1: Box::new(Expr::Times {
                                    operand1: Box::new(Expr::Int { value: 2 }),
                                    operand2: Box::new(Expr::Plus {
                                        operand1: Box::new(Expr::Int { value: 3 }),
                                        operand2: Box::new(Expr::Int { value: 4 }),
                                    }),
                                }),
                                operand2: Box::new(Expr::Int { value: 5 }),
                            }),
                        }),
                        operand2: Box::new(Expr::Times {
                            operand1: Box::new(Expr::Int { value: 3 }),
                            operand2: Box::new(Expr::Int { value: 2 }),
                        }),
                    }),
                }],
            },
        }
    );
}
