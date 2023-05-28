// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Translates let expressions. See `translator.rs` for context.

use ir_re::ir::{self, Type};
use ir_re::translate_expr::translate_expr;
use ir_re::type_checker::TypeChecker;
use parser::ast;
use std::rc::Rc;
use utils::context::Position;
use utils::error_messages::{compilation_error, internal_compiler_error, ErrorPosition};

/// Translates a `ast::Expr::Let` into a `ir::Expr::Let`
///
/// * expr - the ast expression
/// * bindings - where to put additional bindings that are needed to translate the expression (temporary let-bindings)
/// * type_checker - type checker for this scope
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
            Type::Unit.into(),
        )
    } else {
        internal_compiler_error("non ast::Expr::Let passed in")
    }
}

impl<'a> TypeChecker<'a> {
    // Type checks let expressions.
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
