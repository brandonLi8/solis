// Copyright © 2022 Brandon Li. All rights reserved.

//! This file parses infix expressions, which is an expression that is in infix notation, where operators are between
//! operands (like "1 + 2"). For Solis, this includes the case where there are no operators at all (i.e. we are
//! responsible for parsing something like just "1"), which is defined to be a <factor>. In general, we are responsible
//! for parsing any number of factors (> 0) within infix notation.
//!
//! This means we have to account for two tricky things.
//!   1. Operator precedence: order that operations should be performed. For example, "1 + 2 * 3" is different from
//!                           "1 * 2 + 3".
//!   2. Operand associativity: how operands are grouped together. "1 - 2 - 3" is evaluated as (1 - 2) - 3, and not
//!                             1 - (2 - 3). In general, we want left associativity.
//!
//! For operator precedence, we can implement this by separating each level of precedence into the grammar with several
//! layers, one for each level of precedence. It is recommended to look at the precedence section of `solis_grammar.txt`
//!
//! Operand associativity, on the other hand, is much trickier. This is because our grammar cannot be left-recursive
//! (this would cause an infinite loop). For example, a - b - c MUST have the parse tree
//!
//!            <infix operation>
//!            /               \
//!    <operand> id(a)  "-"     <rest>                        Key; <operand> = <precedence-1-operand>
//!                            /       \                           <rest>    = <precedence-1-operand>
//!              <operand> id(b)  "-"  <rest>
//!                                  /       \
//!                    <operand> id(c)  "-"   ε
//!
//!
//! Notice that this parse tree is naturally right associative (do b - c first, then a - (b - c)). To fix this, we
//! must "convert" the parse tree into the AST that looks like
//!            Minus
//!          /       \
//!      Minus      Id(c)
//!    /      \
//!  Id(a)   Id(b)
//!
//! For Solis, the way this is done is by passing a `left_operand` to the <rest> parser. This corresponds to the infix
//! expression of everything on the "left". For example, when parsing the final <rest> (id(c) and ε) the `left_operand`
//! corresponds to (a - b). Then, when creating the AST, we can but the `left_operand` on the left and the newest
//! operand (c) to the right. The final AST is passed onto the next <rest> call as the `left_operand`! This essentially
//! is converting the parse tree "on the fly" into a left associative structure!

use parser::ast::Expr;
use parser::parse_expression::parse_expr;
use parser::parser::parse_terminal;
use parser::parser::{consume_token, ParseContext};
use parser::tokenizer::{Token, TokenKind};
use utils;

/// Corresponds to <infix-operation> rule and parses into `ast::Expr`.
pub fn parse_infix_operation(context: &mut ParseContext) -> Expr {
    let precedence_1_operand = parse_precedence_1_operand(context);
    parse_precedence_1_rest(precedence_1_operand, context)
}

/// Corresponds to <`parse_precedence_1_rest`> rule and parses into `ast::Expr`
/// * `left_operand`: the left operand for the in result infix operation. See the comment at the top for full context.
pub fn parse_precedence_1_rest(mut left_operand: Expr, context: &mut ParseContext) -> Expr {
    match context.remaining_tokens {
        [token @ Token { kind: TokenKind::Plus | TokenKind::Minus, .. }, remaining_tokens @ ..] => {
            context.last_token = Some(token);
            context.remaining_tokens = remaining_tokens;

            let precedence_1_operand = parse_precedence_1_operand(context);
            left_operand = match token.kind {
                TokenKind::Plus => Expr::Plus {
                    operand_1: Box::new(left_operand),
                    operand_2: Box::new(precedence_1_operand),
                },
                TokenKind::Minus => Expr::Minus {
                    operand_1: Box::new(left_operand),
                    operand_2: Box::new(precedence_1_operand),
                },
                _ => utils::internal_compiler_error("Could not match +/- on inner match"),
            };
            parse_precedence_1_rest(left_operand, context)
        }
        _ => left_operand,
    }
}

/// Corresponds to <precedence-1-operand> rule and parses into `ast::Expr`.
pub fn parse_precedence_1_operand(context: &mut ParseContext) -> Expr {
    let precedence_2_operand = parse_precedence_2_operand(context);
    parse_precedence_2_rest(precedence_2_operand, context)
}

/// Corresponds to <`parse_precedence_2_rest`> rule and parses into `ast::Expr`
/// * `left_operand`: the left operand for the in result infix operation. See the comment at the top for full context.
pub fn parse_precedence_2_rest(mut left_operand: Expr, context: &mut ParseContext) -> Expr {
    match context.remaining_tokens {
        [token @ Token { kind: TokenKind::Times | TokenKind::Divide | TokenKind::Mod, .. }, remaining_tokens @ ..] => {
            context.last_token = Some(token);
            context.remaining_tokens = remaining_tokens;

            let precedence_2_operand = parse_precedence_2_operand(context);
            left_operand = match token.kind {
                TokenKind::Times => Expr::Times {
                    operand_1: Box::new(left_operand),
                    operand_2: Box::new(precedence_2_operand),
                },
                TokenKind::Divide => Expr::Divide {
                    operand_1: Box::new(left_operand),
                    operand_2: Box::new(precedence_2_operand),
                },
                TokenKind::Mod => Expr::Mod {
                    operand_1: Box::new(left_operand),
                    operand_2: Box::new(precedence_2_operand),
                },
                _ => utils::internal_compiler_error("Could not match +/- on inner match"),
            };
            parse_precedence_2_rest(left_operand, context)
        }
        _ => left_operand,
    }
}

/// Corresponds to <precedence-2-operand> rule and parses into `ast::Expr`.
fn parse_precedence_2_operand(context: &mut ParseContext) -> Expr {
    parse_factor(context)
}

/// Corresponds to <factor> rule and parses into `ast::Expr`.
fn parse_factor(context: &mut ParseContext) -> Expr {
    match context.remaining_tokens {
        [token @ Token { kind: TokenKind::OpenParen, .. }, remaining_tokens @ ..] => {
            context.last_token = Some(token);
            context.remaining_tokens = remaining_tokens;

            let expr = parse_expr(context);
            consume_token(TokenKind::CloseParen, context);
            expr
        }
        _ => parse_terminal(context),
    }
}
