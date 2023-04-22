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
//!    <operand> id(a)  "-"     <rest>                        Key: <operand> = <precedence-1-operand>
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
use parser::ast::{BinaryExprKind, Expr, UnaryExprKind};
use parser::parse_expr::parse_expr;
use parser::parser::parse_terminal;
use tokenizer::token_iterator::TokenIterator;
use tokenizer::tokenizer::Token;

/// Corresponds to `<infix-expr>` rule and parses into a `ast::Expr`.
pub fn parse_infix_expr(tokens: TokenIterator) -> (Expr, TokenIterator) {
    parse_comparison_expr(tokens)
}

/// Corresponds to `<arithmetic-expr>` rule and parses into a `ast::Expr`.
fn parse_arithmetic_expr(tokens: TokenIterator) -> (Expr, TokenIterator) {
    let (arithmetic_1_operand, tokens) = parse_arithmetic_1_operand(tokens);
    parse_arithmetic_1_rest(arithmetic_1_operand, tokens)
}

/// Corresponds to `<parse_arithmetic_1_rest>` rule and parses into a `ast::Expr`
/// * `left_operand`: the left operand for the in result infix operation. See the comment at the top for full `tokens`.
fn parse_arithmetic_1_rest<'a>(
    mut left_operand: Expr<'a>,
    mut tokens: TokenIterator<'a>,
) -> (Expr<'a>, TokenIterator<'a>) {
    if let Some((kind @ (Token::Plus | Token::Minus), _)) = tokens.peek() {
        let kind = match kind {
            Token::Plus => BinaryExprKind::Plus,
            Token::Minus => BinaryExprKind::Minus,
            _ => internal_compiler_error("Could not match +/- on inner match"),
        };

        tokens.advance();

        let (arithmetic_1_operand, tokens) = parse_arithmetic_1_operand(tokens);
        left_operand = Expr::BinaryExpr {
            kind,
            operand_1: Box::new(left_operand),
            operand_2: Box::new(arithmetic_1_operand),
        };

        parse_arithmetic_1_rest(left_operand, tokens)
    } else {
        (left_operand, tokens)
    }
}

/// Corresponds to <precedence-1-operand> rule and parses into a `ast::Expr`.
fn parse_arithmetic_1_operand(tokens: TokenIterator) -> (Expr, TokenIterator) {
    let (arithmetic_2_operand, tokens) = parse_arithmetic_2_operand(tokens);
    parse_arithmetic_2_rest(arithmetic_2_operand, tokens)
}

/// Corresponds to `<parse_arithmetic_2_rest>` rule and parses into a `ast::Expr`
/// * `left_operand`: the left operand for the in result infix operation. See the comment at the top for full `tokens`.
fn parse_arithmetic_2_rest<'a>(
    mut left_operand: Expr<'a>,
    mut tokens: TokenIterator<'a>,
) -> (Expr<'a>, TokenIterator<'a>) {
    if let Some((kind @ (Token::Times | Token::Divide | Token::Mod), _)) = tokens.peek() {
        let kind = match kind {
            Token::Times => BinaryExprKind::Times,
            Token::Divide => BinaryExprKind::Divide,
            Token::Mod => BinaryExprKind::Mod,
            _ => internal_compiler_error("Could not match */%// on inner match"),
        };

        tokens.advance();

        let (arithmetic_2_operand, tokens) = parse_arithmetic_2_operand(tokens);
        left_operand = Expr::BinaryExpr {
            kind,
            operand_1: Box::new(left_operand),
            operand_2: Box::new(arithmetic_2_operand),
        };

        parse_arithmetic_2_rest(left_operand, tokens)
    } else {
        (left_operand, tokens)
    }
}

/// Corresponds to <precedence-2-operand> rule and parses into a `ast::Expr`.
fn parse_arithmetic_2_operand(tokens: TokenIterator) -> (Expr, TokenIterator) {
    parse_factor(tokens)
}

/// Corresponds to `<comparison-expr>` rule and parses into a `ast::Expr`.
fn parse_comparison_expr(tokens: TokenIterator) -> (Expr, TokenIterator) {
    let (comparison_operand, tokens) = parse_arithmetic_expr(tokens);
    parse_comparison_rest(comparison_operand, tokens)
}

/// Corresponds to `<parse_comparison_rest>` rule and parses into a `ast::Expr`
/// * `left_operand`: the left operand for the in result infix operation. See the comment at the top for full `tokens`.
fn parse_comparison_rest<'a>(
    mut left_operand: Expr<'a>,
    mut tokens: TokenIterator<'a>,
) -> (Expr<'a>, TokenIterator<'a>) {
    if let Some((
        kind @ (Token::LessThan
        | Token::LessThanOrEquals
        | Token::MoreThan
        | Token::MoreThanOrEquals
        | Token::EqualsEquals
        | Token::NotEquals),
        _,
    )) = tokens.peek()
    {
        let kind = match kind {
            Token::LessThan => BinaryExprKind::LessThan,
            Token::LessThanOrEquals => BinaryExprKind::LessThanOrEquals,
            Token::MoreThan => BinaryExprKind::MoreThan,
            Token::MoreThanOrEquals => BinaryExprKind::MoreThanOrEquals,
            Token::EqualsEquals => BinaryExprKind::EqualsEquals,
            Token::NotEquals => BinaryExprKind::NotEquals,
            _ => internal_compiler_error("Could not match comparison operator on inner match"),
        };

        tokens.advance();

        let (comparison_operand, tokens) = parse_arithmetic_expr(tokens);
        left_operand = Expr::BinaryExpr {
            kind,
            operand_1: Box::new(left_operand),
            operand_2: Box::new(comparison_operand),
        };

        parse_comparison_rest(left_operand, tokens)
    } else {
        (left_operand, tokens)
    }
}

/// Corresponds to <factor> rule and parses into a `ast::Expr`.
fn parse_factor(mut tokens: TokenIterator) -> (Expr, TokenIterator) {
    if let Some((Token::OpenParen, _)) = tokens.peek() {
        tokens.advance();

        let (expr, mut tokens) = parse_expr(tokens);
        tokens.consume_token(Token::CloseParen);
        (expr, tokens)
    } else {
        parse_prefix_expr(tokens)
    }
}

/// Corresponds to `<prefix-expr>` rule and parses into a `ast::Expr`
fn parse_prefix_expr(mut tokens: TokenIterator) -> (Expr, TokenIterator) {
    if let Some((kind @ (Token::Minus | Token::Not), _)) = tokens.peek() {
        let kind = match kind {
            Token::Minus => UnaryExprKind::Negative,
            Token::Not => UnaryExprKind::Not,
            _ => internal_compiler_error("Could not match prefix operator on inner match"),
        };

        tokens.advance();

        let (operand, tokens) = parse_factor(tokens);

        (Expr::UnaryExpr { kind, operand: Box::new(operand) }, tokens)
    } else if let Some((Token::Plus, _)) = tokens.peek() {
        tokens.advance();
        parse_factor(tokens)
    } else {
        parse_terminal(tokens)
    }
}
