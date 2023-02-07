// Copyright © 2022 Brandon Li. All rights reserved.

//! The translator lowers the AST into the intermediate representation of the program. This is the first stage of the
//! back end of the compiler. The IR structures are defined in `ir.rs`, where the context and rationale are documented.
//!
//! As documented in `ir.rs`, the IR is like the AST except every operand must be a `Direct`. To translate operands that
//! are complex expressions (like unary or binary expressions), we add temporary variables for the translations of
//! each operands, and substitute the identifier as a Direct into the original expression.

use ir::ir;
use parser::ast;
use std::cell::RefCell;

/// Translates a `ast::Program` into a `ir::Program`
pub fn translate_program(program: ast::Program) -> ir::Program {
    ir::Program { body: translate_block(program.body) }
}

// Translates a `ast::Block` into a `ir::Block`
fn translate_block(block: ast::Block) -> ir::Block {
    let mut exprs = vec![];

    for expr in block.exprs {
        let translated_expr = translate_expr(expr, &mut exprs);
        exprs.push(translated_expr);
    }

    ir::Block { exprs }
}

// Translates a `ast::Expr` into a `ir::Expr>
// * bindings - where to put additional bindings that are needed to translate the expression (temporary let-bindings)
fn translate_expr(expr: ast::Expr, bindings: &mut Vec<ir::Expr>) -> ir::Expr {
    match expr {
        ast::Expr::Id { value } => ir::Expr::Direct { expr: ir::DirectExpr::Id { value } },
        ast::Expr::Int { value } => ir::Expr::Direct { expr: ir::DirectExpr::Int { value } },
        ast::Expr::Bool { value } => ir::Expr::Direct { expr: ir::DirectExpr::Bool { value } },
        ast::Expr::Let { id, init_expr, .. } => {
            ir::Expr::Let { id, init_expr: Box::new(translate_expr(*init_expr, bindings)) }
        }
        ast::Expr::UnaryExpr { kind, operand } => ir::Expr::UnaryExpr {
            kind: match kind {
                ast::UnaryExprKind::Not => ir::UnaryExprKind::Not,
                ast::UnaryExprKind::Negative => ir::UnaryExprKind::Negative,
            },
            operand: Box::new(to_direct(translate_expr(*operand, bindings), bindings)),
        },
        ast::Expr::BinaryExpr { kind, operand_1, operand_2 } => ir::Expr::BinaryExpr {
            kind: match kind {
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
            },
            operand_1: Box::new(to_direct(translate_expr(*operand_1, bindings), bindings)),
            operand_2: Box::new(to_direct(translate_expr(*operand_2, bindings), bindings)),
        },
    }
}

// Translates a `ir::Expr` into `ir::DirectExpr>`.
// * bindings - where to put additional bindings that are needed to translate the expression (temporary let-bindings)
fn to_direct(expr: ir::Expr, bindings: &mut Vec<ir::Expr>) -> ir::DirectExpr {
    if let ir::Expr::Direct { expr } = expr {
        expr
    } else {
        let direct_identifier = gen_temp_identifier();
        bindings.push(ir::Expr::Let { id: direct_identifier.to_string(), init_expr: Box::new(expr) });
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
