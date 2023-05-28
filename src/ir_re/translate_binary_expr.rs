// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Translates binary expressions. See `translator.rs` for context.

use ir_re::ir::{self, BinaryExprKind, Type};
use ir_re::translate_expr::translate_expr;
use ir_re::translator::{coerce, lift};
use ir_re::type_checker::TypeChecker;
use parser::ast;
use std::rc::Rc;
use utils::context::Position;
use utils::error_messages::{compilation_error, internal_compiler_error, ErrorPosition};

/// Translates a `ast::Expr::BinaryExpr` into a `ir::Expr::BinaryExpr`
///
/// * expr - the ast expression
/// * bindings - where to put additional bindings that are needed to translate the expression (temporary let-bindings)
/// * type_checker - type checker for this scope
pub fn translate_binary_expr<'a, 'b, 't>(
    expr: ast::Expr<'a>,
    bindings: &'b mut Vec<ir::Expr<'a>>,
    type_checker: &'t mut TypeChecker<'a>,
) -> (ir::Expr<'a>, Rc<Type>)
where
    'a: 't,
    't: 'b,
{
    if let ast::Expr::BinaryExpr { kind, operand_1, operand_2, operator_position } = expr {
        // Translate both operands
        let (operand_1, operand_1_type) = translate_expr(*operand_1, bindings, type_checker);
        let (operand_2, operand_2_type) = translate_expr(*operand_2, bindings, type_checker);

        // Type check and get the result type
        let (result_type, operand_1_coercion, operand_2_coercion) =
            type_checker.type_check_binary_expr(&kind, &operand_1_type, &operand_2_type, &operator_position);

        // Lift both operands.
        let operand_1 = lift(operand_1, &operand_1_type, bindings);
        let operand_2 = lift(operand_2, &operand_2_type, bindings);

        // Perform type coercion, if needed
        let (operand_1, operand_1_type) = coerce(operand_1, operand_1_type, operand_1_coercion, bindings);
        let (operand_2, operand_2_type) = coerce(operand_2, operand_2_type, operand_2_coercion, bindings);

        // For Solis, binary expressions must have operands be the same type (for now)
        if *operand_1_type != *operand_2_type {
            internal_compiler_error("operand type mismatch after coercion")
        }

        (
            ir::Expr::BinaryExpr {
                kind,
                operand_1: Box::new(operand_1),
                operand_2: Box::new(operand_2),
                operand_type: operand_1_type,
            },
            result_type,
        )
    } else {
        internal_compiler_error("non ast::Expr::BinaryExpr passed in")
    }
}

impl<'a> TypeChecker<'a> {
    // Type checks binary expressions.
    //
    // * binary_expr_kind - the type of binary expression
    // * operand_1_type - the type of the first operand
    // * operand_2_type - the type of the second operand
    // * operator_position - the position of the operator
    //
    // * return - (
    //     - the type of the result expression,
    //     - the type that the `operand_1` needs to be coerced into, if at all
    //     - the type that the `operand_2` needs to be coerced into, if at all
    // )
    fn type_check_binary_expr(
        &mut self,
        binary_expr_kind: &BinaryExprKind,
        operand_1_type: &Rc<Type>,
        operand_2_type: &Rc<Type>,
        operator_position: &Position,
    ) -> (Rc<Type>, Option<Rc<Type>>, Option<Rc<Type>>) {
        match binary_expr_kind {
            // For numerical operators, ensure both operands are integers/floats
            ir::BinaryExprKind::Plus
            | ir::BinaryExprKind::Minus
            | ir::BinaryExprKind::Times
            | ir::BinaryExprKind::Divide
            | ir::BinaryExprKind::Mod => {
                if !matches!(**operand_1_type, Type::Int | Type::Float)
                    || !matches!(**operand_2_type, Type::Int | Type::Float)
                {
                    compilation_error(
                      self.context,
                      ErrorPosition::Span(operator_position),
                      &format!("Bad operand types for `{binary_expr_kind:?}` operator: `{operand_1_type}` and `{operand_2_type}`")
                    )
                }

                let operand_1_is_float = matches!(**operand_1_type, Type::Float);
                let operand_2_is_float = matches!(**operand_2_type, Type::Float);

                if operand_1_is_float || operand_2_is_float {
                    (
                        Type::Float.into(),
                        if operand_1_is_float { None } else { Some(Type::Float.into()) },
                        if operand_2_is_float { None } else { Some(Type::Float.into()) },
                    )
                } else {
                    (Type::Int.into(), None, None)
                }
            }

            // For comparison operators, ensure both operands are integers/Floats
            ir::BinaryExprKind::LessThan
            | ir::BinaryExprKind::LessThanOrEquals
            | ir::BinaryExprKind::MoreThan
            | ir::BinaryExprKind::MoreThanOrEquals => {
                if !matches!(**operand_1_type, Type::Int | Type::Float)
                    || !matches!(**operand_2_type, Type::Int | Type::Float)
                {
                    compilation_error(
                      self.context,
                      ErrorPosition::Span(operator_position),
                      &format!("Bad operand types for `{binary_expr_kind:?}` operator: `{operand_1_type}` and `{operand_2_type}`")
                    )
                }

                let operand_1_is_float = matches!(**operand_1_type, Type::Float);
                let operand_2_is_float = matches!(**operand_2_type, Type::Float);

                if operand_1_is_float || operand_2_is_float {
                    (
                        Type::Bool.into(),
                        if operand_1_is_float { None } else { Some(Type::Float.into()) },
                        if operand_2_is_float { None } else { Some(Type::Float.into()) },
                    )
                } else {
                    (Type::Bool.into(), None, None)
                }
            }

            // For equality, ensure that both operands are the same type.
            ir::BinaryExprKind::EqualsEquals | ir::BinaryExprKind::NotEquals => {
                if **operand_1_type != **operand_2_type {
                    compilation_error(
                      self.context,
                      ErrorPosition::Span(operator_position),
                      &format!("Mismatched types. `{binary_expr_kind:?}` cannot be used with `{operand_1_type}` and `{operand_2_type}`")
                    )
                }
                (Type::Bool.into(), None, None)
            }
        }
    }
}
