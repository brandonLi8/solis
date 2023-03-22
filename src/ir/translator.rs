// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! The translator lowers the AST into the intermediate representation of the program. This is the first stage of the
//! back end of the compiler. The IR structures are defined in `ir.rs`, where the context and rationale are documented.
//!
//! As documented in `ir.rs`, the IR is like the AST except every operand must be a `Direct`. To translate operands that
//! are complex expressions (like unary or binary expressions), we add temporary variables for the translations of
//! each operands, and substitute the identifier as a Direct into the original expression.

use error_messages::internal_compiler_error;
use ir::ir::{self, Type};
use ir::type_checker::TypeChecker;
use parser::ast;
use std::cell::RefCell;
use File;

/// Translates a `ast::Program` into a `ir::Program`
pub fn translate_program(file: &File, program: ast::Program) -> ir::Program {
    let mut type_checker = TypeChecker::new(file);
    let (body, _) = translate_block(&mut type_checker, program.body);
    ir::Program { body }
}

// Translates a `ast::Block` into a `ir::Block`
// * return - the block and the type that the block evaluates to
fn translate_block(type_checker: &mut TypeChecker, block: ast::Block) -> (ir::Block, Type) {
    let num_exprs = block.exprs.len();
    let mut exprs = vec![];

    let mut result_type = Type::Unit;

    for (i, expr) in block.exprs.into_iter().enumerate() {
        let (translated_expr, expr_type) = translate_expr(expr, type_checker, &mut exprs);

        // Ignore top-level directs, unless it is the last expression in the block
        if i == num_exprs - 1 || !matches!(translated_expr, ir::Expr::Direct { .. }) {
            exprs.push(translated_expr);
        }
        result_type = expr_type;
    }

    (ir::Block { exprs }, result_type)
}

