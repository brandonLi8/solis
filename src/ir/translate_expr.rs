// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Defines the functions for translating various types of expressions. See `translator.rs` for context.

use ir::ir::{self, Type};
use ir::translate_binary_expr::translate_binary_expr;
use ir::translate_function::translate_call;
use ir::translate_if::translate_if;
use ir::translate_let::translate_let;
use ir::translate_unary_expr::translate_unary_expr;
use ir::translator::force_lift;
use ir::type_checker::TypeChecker;
use parser::ast;
use std::rc::Rc;

/// Translates a `ast::Expr` into a `ir::Expr`
///
/// * expr - the ast expression
/// * bindings - where to put additional bindings that are needed to translate the expression (temporary let-bindings)
/// * type_checker - type checker for this scope
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
            Type::Int.into(),
        ),
        ast::Expr::Bool { value } => (
            ir::Expr::Direct { expr: ir::DirectExpr::Bool { value } },
            Type::Bool.into(),
        ),
        ast::Expr::Float { value } => {
            (
                // There are no such things as float immediates for x86. Instead, we must make each float a variable
                // binding (in the compiler, there must be a `Location` for floats). For example `let a: float = 1.2 + 2.3`
                // should translate to `let temp1 = 1.2; let temp2 = 2.3; let a = temp1 + temp2`.
                ir::Expr::Direct {
                    expr: force_lift(
                        ir::Expr::Direct { expr: ir::DirectExpr::Float { value } },
                        &Type::Float.into(),
                        bindings,
                    ),
                },
                Type::Float.into(),
            )
        }

        ast::Expr::Id { value, position } => {
            let id_type = type_checker.get_variable_type(value, &position);

            (
                ir::Expr::Direct { expr: ir::DirectExpr::Id { value, id_type: Rc::clone(id_type) } },
                Rc::clone(id_type),
            )
        }

        ast::Expr::Let { .. } => translate_let(expr, bindings, type_checker),
        ast::Expr::If { .. } => translate_if(expr, bindings, type_checker),
        ast::Expr::BinaryExpr { .. } => translate_binary_expr(expr, bindings, type_checker),
        ast::Expr::UnaryExpr { .. } => translate_unary_expr(expr, bindings, type_checker),
        ast::Expr::Call { .. } => translate_call(expr, bindings, type_checker),
    }
}
