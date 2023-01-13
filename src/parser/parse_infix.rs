// Copyright © 2022 Brandon Li. All rights reserved.

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
use parser::ast::Expr;
use parser::parse_expr::parse_expr;
use parser::parser::parse_terminal;
use parser::tokens_cursor::TokensCursor;
use tokenizer::tokenizer::{Token, TokenKind};

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

    if let Some(Token { kind: kind @ (TokenKind::Plus | TokenKind::Minus), .. }) = next_token {
        tokens_cursor.advance();

        let arithmetic_1_operand = parse_arithmetic_1_operand(tokens_cursor);
        left_operand = match kind {
            TokenKind::Plus => Expr::Plus {
                operand_1: Box::new(left_operand),
                operand_2: Box::new(arithmetic_1_operand),
            },
            TokenKind::Minus => Expr::Minus {
                operand_1: Box::new(left_operand),
                operand_2: Box::new(arithmetic_1_operand),
            },
            _ => internal_compiler_error("Could not match +/- on inner match"),
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

    if let Some(Token {
        kind: kind @ (TokenKind::Times | TokenKind::Divide | TokenKind::Mod), ..
    }) = next_token
    {
        tokens_cursor.advance();

        let arithmetic_2_operand = parse_arithmetic_2_operand(tokens_cursor);
        left_operand = match kind {
            TokenKind::Times => Expr::Times {
                operand_1: Box::new(left_operand),
                operand_2: Box::new(arithmetic_2_operand),
            },
            TokenKind::Divide => Expr::Divide {
                operand_1: Box::new(left_operand),
                operand_2: Box::new(arithmetic_2_operand),
            },
            TokenKind::Mod => Expr::Mod {
                operand_1: Box::new(left_operand),
                operand_2: Box::new(arithmetic_2_operand),
            },
            _ => internal_compiler_error("Could not match +/- on inner match"),
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

    if let Some(Token {
        kind:
            kind @ (TokenKind::LessThan
            | TokenKind::LessThanOrEquals
            | TokenKind::MoreThan
            | TokenKind::MoreThanOrEquals
            | TokenKind::EqualsEquals
            | TokenKind::NotEquals),
        ..
    }) = next_token
    {
        tokens_cursor.advance();

        let comparison_operand = parse_arithmetic_expr(tokens_cursor);
        left_operand = match kind {
            TokenKind::Times => Expr::Times {
                operand_1: Box::new(left_operand),
                operand_2: Box::new(comparison_operand),
            },
            TokenKind::LessThan => Expr::LessThan {
                operand_1: Box::new(left_operand),
                operand_2: Box::new(comparison_operand),
            },
            TokenKind::LessThanOrEquals => Expr::LessThanOrEquals {
                operand_1: Box::new(left_operand),
                operand_2: Box::new(comparison_operand),
            },
            TokenKind::MoreThan => Expr::MoreThan {
                operand_1: Box::new(left_operand),
                operand_2: Box::new(comparison_operand),
            },
            TokenKind::MoreThanOrEquals => Expr::MoreThanOrEquals {
                operand_1: Box::new(left_operand),
                operand_2: Box::new(comparison_operand),
            },
            TokenKind::EqualsEquals => Expr::EqualsEquals {
                operand_1: Box::new(left_operand),
                operand_2: Box::new(comparison_operand),
            },
            TokenKind::NotEquals => Expr::NotEquals {
                operand_1: Box::new(left_operand),
                operand_2: Box::new(comparison_operand),
            },
            _ => internal_compiler_error("Could not match +/- on inner match"),
        };

        parse_comparison_rest(left_operand, tokens_cursor)
    } else {
        left_operand
    }
}

/// Corresponds to <factor> rule and parses into `ast::Expr`.
fn parse_factor(tokens_cursor: &mut TokensCursor) -> Expr {
    let (next_token, tokens_cursor) = tokens_cursor.peek();

    if let Some(Token { kind: TokenKind::OpenParen, .. }) = next_token {
        tokens_cursor.advance();

        let expr = parse_expr(tokens_cursor);
        tokens_cursor.consume_token(TokenKind::CloseParen);
        expr
    } else {
        parse_terminal(tokens_cursor)
    }
}