// Translates a `ast::Expr` into a `ir::Expr`
// * bindings - where to put additional bindings that are needed to translate the expression (temporary let-bindings)
fn translate_expr(expr: ast::Expr, type_checker: &mut TypeChecker, bindings: &mut Vec<ir::Expr>) -> (ir::Expr, Type) {
    match expr.kind {
        ast::ExprKind::Id { value } => {
            let id_type = type_checker.get_declared_variable_type(&value, &expr.position);
            (
                ir::Expr::Direct { expr: ir::DirectExpr::Id { value, id_type: id_type.clone() } },
                id_type,
            )
        }
        ast::ExprKind::Int { value } => (ir::Expr::Direct { expr: ir::DirectExpr::Int { value } }, Type::Int),
        ast::ExprKind::Bool { value } => (ir::Expr::Direct { expr: ir::DirectExpr::Bool { value } }, Type::Bool),
        ast::ExprKind::Float { value } => {
            let float_expr = ir::Expr::Direct { expr: ir::DirectExpr::Float { value } };
            (
                // There are no such things as float immediates for x86. Instead, we must make each float a variable
                // binding (in the compiler, there must be a `Location` for floats). For example `let a: float = 1.2 + 2.3`
                // should translate to `let temp1 = 1.2; let temp2 = 2.3; let a = temp1 + temp2`.
                ir::Expr::Direct { expr: to_binding(float_expr, Type::Float, bindings) },
                Type::Float,
            )
        }
        ast::ExprKind::Let { id, init_expr, type_reference } => {
            let type_reference = match type_reference {
                ast::Type::Int => ir::Type::Int,
                ast::Type::Bool => ir::Type::Bool,
                ast::Type::Float => ir::Type::Float,
                ast::Type::Unit => ir::Type::Unit,
            };

            type_checker.register_variable_being_declared(&id, type_reference.clone(), &expr.position);
            let (init_expr, init_type) = translate_expr(*init_expr, type_checker, bindings);
            type_checker.type_check_let(&id, init_type.clone(), type_reference, &expr.position);

            // Flatten out let bindings inside sub expressions as well.
            bindings.push(ir::Expr::Let { id: id.clone(), init_expr: Box::new(init_expr) });
            (
                ir::Expr::Direct { expr: ir::DirectExpr::Id { value: id, id_type: init_type } },
                Type::Unit,
            )
        }
        ast::ExprKind::If { condition, then_block, else_block } => {
            let (condition, condition_type) = translate_expr(*condition, type_checker, bindings);
            let condition = to_binding(condition, condition_type.clone(), bindings);

            let (then_block, then_block_type) = translate_block(&mut TypeChecker::inherited(type_checker), then_block);

            let (else_block, else_block_type) = else_block.map_or((None, None), |else_block| {
                let (block, block_type) = translate_block(&mut TypeChecker::inherited(type_checker), else_block);
                (Some(block), Some(block_type))
            });

            // Type check and get the result type
            let result_type =
                type_checker.type_check_if(condition_type, then_block_type, else_block_type, &expr.position);
            (
                ir::Expr::If { condition: Box::new(condition), then_block, else_block },
                result_type,
            )
        }
        ast::ExprKind::UnaryExpr { kind, operand } => {
            // Translate operand
            let (operand_ir, operand_type) = translate_expr(*operand, type_checker, bindings);

            let kind = match kind {
                ast::UnaryExprKind::Not => ir::UnaryExprKind::Not,
                ast::UnaryExprKind::Negative => ir::UnaryExprKind::Negative,
            };

            // Type check and get the result type
            let (result_type, operand_coercion) =
                type_checker.type_check_unary_expr(&kind, operand_type.clone(), &expr.position);

            // Convert the operand to a direct
            let operand_ir = to_direct(operand_ir, operand_type.clone(), bindings);

            // Perform type coercion, if needed
            let (operand_ir, operand_type) = coerce_type(operand_ir, operand_type, operand_coercion, bindings);

            (
                ir::Expr::UnaryExpr { kind, operand: Box::new(operand_ir), operand_type },
                result_type,
            )
        }
        ast::ExprKind::BinaryExpr { kind, operand_1, operand_2 } => {
            // Translate both operands
            let (operand_1, operand_1_type) = translate_expr(*operand_1, type_checker, bindings);
            let (operand_2, operand_2_type) = translate_expr(*operand_2, type_checker, bindings);

            // Convert ast::BinaryExprKind to ir::BinaryExprKind.
            let kind = match kind {
                ast::BinaryExprKind::Plus => ir::BinaryExprKind::Plus,
                ast::BinaryExprKind::Minus => ir::BinaryExprKind::Minus,
                ast::BinaryExprKind::Times => ir::BinaryExprKind::Times,
                ast::BinaryExprKind::Divide => ir::BinaryExprKind::Divide,
                ast::BinaryExprKind::Mod => ir::BinaryExprKind::Mod,
                ast::BinaryExprKind::LessThan => ir::BinaryExprKind::LessThan,
                ast::BinaryExprKind::LessThanOrEquals => ir::BinaryExprKind::LessThanOrEquals,
                ast::BinaryExprKind::MoreThan => ir::BinaryExprKind::MoreThan,
                ast::BinaryExprKind::MoreThanOrEquals => ir::BinaryExprKind::MoreThanOrEquals,
                ast::BinaryExprKind::EqualsEquals => ir::BinaryExprKind::EqualsEquals,
                ast::BinaryExprKind::NotEquals => ir::BinaryExprKind::NotEquals,
            };

            // Type check and get the result type
            let (result_type, operand_1_coercion, operand_2_coercion) = type_checker.type_check_binary_expr(
                &kind,
                operand_1_type.clone(),
                operand_2_type.clone(),
                &expr.position,
            );

            // Convert the operands to directs
            let operand_1 = to_direct(operand_1, operand_1_type.clone(), bindings);
            let operand_2 = to_direct(operand_2, operand_2_type.clone(), bindings);

            // Perform type coercion, if needed
            let (operand_1, operand_1_type) = coerce_type(operand_1, operand_1_type, operand_1_coercion, bindings);
            let (operand_2, operand_2_type) = coerce_type(operand_2, operand_2_type, operand_2_coercion, bindings);

            // For Solis, binary expressions must have operands be the same type (for now)
            if operand_1_type != operand_2_type {
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
        }
        ast::ExprKind::Call { .. } => todo!(),
    }
}

// Converts a direct to another type, if given `expr_coercion`, by adding an additional binding.
// * return - (the (new) direct, and the type of the expression)
fn coerce_type(
    expr: ir::DirectExpr,
    expr_type: Type,
    expr_coercion: Option<Type>,
    bindings: &mut Vec<ir::Expr>,
) -> (ir::DirectExpr, Type) {
    if let Some(expr_coercion) = expr_coercion {
        let direct_identifier = gen_temp_identifier();
        let init_expr = ir::Expr::TypeCoercion {
            expr: Box::new(expr),
            from_type: expr_type,
            to_type: expr_coercion.clone(),
        };
        bindings.push(ir::Expr::Let { id: direct_identifier.to_string(), init_expr: Box::new(init_expr) });

        (
            ir::DirectExpr::Id { value: direct_identifier, id_type: expr_coercion.clone() },
            expr_coercion,
        )
    } else {
        (expr, expr_type)
    }
}

// Translates a `ir::Expr` into `ir::DirectExpr`.
// * bindings - where to put additional bindings that are needed to translate the expression (temporary let-bindings)
fn to_direct(expr: ir::Expr, expr_type: Type, bindings: &mut Vec<ir::Expr>) -> ir::DirectExpr {
    if let ir::Expr::Direct { expr } = expr {
        expr
    } else {
        to_binding(expr, expr_type, bindings)
    }
}

// Translates a `ir::Expr` into `ir::DirectExpr::Id` by adding a temporary let-binding.
fn to_binding(expr: ir::Expr, expr_type: Type, bindings: &mut Vec<ir::Expr>) -> ir::DirectExpr {
    let direct_identifier = gen_temp_identifier();
    bindings.push(ir::Expr::Let { id: direct_identifier.to_string(), init_expr: Box::new(expr) });
    ir::DirectExpr::Id { value: direct_identifier, id_type: expr_type }
}

// Ensures that the variable name of temporary variables are unique and can't conflict any source code names.
fn gen_temp_identifier() -> String {
    thread_local! {
        pub static TAG: RefCell<u32> = RefCell::new(0);
    }

    let mut tag_value = 0;

    TAG.with(|tag| {
        tag_value = *tag.borrow();
        *tag.borrow_mut() += 1;
    });

    format!("@temp{tag_value}")
}
