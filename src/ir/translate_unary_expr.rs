// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Translates unary expressions. See `translator.rs` for context.

use ir::ir::{self, Type, UnaryExprKind};
use ir::translate_expr::translate_expr;
use ir::translator::{coerce, lift};
use ir::type_checker::TypeChecker;
use parser::ast;
use std::rc::Rc;
use utils::context::Position;
use utils::error_messages::{compilation_error, internal_compiler_error, ErrorPosition};

/// Translates a `ast::Expr::UnaryExpr` into a `ir::Expr::UnaryExpr`
///
/// * expr - the ast expression
/// * bindings - where to put additional bindings that are needed to translate the expression (temporary let-bindings)
/// * type_checker - type checker for this scope
pub fn translate_unary_expr<'a, 'b, 't>(
    expr: ast::Expr<'a>,
    bindings: &'b mut Vec<ir::Expr<'a>>,
    type_checker: &'t mut TypeChecker<'a>,
) -> (ir::Expr<'a>, Rc<Type>)
where
    'a: 't,
    't: 'b,
{
    if let ast::Expr::UnaryExpr { kind, operand, operator_position } = expr {
        // Translate the operands
        let (operand, operand_type) = translate_expr(*operand, bindings, type_checker);

        // Type check and get the result type
        let (result_type, operand_coercion) =
            type_checker.type_check_unary_expr(&kind, &operand_type, &operator_position);

        // Lift the operand.
        let operand = lift(operand, &operand_type, bindings);

        // Perform type coercion, if needed
        let (operand, operand_type) = coerce(operand, operand_type, operand_coercion, bindings);

        (
            ir::Expr::UnaryExpr { kind, operand: Box::new(operand), operand_type },
            result_type,
        )
    } else {
        internal_compiler_error("non ast::Expr::UnaryExpr passed in")
    }
}

impl<'a> TypeChecker<'a> {
    // Type checks unary expressions.
    //
    // * unary_expr_kind - the type of unary expression
    // * operand_type - the type of the operand
    // * operator_position - the position of the operator
    //
    // * return - (
    //     - the type of the result expression,
    //     - the type that the `operand` needs to be coerced into, if at all
    // )
    fn type_check_unary_expr(
        &mut self,
        unary_expr_kind: &UnaryExprKind,
        operand_type: &Rc<Type>,
        operator_position: &Position,
    ) -> (Rc<Type>, Option<Rc<Type>>) {
        match unary_expr_kind {
            ir::UnaryExprKind::Not => {
                if **operand_type != Type::Bool {
                    compilation_error(
                        self.context,
                        ErrorPosition::Span(operator_position),
                        &format!(
                            "Mismatched types. `{unary_expr_kind:?}` operator expected `bool`, found `{operand_type}`"
                        ),
                    )
                }
                (Type::Bool.into(), None)
            }

            ir::UnaryExprKind::Negative => {
                if **operand_type != Type::Int && **operand_type != Type::Float {
                    compilation_error(
                        self.context,
                        ErrorPosition::Span(operator_position),
                        &format!(
                            "Mismatched types. `{unary_expr_kind:?}` operator expected `int` or `float`, found `{operand_type}`"
                        ),
                    )
                }
                (Rc::clone(operand_type), None)
            }
        }
    }
}
