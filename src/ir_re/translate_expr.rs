// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Defines the functions for translating various types of expressions. See `translator.rs` for context.

use ir_re::ir::{self, Type};
use ir_re::translator::lift;
use ir_re::type_checker::TypeChecker;
use parser::ast;
use std::rc::Rc;
use utils::context::Position;
use utils::error_messages::internal_compiler_error;
use utils::error_messages::{compilation_error, ErrorPosition};

/// Translates a `ast::Expr` into a `ir::Expr`
///
/// * expr - the ast expression
/// * bindings - where to put additional bindings that are needed to translate the expression (temporary let-bindings)
pub fn translate_expr<'a, 'b, 't>(
    expr: ast::Expr<'a>,
    bindings: &'b mut Vec<ir::Expr<'a>>,
    type_checker: &'t mut TypeChecker<'a>,
) -> (ir::Expr<'a>, Rc<Type>)
where
    'a: 't,
    't: 'b,
{
    match expr {
        ast::Expr::Int { value } => (
            ir::Expr::Direct { expr: ir::DirectExpr::Int { value } },
            Rc::new(Type::Int),
        ),

        ast::Expr::Id { value, position } => {
            let id_type = type_checker.get_variable_type(value, &position);

            (
                ir::Expr::Direct { expr: ir::DirectExpr::Id { value, id_type: Rc::clone(id_type) } },
                Rc::clone(id_type),
            )
        }

        ast::Expr::Let { .. } => translate_let(expr, bindings, type_checker),

        ast::Expr::BinaryExpr { kind, operand_1, operand_2, operator_position: _ } => {
            let (operand_1, operand_1_type) = translate_expr(*operand_1, bindings, type_checker);
            let (operand_2, operand_2_type) = translate_expr(*operand_2, bindings, type_checker);

            (
                ir::Expr::BinaryExpr {
                    kind,
                    operand_1: Box::new(lift(operand_1, &operand_1_type, bindings)),
                    operand_2: Box::new(lift(operand_2, &operand_2_type, bindings)),
                    operand_type: operand_1_type,
                },
                Rc::new(Type::Unit),
            )
        }

        _ => todo!(),
    }
}

/// Translates a `ast::Expr::Let` into a `ir::Expr::Let`
///
/// * expr - the ast expression
/// * bindings - where to put additional bindings that are needed to translate the expression (temporary let-bindings)
pub fn translate_let<'a, 'b, 't>(
    expr: ast::Expr<'a>,
    bindings: &'b mut Vec<ir::Expr<'a>>,
    type_checker: &'t mut TypeChecker<'a>,
) -> (ir::Expr<'a>, Rc<Type>)
where
    'a: 't,
    't: 'b,
{
    if let ast::Expr::Let { id, id_position, type_reference, init_expr } = expr {
        // Reserve the variable to prevent use of the identifier in the init_expr.
        type_checker.reserve_variable(id, type_reference, &id_position);

        // Now translate the init_expr
        let (init_expr, init_type) = translate_expr(*init_expr, bindings, type_checker);

        type_checker.type_check_let(id, &*init_type, &id_position);

        // Flatten out let bindings inside sub expressions as well.
        bindings.push(ir::Expr::Let { id, init_expr: Box::new(init_expr) });
        (
            ir::Expr::Direct { expr: ir::DirectExpr::Id { value: id, id_type: Rc::clone(&init_type) } },
            init_type,
        )
    } else {
        internal_compiler_error("non ast::Expr::Let passed in")
    }
}

impl<'a> TypeChecker<'a> {
    // Type checks a let expression.
    //
    // * id - the name of the identifier
    // * init_expr_type - the resulting type of the `init_expr`
    fn type_check_let(&mut self, id: &'a str, init_expr_type: &Type, position: &Position) {
        // Get the annotated type_reference.
        let type_reference = self.get_reserved_variable_type(id);

        if type_reference != init_expr_type {
            compilation_error(
                self.context,
                ErrorPosition::Span(position),
                &format!("Mismatched types, expected `{init_expr_type}`, but found `{type_reference}`"),
            )
        }

        self.declare_reserved_variable(id, position);
    }
}
