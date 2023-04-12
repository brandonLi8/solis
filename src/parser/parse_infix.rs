// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! This file parses infix expressions, which is an expression that is in infix notation, where operators are between
//! operands (like "1 + 2"). For Solis, this includes both the case where there are no operators at all (i.e. we are
//! responsible for parsing something like just "1"), which is defined to be a <factor>. In general, we are responsible
//! for parsing any number of factors (> 0) within infix notation. Note that infix expressions include both
//! arithmetic (+,-, ..) and comparison operators (<, >, ...). Please see `solis_grammar.txt`
//!
//! To parse infix expressions, we have to account for two tricky things.
//!   1. Operator precedence: order that operations should be performed. For example, "1 + 2 * 3" is different from
//!                           "1 * 2 + 3". Similarly "1 < 2 + 3" is "1 < (2 + 3)" and not "(1 < 2) + 3".
//!   2. Operand associativity: how operands are grouped together. "1 - 2 - 3" is evaluated as (1 - 2) - 3, and not
//!                             1 - (2 - 3). In general, we want left associativity.
//!
//! For operator precedence, we can implement this by separating each level of precedence into the grammar with several
//! layers, one for each level of precedence. It is recommended to look at the precedence section of `solis_grammar.txt`
//!
//! Operand associativity, on the other hand, is much trickier. This is because our grammar cannot be left-recursive
//! (this would cause an infinite recursion). For example, a - b - c MUST have the parse tree
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
//! Notice that the expected parse tree is naturally right associative (do b - c first, then a - (b - c)). To fix this,
//! we must "convert" the parse tree into the AST that looks like
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

use error_messages::internal_compiler_error;
use parser::ast::{BinaryExprKind, Expr, ExprKind, UnaryExprKind};
use parser::parse_expr::parse_expr;
use parser::parser::parse_terminal;
use parser::tokens_cursor::TokensCursor;
use tokenizer::tokenizer::Token;

/// Corresponds to `<infix-expr>` rule and parses into `ast::Expr`.
pub fn parse_infix_expr(tokens_cursor: &mut TokensCursor) -> Expr {
    parse_comparison_expr(tokens_cursor)
}

/// Corresponds to `<arithmetic-expr>` rule and parses into `ast::Expr`.
fn parse_arithmetic_expr(tokens_cursor: &mut TokensCursor) -> Expr {
    let arithmetic_1_operand = parse_arithmetic_1_operand(tokens_cursor);
    parse_arithmetic_1_rest(arithmetic_1_operand, tokens_cursor)
}

/// Corresponds to `<parse_arithmetic_1_rest>` rule and parses into `ast::Expr`
/// * `left_operand`: the left operand for the in result infix operation. See the comment at the top for full `tokens_cursor`.
fn parse_arithmetic_1_rest(mut left_operand: Expr, tokens_cursor: &mut TokensCursor) -> Expr {
    let (next_token, tokens_cursor) = tokens_cursor.peek();

    if let Some((kind @ (Token::Plus | Token::Minus), position )) = next_token {
        tokens_cursor.advance();

        let arithmetic_1_operand = parse_arithmetic_1_operand(tokens_cursor);
        left_operand = Expr {
            kind: ExprKind::BinaryExpr {
                kind: match kind {
                    Token::Plus => BinaryExprKind::Plus,
                    Token::Minus => BinaryExprKind::Minus,
                    _ => internal_compiler_error("Could not match +/- on inner match"),
                },
                operand_1: Box::new(left_operand),
                operand_2: Box::new(arithmetic_1_operand),
            },
            position: position.clone(),
        };

        parse_arithmetic_1_rest(left_operand, tokens_cursor)
    } else {
        left_operand
    }
}

/// Corresponds to <precedence-1-operand> rule and parses into `ast::Expr`.
fn parse_arithmetic_1_operand(tokens_cursor: &mut TokensCursor) -> Expr {
    let arithmetic_2_operand = parse_arithmetic_2_operand(tokens_cursor);
    parse_arithmetic_2_rest(arithmetic_2_operand, tokens_cursor)
}

