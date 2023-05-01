// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! The translator lowers the AST into the intermediate representation of the program. This is the first stage of the
//! back end of the compiler. The IR structures are defined in `ir.rs`, where the context and rationale are documented.
//!
//! As documented in `ir.rs`, the IR is like the AST except every operand must be a `Direct`. To translate operands that
//! are complex expressions (like unary or binary expressions), we add temporary variables for the translations of
//! each operands (this is called "lifting" in the source code), and substitute the identifier as a Direct into the
//! original expression.

use ir_re::ir::{self, Type};
use ir_re::translate_expr::translate_expr;
use ir_re::type_checker::TypeChecker;
use parser::ast;
use std::rc::Rc;
use utils::context::Context;

/// Main translator function, which returns a `ir::Program`.
/// * program: output from the parser
pub fn translate<'a>(program: ast::Program<'a>, context: &'a Context) -> ir::Program<'a> {
    // Create the type_checker with lifetime 't (out of scope after this function)
    let mut type_checker = TypeChecker::new(context);

    ir::Program {
        functions: vec![],
        body: translate_block(program.body, &mut type_checker).0,
    }
}

// Translates a `ast::Block` into a `ir::Block`.
fn translate_block<'a: 't, 't>(
    block: ast::Block<'a>,
    type_checker: &'t mut TypeChecker<'a>,
) -> (ir::Block<'a>, Rc<Type>) {
    let num_exprs = block.exprs.len();
    let mut exprs = vec![];
    let mut result_type: Rc<Type> = Rc::new(Type::Int);

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
    ir::DirectExpr::Id { value: &direct_identifier, id_type: Rc::clone(expr_type) }
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

// // Translates a `ir::Expr` into `ir::DirectExpr::Id` by adding a temporary let-binding.
// fn to_binding(expr: ir::Expr, expr_type: Type, bindings: &mut Vec<ir::Expr>) -> ir::DirectExpr {
//     let direct_identifier = gen_temp_identifier();
//     bindings.push(ir::Expr::Let { id: direct_identifier.to_string(), init_expr: Box::new(expr) });
//     ir::DirectExpr::Id { value: direct_identifier, id_type: expr_type }
// }

// /// Translates a `ast::Program` into a `ir::Program`
// pub fn translate_program<'a>(context: &'a Context, program: ast::Program<'a>) -> ir::Program<'a> {
//     let mut type_checker = TypeChecker::new(context, &HashMap::new());

//     let mut functions_map = HashMap::new();
//     for function in &program.functions {
//         if functions_map.insert(function.id, function).is_some() {
//             compilation_error(
//                 context,
//                 ErrorPosition::Span(&function.id_position),
//                 &format!("Function`{}` has already been declared", function.id),
//             )
//         }
//     }

//     let mut type_checker = TypeChecker::new(context, &functions_map);

//     // Translate functions.
//     let mut functions = vec![];
//     for function in &program.functions {
//         // functions.push(translate_function(&mut type_checker, function));
//     }

//     let (body, _) = translate_block(&mut type_checker, &program.body);
//     ir::Program { functions, body }
// }

// Translates a `ast::Function` into a `ir::Function`
// fn translate_function<'a>(type_checker: &'a mut TypeChecker<'a>, function: &'a ast::Function<'a>) -> ir::Function<'a> {
//     todo!()
// let mut type_checker = TypeChecker::inherited(type_checker);

// // Bind parameters in the function scope.
// for param in &function.params {
//     type_checker.bind_variable(&param.id, &param.type_reference);
// }

// let (body, return_type) = translate_block(&mut type_checker, &function.body);

// // Type check the function
// type_checker.type_check_function(&function.id, return_type, &function.id_position);

// ir::Function {
//     id: function.id,
//     params: function.params.iter().map(|p| p.id).collect(),
//     body,
// }
// }
