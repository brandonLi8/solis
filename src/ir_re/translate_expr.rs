// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Defines the functions for translating various types of expressions. See `translator.rs` for context.

use ir_re::translate_if::translate_if;
use ir_re::translate_let::translate_let;
use ir_re::ir::{self, Type};
use ir_re::translator::{lift};
use ir_re::type_checker::TypeChecker;
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
        ast::Expr::If { .. } => translate_if(expr, bindings, type_checker),

        ast::Expr::BinaryExpr { kind, operand_1, operand_2, operator_position: _ } => {
            let (operand_1, operand_1_type) = translate_expr(*operand_1, bindings, type_checker);
            let (operand_2, operand_2_type) = translate_expr(*operand_2, bindings, type_checker);
            let operand_type = Rc::clone(&operand_1_type);

            (
                ir::Expr::BinaryExpr {
                    kind,
                    operand_1: Box::new(lift(operand_1, &operand_1_type, bindings)),
                    operand_2: Box::new(lift(operand_2, &operand_2_type, bindings)),
                    operand_type,
                },
                Rc::new(Type::Unit),
            )
        }

        _ => todo!(),
    }
}