/// Corresponds to `<parse_arithmetic_2_rest>` rule and parses into `ast::Expr`
/// * `left_operand`: the left operand for the in result infix operation. See the comment at the top for full `tokens_cursor`.
fn parse_arithmetic_2_rest(mut left_operand: Expr, tokens_cursor: &mut TokensCursor) -> Expr {
    let (next_token, tokens_cursor) = tokens_cursor.peek();

    if let Some((kind @ (Token::Times | Token::Divide | Token::Mod),
        position,
    )) = next_token
    {
        tokens_cursor.advance();

        let arithmetic_2_operand = parse_arithmetic_2_operand(tokens_cursor);
        left_operand = Expr {
            kind: ExprKind::BinaryExpr {
                kind: match kind {
                    Token::Times => BinaryExprKind::Times,
                    Token::Divide => BinaryExprKind::Divide,
                    Token::Mod => BinaryExprKind::Mod,
                    _ => internal_compiler_error("Could not match */%// on inner match"),
                },
                operand_1: Box::new(left_operand),
                operand_2: Box::new(arithmetic_2_operand),
            },
            position: position.clone(),
        };

        parse_arithmetic_2_rest(left_operand, tokens_cursor)
    } else {
        left_operand
    }
}

/// Corresponds to <precedence-2-operand> rule and parses into `ast::Expr`.
fn parse_arithmetic_2_operand(tokens_cursor: &mut TokensCursor) -> Expr {
    parse_factor(tokens_cursor)
}

/// Corresponds to `<comparison-expr>` rule and parses into `ast::Expr`.
fn parse_comparison_expr(tokens_cursor: &mut TokensCursor) -> Expr {
    let comparison_operand = parse_arithmetic_expr(tokens_cursor);
    parse_comparison_rest(comparison_operand, tokens_cursor)
}

/// Corresponds to `<parse_comparison_rest>` rule and parses into `ast::Expr`
/// * `left_operand`: the left operand for the in result infix operation. See the comment at the top for full `tokens_cursor`.
fn parse_comparison_rest(mut left_operand: Expr, tokens_cursor: &mut TokensCursor) -> Expr {
    let (next_token, tokens_cursor) = tokens_cursor.peek();

    if let Some((
            kind @ (Token::LessThan
            | Token::LessThanOrEquals
            | Token::MoreThan
            | Token::MoreThanOrEquals
            | Token::EqualsEquals
            | Token::NotEquals),
        position,
    )) = next_token
    {
        tokens_cursor.advance();

        let comparison_operand = parse_arithmetic_expr(tokens_cursor);
        left_operand = Expr {
            kind: ExprKind::BinaryExpr {
                kind: match kind {
                    Token::LessThan => BinaryExprKind::LessThan,
                    Token::LessThanOrEquals => BinaryExprKind::LessThanOrEquals,
                    Token::MoreThan => BinaryExprKind::MoreThan,
                    Token::MoreThanOrEquals => BinaryExprKind::MoreThanOrEquals,
                    Token::EqualsEquals => BinaryExprKind::EqualsEquals,
                    Token::NotEquals => BinaryExprKind::NotEquals,
                    _ => internal_compiler_error("Could not match comparison operator on inner match"),
                },
                operand_1: Box::new(left_operand),
                operand_2: Box::new(comparison_operand),
            },
            position: position.clone(),
        };

        parse_comparison_rest(left_operand, tokens_cursor)
    } else {
        left_operand
    }
}

/// Corresponds to <factor> rule and parses into `ast::Expr`.
fn parse_factor(tokens_cursor: &mut TokensCursor) -> Expr {
    let (next_token, tokens_cursor) = tokens_cursor.peek();

    if let Some((Token::OpenParen, _ )) = next_token {
        tokens_cursor.advance();

        let expr = parse_expr(tokens_cursor);
        tokens_cursor.consume_token(Token::CloseParen);
        expr
    } else {
        parse_prefix_expr(tokens_cursor)
    }
}

/// Corresponds to `<prefix-expr>` rule and parses into `ast::Expr`
fn parse_prefix_expr(tokens_cursor: &mut TokensCursor) -> Expr {
    let (next_token, tokens_cursor) = tokens_cursor.peek();

    if let Some(( kind @ (Token::Plus | Token::Minus | Token::Not),
        position,
    )) = next_token
    {
        tokens_cursor.advance();

        let operand = parse_factor(tokens_cursor);

        if let Token::Plus = kind {
            operand
        } else {
            Expr {
                kind: ExprKind::UnaryExpr {
                    kind: match kind {
                        Token::Minus => UnaryExprKind::Negative,
                        Token::Not => UnaryExprKind::Not,
                        _ => internal_compiler_error("Could not match prefix operator on inner match"),
                    },
                    operand: Box::new(operand),
                },
                position: position.clone(),
            }
        }
    } else {
        parse_terminal(tokens_cursor)
    }
}
