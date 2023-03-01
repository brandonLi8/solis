// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! The translator lowers the AST into the intermediate representation of the program. This is the first stage of the
//! back end of the compiler. The IR structures are defined in `ir.rs`, where the context and rationale are documented.
//!
//! As documented in `ir.rs`, the IR is like the AST except every operand must be a `Direct`. To translate operands that
//! are complex expressions (like unary or binary expressions), we add temporary variables for the translations of
//! each operands, and substitute the identifier as a Direct into the original expression.

use error_messages::internal_compiler_error;
use ir::ir;
use ir::type_checker::{SolisType, TypeChecker};
use parser::ast;
use std::cell::RefCell;
use File;

/// Translates a `ast::Program` into a `ir::Program`
pub fn translate_program(file: &File, program: ast::Program) -> ir::Program {
    ir::Program { body: translate_block(file, program.body) }
}

// Translates a `ast::Block` into a `ir::Block`
fn translate_block(file: &File, block: ast::Block) -> ir::Block {
    let num_exprs = block.exprs.len();
    let mut exprs = vec![];

    let mut type_checker = TypeChecker::new(file);

    for (i, expr) in block.exprs.into_iter().enumerate() {
        let (translated_expr, _) = translate_expr(expr, &mut type_checker, &mut exprs);

        // Ignore top-level directs, unless it is the last expression in the block
        if i == num_exprs - 1 || !matches!(translated_expr, ir::Expr::Direct { .. }) {
            exprs.push(translated_expr);
        }
    }

    ir::Block { exprs, identifier_types: type_checker.identifier_types }
}

// Translates a `ast::Expr` into a `ir::Expr`
// * bindings - where to put additional bindings that are needed to translate the expression (temporary let-bindings)
fn translate_expr(
    expr: ast::Expr,
    type_checker: &mut TypeChecker,
    bindings: &mut Vec<ir::Expr>,
) -> (ir::Expr, SolisType) {
    match expr.kind {
        ast::ExprKind::Id { value } => (
            ir::Expr::Direct { expr: ir::DirectExpr::Id { value: value.to_string() } },
            type_checker.get_type(&value, &expr.position),
        ),
        ast::ExprKind::Int { value } => (ir::Expr::Direct { expr: ir::DirectExpr::Int { value } }, SolisType::Int),
        ast::ExprKind::Bool { value } => (
            ir::Expr::Direct { expr: ir::DirectExpr::Bool { value } },
            SolisType::Bool,
        ),
        ast::ExprKind::Float { value } => {
            let float_expr = ir::Expr::Direct { expr: ir::DirectExpr::Float { value } };

            // There are no such things as float immediates for x86. Instead, we must make each float a variable
            // binding (in the compiler, there must be a `Location` for floats). For example `let a: float = 1.2 + 2.3`
            // should translate to `let temp1 = 1.2; let temp2 = 2.3; let a = temp1 + temp2`.
            let identifier = gen_temp_identifier();
            bindings.push(ir::Expr::Let { id: identifier.to_string(), init_expr: Box::new(float_expr) });
            type_checker
                .identifier_types
                .insert(identifier.to_string(), SolisType::Float);

            (
                ir::Expr::Direct { expr: ir::DirectExpr::Id { value: identifier } },
                SolisType::Float,
            )
        }
        ast::ExprKind::Let { id, init_expr, type_reference } => {
            let (init_expr, init_type) = translate_expr(*init_expr, type_checker, bindings);
            type_checker.type_check_let(&id, init_type.clone(), type_reference, &expr.position);

            // Flatten out let bindings inside sub expressions as well.
            bindings.push(ir::Expr::Let { id: id.clone(), init_expr: Box::new(init_expr) });
            (ir::Expr::Direct { expr: ir::DirectExpr::Id { value: id } }, init_type)
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
            let operand_ir = to_direct(operand_ir, operand_type.clone(), type_checker, bindings);

            // Perform type coercion, if needed
            let (operand_ir, operand_type) =
                coerce_type(operand_ir, operand_type, operand_coercion, type_checker, bindings);

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
            let operand_1 = to_direct(operand_1, operand_1_type.clone(), type_checker, bindings);
            let operand_2 = to_direct(operand_2, operand_2_type.clone(), type_checker, bindings);

            // Perform type coercion, if needed
            let (operand_1, operand_1_type) =
                coerce_type(operand_1, operand_1_type, operand_1_coercion, type_checker, bindings);
            let (operand_2, operand_2_type) =
                coerce_type(operand_2, operand_2_type, operand_2_coercion, type_checker, bindings);

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
    }
}

// Converts a direct to another type, if given `expr_coercion`, by adding an additional binding.
// * return - (the (new) direct, and the type of the expression)
fn coerce_type(
    expr: ir::DirectExpr,
    expr_type: SolisType,
    expr_coercion: Option<SolisType>,
    type_checker: &mut TypeChecker,
    bindings: &mut Vec<ir::Expr>,
) -> (ir::DirectExpr, SolisType) {
    if let Some(expr_coercion) = expr_coercion {
        let direct_identifier = gen_temp_identifier();
        let init_expr = ir::Expr::TypeCoercion {
            expr: Box::new(expr),
            from_type: expr_type,
            to_type: expr_coercion.clone(),
        };
        bindings.push(ir::Expr::Let { id: direct_identifier.to_string(), init_expr: Box::new(init_expr) });
        type_checker
            .identifier_types
            .insert(direct_identifier.to_string(), expr_coercion.clone());

        (ir::DirectExpr::Id { value: direct_identifier }, expr_coercion)
    } else {
        (expr, expr_type)
    }
}

// Translates a `ir::Expr` into `ir::DirectExpr`.
// * bindings - where to put additional bindings that are needed to translate the expression (temporary let-bindings)
fn to_direct(
    expr: ir::Expr,
    expr_type: SolisType,
    type_checker: &mut TypeChecker,
    bindings: &mut Vec<ir::Expr>,
) -> ir::DirectExpr {
    if let ir::Expr::Direct { expr } = expr {
        expr
    } else {
        let direct_identifier = gen_temp_identifier();
        bindings.push(ir::Expr::Let { id: direct_identifier.to_string(), init_expr: Box::new(expr) });
        type_checker
            .identifier_types
            .insert(direct_identifier.to_string(), expr_type);

        ir::DirectExpr::Id { value: direct_identifier }
    }
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
