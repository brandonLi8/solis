// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! The translator lowers the AST into the intermediate representation of the program. This is the first stage of the
//! back end of the compiler. The IR structures are defined in `ir.rs`, where the context and rationale are documented.
//!
//! As documented in `ir.rs`, the IR is like the AST except every operand must be a `Direct`. To translate operands that
//! are complex expressions (like unary or binary expressions), we add temporary variables for the translations of
//! each operands (this is called "lifting" in the source code), and substitute the identifier as a Direct into the
//! original expression.

use ir::ir::{self, Type};
use ir::translate_expr::translate_expr;
use ir::translate_function::{create_procedure_table, translate_functions};
use ir::type_checker::TypeChecker;
use parser::ast;
use std::rc::Rc;
use utils::context::Context;

/// Main translator function, which returns a `ir::Program`.
/// * program: output from the parser
pub fn translate<'a>(program: ast::Program<'a>, context: &'a Context) -> ir::Program<'a> {
    // Create the type_checker with lifetime 't (out of scope after this function)
    let mut type_checker = TypeChecker::new(create_procedure_table(&program.functions, context), context);

    ir::Program {
        functions: translate_functions(program.functions, &mut type_checker),
        body: translate_block(program.body, &mut type_checker).0,
    }
}

/// Translates a `ast::Block` into a `ir::Block`.
pub fn translate_block<'a: 't, 't>(
    block: ast::Block<'a>,
    type_checker: &'t mut TypeChecker<'a>,
) -> (ir::Block<'a>, Rc<Type>) {
    let num_exprs = block.exprs.len();
    let mut exprs = vec![];
    let mut result_type: Rc<Type> = Rc::new(Type::Unit);

    for (i, expr) in block.exprs.into_iter().enumerate() {
        // Collect temporary bindings that are needed to execute the next expression here, with lifetime 'b (out of
        // scope after each iteration)
        let mut bindings = vec![];
        let (translated_expr, expr_type) = translate_expr(expr, &mut bindings, type_checker);

        // Add the binding expressions to the block first before the result expression.
        exprs.append(&mut bindings);

        // Ignore top-level directs, unless it is the last expression in the block
        if i == num_exprs - 1 || !matches!(translated_expr, ir::Expr::Direct { .. }) {
            exprs.push(translated_expr);
        }
        result_type = expr_type;
    }

    (ir::Block { exprs }, result_type)
}

/// Lifts a `ir::Expr` into `ir::DirectExpr` (specifically `ir::DirectExpr::Id`), but only if the `ir::Expr` is not
/// already a direct. For example, `lift((+ 1, 2))` -> `let temp = 1 + 2; temp`, but `lift(a)` -> `a`.
///
/// * `expr` - the translated expression
/// * `expr_type` - the type of the expr
/// * `bindings` - where to put the additional binding
pub fn lift<'a: 'b, 'b>(
    expr: ir::Expr<'a>,
    expr_type: &Rc<Type>,
    bindings: &'b mut Vec<ir::Expr<'a>>,
) -> ir::DirectExpr<'a> {
    if let ir::Expr::Direct { expr } = expr {
        expr
    } else {
        force_lift(expr, expr_type, bindings)
    }
}

/// Lifts a `ir::Expr` into `ir::DirectExpr` (specifically `ir::DirectExpr::Id`), no matter what.
/// For example, `lift((+ 1, 2))` -> `let temp = 1 + 2; temp`.
///
/// * `expr` - the translated expression
/// * `expr_type` - the type of the expr
/// * `bindings` - where to put the additional binding
pub fn force_lift<'a: 'b, 'b>(
    expr: ir::Expr<'a>,
    expr_type: &Rc<Type>,
    bindings: &'b mut Vec<ir::Expr<'a>>,
) -> ir::DirectExpr<'a> {
    // Get a unique, temporary identifier.
    let direct_identifier = gen_temp_identifier();

    // Create the lifted binding.
    let binding = ir::Expr::Let { id: &direct_identifier, init_expr: Box::new(expr) };

    bindings.push(binding);
    ir::DirectExpr::Id { value: &direct_identifier, id_type: Rc::clone(&expr_type) }
}

/// Converts a direct to another type, if given `to_type`, by lifting the direct through a `Expr::TypeCoercion`.
///
/// * `expr` - the translated expression
/// * `from_type` - the type of the expr
/// * `to_type - the type to convert to
/// * `bindings` - where to put the additional binding
///
/// * return - (
///     - the result expression,
///     - and the type of the expression
///   )
pub fn coerce<'a: 'b, 'b>(
    expr: ir::DirectExpr<'a>,
    from_type: Rc<Type>,
    to_type: Option<Rc<Type>>,
    bindings: &'b mut Vec<ir::Expr<'a>>,
) -> (ir::DirectExpr<'a>, Rc<Type>) {
    if let Some(to_type) = to_type {
        // create the type_coercion expression
        let type_coercion = ir::Expr::TypeCoercion {
            expr: Box::new(expr),
            from_type: Rc::clone(&from_type),
            to_type: Rc::clone(&to_type),
        };

        // lift the type coercion
        (force_lift(type_coercion, &to_type, bindings), to_type)
    } else {
        (expr, from_type)
    }
}

// Generates a unique string (&str) that is used as the identifier of temporary lifts.
// The name cannot conflict with any source code names.
fn gen_temp_identifier<'a>() -> &'a str {
    thread_local! {
        pub static TAG: std::cell::RefCell<u32> = std::cell::RefCell::new(0);
    }

    let mut tag_value = 0;

    TAG.with(|tag| {
        tag_value = *tag.borrow();
        *tag.borrow_mut() += 1;
    });

    Box::leak(format!("@temp{}", tag_value).into_boxed_str())
}
