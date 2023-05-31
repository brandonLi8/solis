// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Translates if expressions. See `translator.rs` for context.

use ir::ir::{self, Type};
use ir::translate_expr::translate_expr;
use ir::translator::{force_lift, translate_block};
use ir::type_checker::TypeChecker;
use parser::ast;
use std::rc::Rc;
use utils::context::Position;
use utils::error_messages::{compilation_error, internal_compiler_error, ErrorPosition};

/// Translates a `ast::Expr::If` into a `ir::Expr::If`
///
/// * expr - the ast expression
/// * bindings - where to put additional bindings that are needed to translate the expression (temporary let-bindings)
/// * type_checker - type checker for this scope
pub fn translate_if<'a, 'b, 't>(
    expr: ast::Expr<'a>,
    bindings: &'b mut Vec<ir::Expr<'a>>,
    type_checker: &'t mut TypeChecker<'a>,
) -> (ir::Expr<'a>, Rc<Type>)
where
    'a: 't,
    't: 'b,
{
    if let ast::Expr::If { condition, then_block, else_block, if_position } = expr {
        let (condition, condition_type) = translate_expr(*condition, bindings, type_checker);

        // force_lift the condition to "outside" the if statement. We force_lift because the `Cmp` instruction disallows
        // immediates. See `asm.rs`
        let condition = force_lift(condition, &condition_type, bindings);

        let (then_block, then_block_type) = translate_block(then_block, &mut TypeChecker::inherit_scope(type_checker));

        let (else_block, else_block_type) = match else_block {
            None => (None, None),
            Some(else_block) => {
                let (block, block_type) = translate_block(else_block, &mut TypeChecker::inherit_scope(type_checker));
                (Some(block), Some(block_type))
            }
        };

        // Type check and get the result type
        let result_type = type_checker.type_check_if(condition_type, then_block_type, else_block_type, &if_position);

        (
            ir::Expr::If { condition: Box::new(condition), then_block, else_block },
            result_type,
        )
    } else {
        internal_compiler_error("non ast::Expr::Let passed in")
    }
}

impl<'a> TypeChecker<'a> {
    // Type checks if expressions.
    //
    // * condition_type - the type of the condition expression
    // * then_block_type - the type of the consequent expression
    // * else_block_type - the type of the alternate expression, if it exists
    // * if_position - the position of the if token
    //
    // * return - the type of the result expression.
    fn type_check_if(
        &mut self,
        condition_type: Rc<Type>,
        then_block_type: Rc<Type>,
        else_block_type: Option<Rc<Type>>,
        if_position: &Position,
    ) -> Rc<Type> {
        if *condition_type != Type::Bool {
            compilation_error(
                self.context,
                ErrorPosition::Span(if_position),
                &format!("`if` condition expected type `bool`, instead found `{condition_type}`"),
            )
        }

        if let Some(else_block_type) = else_block_type {
            if else_block_type != then_block_type {
                compilation_error(
                    self.context,
                    ErrorPosition::Span(if_position),
                    &format!("Mismatched types on `if` branches, `{then_block_type}` and `{else_block_type}`"),
                )
            }
            then_block_type
        } else {
            // If expressions with no else block evaluate to the unit type
            Type::Unit.into()
        }
    }
}
