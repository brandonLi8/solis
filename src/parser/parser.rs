// Copyright © 2022 Brandon Li. All rights reserved.

//! The parser for Solis programs. A parser turns tokens into an AST representation of the program. This is the second
//! stage of the front end of the compiler. The AST is a more "nested" representation compared to tokens, and will be
//! ideal for the code-gen phase. The AST structure is defined in ast.rs
//!
//! Internally, the Solis parser is a recursive decent parser. The formal grammar is described in solis_grammar.txt
//! It works by creating a bunch of mutually recursive functions -- one for each production rule. Each function does:
//!   1. making sure there exists a parse tree for the program (is syntactically valid) by "walking" to the next rule
//!   2. transforming the parse tree into the AST
//! The parser runs in O(n) time with respect to the size of the program, since the grammar is a LL(k) class grammar.

use parser::ast::*;
use parser::tokenizer::*;
use utils;

/// Parser
pub fn parse_program(tokens: &[Token]) -> Program {
    let (exprs, remaining_tokens) = parse_exprs(vec![], tokens);

    // In the case that there are some remaining tokens (potentially a expression that wasn't fully written),
    // we raise a code error.
    if remaining_tokens.len() != 0 {
        utils::raise_code_error("TODO".to_string(), 0..0, "TODO1")
    }
    return Program {
        body: Expr::Do { exprs: exprs },
    };
}

fn parse_exprs(mut previous_exprs: Vec<Expr>, tokens: &[Token]) -> (Vec<Expr>, &[Token]) {
    match tokens {
        [] => (previous_exprs, tokens),
        _ => {
            let (next_expr, remaining_tokens) = parse_expr(tokens);
            previous_exprs.push(next_expr);
            parse_exprs(previous_exprs, remaining_tokens)
        }
    }
}

fn parse_expr(tokens: &[Token]) -> (Expr, &[Token]) {
    match tokens {
        [Token {
            token_type: TokenType::Let,
            ..
        }, Token {
            token_type: TokenType::Id(id),
            ..
        }, Token {
            token_type: TokenType::Colon,
            ..
        }, Token {
            token_type: TokenType::Id(static_type),
            ..
        }, Token {
            token_type: TokenType::Equals,
            ..
        }, expression_tokens @ ..] => {
            let (expr, remaining_tokens) = parse_expr(expression_tokens);
            (
                Expr::Let {
                    id: id.to_string(),
                    static_type: static_type.to_string(),
                    expr: Box::new(expr),
                },
                remaining_tokens,
            )
        }
        _ => {
            let (addition_operand, rhs_addition_tokens) = parse_addition_operand(tokens);
            parse_rest_addition(addition_operand, rhs_addition_tokens)
        }
    }
}

fn parse_terminal(tokens: &[Token]) -> (Expr, &[Token]) {
    match tokens {
        [Token {
            token_type: TokenType::Id(id),
            ..
        }, remaining_tokens @ ..] => (
            Expr::Id {
                value: id.to_string(),
            },
            remaining_tokens,
        ),
        [Token {
            token_type: TokenType::Int(int),
            ..
        }, remaining_tokens @ ..] => (Expr::Int { value: *int }, remaining_tokens),
        _ => utils::raise_code_error("TODO".to_string(), 0..0, "TODO3"),
    }
}

fn parse_rest_addition(mut prev_combined_operands: Expr, tokens: &[Token]) -> (Expr, &[Token]) {
    match tokens {
        [Token {
            token_type: token_type @ TokenType::Plus | token_type @ TokenType::Minus,
            ..
        }, addition_operand_tokens @ ..] => {
            let (addition_operand, remaining_tokens) =
                parse_addition_operand(addition_operand_tokens);
            prev_combined_operands = match token_type {
                TokenType::Plus => Expr::Plus {
                    operand1: Box::new(prev_combined_operands),
                    operand2: Box::new(addition_operand),
                },
                TokenType::Minus => Expr::Minus {
                    operand1: Box::new(prev_combined_operands),
                    operand2: Box::new(addition_operand),
                },
                _ => todo!(), // TODO: internal compiler error
            };
            parse_rest_addition(prev_combined_operands, remaining_tokens)
        }
        _ => (prev_combined_operands, tokens),
    }
}

fn parse_addition_operand(tokens: &[Token]) -> (Expr, &[Token]) {
    let (multiplication_operand, rhs_multiplication_tokens) = parse_multiplication_operand(tokens);
    parse_rest_multiplication(multiplication_operand, rhs_multiplication_tokens)
}

fn parse_rest_multiplication(
    mut prev_combined_operands: Expr,
    tokens: &[Token],
) -> (Expr, &[Token]) {
    match tokens {
        [Token {
            token_type: token_type @ TokenType::Times | token_type @ TokenType::Divide,
            ..
        }, multiplication_operand_tokens @ ..] => {
            let (multiplication_operand, remaining_tokens) =
                parse_multiplication_operand(multiplication_operand_tokens);
            prev_combined_operands = match token_type {
                TokenType::Times => Expr::Times {
                    operand1: Box::new(prev_combined_operands),
                    operand2: Box::new(multiplication_operand),
                },
                TokenType::Divide => Expr::Divide {
                    operand1: Box::new(prev_combined_operands),
                    operand2: Box::new(multiplication_operand),
                },
                _ => todo!(), // TODO: internal compiler error
            };
            parse_rest_multiplication(prev_combined_operands, remaining_tokens)
        }
        _ => (prev_combined_operands, tokens),
    }
}

fn parse_multiplication_operand(tokens: &[Token]) -> (Expr, &[Token]) {
    parse_factor(tokens)
}

fn parse_factor(tokens: &[Token]) -> (Expr, &[Token]) {
    match tokens {
        [Token {
            token_type: TokenType::Lparen,
            ..
        }, parenthesized_expr_remaining_tokens @ ..] => {
            let (expr, remaining_tokens) = parse_expr(parenthesized_expr_remaining_tokens);
            (expr, consume_token(TokenType::Rparen, remaining_tokens))
        }
        _ => parse_terminal(tokens),
    }
}

/// Ensures that the head of tokens is expected_token_type. If it is, it returns the
/// rest of tokens. Otherwise, it raises a code error
fn consume_token(expected_token_type: TokenType, tokens: &[Token]) -> &[Token] {
    if tokens.len() == 0 {
        utils::raise_code_error("TODO".to_string(), 0..0, "TODO5")
    }
    if tokens[0].token_type != expected_token_type {
        utils::raise_code_error("TODO".to_string(), 0..0, "TODO6")
    }

    return &tokens[1..];
}
