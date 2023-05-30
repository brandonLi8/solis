// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Translates functions. See `translator.rs` for context.

use ir_re::ir::{self, Type};
use ir_re::translate_expr::translate_expr;
use ir_re::translator::{lift, translate_block};
use ir_re::type_checker::TypeChecker;
use parser::ast;
use std::collections::HashMap;
use std::rc::Rc;
use utils::context::{Context, Position};
use utils::error_messages::{compilation_error, internal_compiler_error, ErrorPosition};
use utils::Set;

/// A ProcedureTable is used to store information about functions definitions.
/// Table looks like:
///
/// | Key=Procedure Name | Return Type | Parameter Types |
/// | -------------------| ----------- | --------------- |
pub type ProcedureTable<'a> = HashMap<&'a str, (Rc<Type>, Vec<Type>)>;

/// Creates a ProcedureTable from a `Vec<ast::Function>`
///
/// * functions: the parsed functions
/// * context: compilation context
pub fn create_procedure_table<'a>(functions: &Vec<ast::Function<'a>>, context: &'a Context) -> ProcedureTable<'a> {
    let mut procedure_table = ProcedureTable::new();

    for procedure in functions {
        // Ensure uniqueness of procedure definitions.
        if procedure_table.contains_key(procedure.id) {
            compilation_error(
                context,
                ErrorPosition::Span(&procedure.id_position),
                &format!("Function`{}` has already been declared", procedure.id),
            )
        }

        procedure_table.insert(
            procedure.id,
            // In this situation, we must clone the `Types` of the function. When we translate functions, we move
            // the `ast::Function` out and convert it to a `ir::Function`. But we must keep around the return/param
            // types for Call expressions, so we pre-clone these types here.
            (
                Rc::new(procedure.return_type.clone()),
                procedure.params.iter().map(|p| p.type_reference.clone()).collect(),
            ),
        );
    }

    procedure_table
}

/// Translates a `Vec<ast::Function>` into a `Vec<ir::Function>`
///
/// * functions - the ast functions
/// * bindings - where to put additional bindings that are needed to translate the functions (temporary let-bindings)
/// * type_checker - type checker for this scope
pub fn translate_functions<'a, 'b, 't>(
    functions: Vec<ast::Function<'a>>,
    type_checker: &'t mut TypeChecker<'a>,
) -> Vec<ir::Function<'a>>
where
    'a: 't,
    't: 'b,
{
    functions.into_iter().map(|function| translate_function(function, type_checker)).collect()
}

/// Translates a `ast::Function` into a `ir::Function`
///
/// * function - the ast function
/// * bindings - where to put additional bindings that are needed to translate the function (temporary let-bindings)
/// * type_checker - type checker for this scope
pub fn translate_function<'a, 'b, 't>(
    function: ast::Function<'a>,
    type_checker: &'t mut TypeChecker<'a>,
) -> ir::Function<'a>
where
    'a: 't,
    't: 'b,
{
    // Each function has its own local scope.
    let mut type_checker = &mut TypeChecker::inherit_scope(type_checker);

    // Bind parameters in the function scope, and collect the param identifiers.
    let mut params = vec![];

    for param in function.params.into_iter() {
        type_checker.force_declare_variable(&param.id, param.type_reference.into());
        params.push(param.id);
    }

    let (body, return_type) = translate_block(function.body, &mut type_checker);

    // Type check the function
    type_checker.type_check_function(&function.id, return_type, &function.id_position);

    ir::Function { id: &function.id, params, body }
}

/// Translates a `ast::Expr::Call` into a `ir::Expr::Call`
///
/// * expr - the ast expression
/// * bindings - where to put additional bindings that are needed to translate the expression (temporary let-bindings)
/// * type_checker - type checker for this scope
pub fn translate_call<'a, 'b, 't>(
    expr: ast::Expr<'a>,
    bindings: &'b mut Vec<ir::Expr<'a>>,
    type_checker: &'t mut TypeChecker<'a>,
) -> (ir::Expr<'a>, Rc<Type>)
where
    'a: 't,
    't: 'b,
{
    if let ast::Expr::Call { id, args, id_position } = expr {
        // We need to lift all arguments into directs.
        let mut lifted_args = vec![];

        // For type checking purposes.
        let mut arg_types = vec![];
        let mut arg_positions = vec![];

        for (arg, arg_position) in args.into_iter() {
            let (arg, arg_type) = translate_expr(arg, bindings, type_checker);

            // Lift the argument
            lifted_args.push(lift(arg, &arg_type, bindings));
            arg_types.push(arg_type);
            arg_positions.push(arg_position);
        }

        let return_type = type_checker.type_check_call(id, &id_position, arg_types, arg_positions);
        (
            ir::Expr::Call { id, args: lifted_args, live_variables: Set::new() },
            return_type,
        )
    } else {
        internal_compiler_error("non ast::Expr::Call passed in")
    }
}

impl<'a> TypeChecker<'a> {
    // Type checks a function declaration.
    //
    // * id - the function identifier
    // * found_return_type - the return type of the translated body
    // * id_position - the position of the function name
    fn type_check_function(&mut self, id: &'a str, found_return_type: Rc<Type>, id_position: &Position) {
        match self.procedure_table.get(id) {
            Some((return_type, _)) => {
                if *return_type != found_return_type {
                    compilation_error(
                        self.context,
                        ErrorPosition::Span(id_position),
                        &format!("Mismatched return types, expected `{return_type}`, but found `{found_return_type}`"),
                    )
                }
            }
            _ => internal_compiler_error("function not found in procedure table"),
        }
    }

    // Type checks a call expression (specifically the parameters), and returns the return type of the function.
    //
    // * id - the function identifier
    // * id_position - the position of the function identifier
    // * arg_types - the types of the arguments
    // * arg_positions - the positions of the arguments
    //
    // * return - the return type of the function (from the declaration)
    fn type_check_call(
        &mut self,
        id: &'a str,
        id_position: &Position,
        arg_types: Vec<Rc<Type>>,
        arg_positions: Vec<Position>,
    ) -> Rc<Type> {
        // Check function existence
        let (return_type, param_types) = self.procedure_table.get(id).unwrap_or_else(|| {
            compilation_error(
                self.context,
                ErrorPosition::Span(id_position),
                &format!("Unknown function `{id}`"),
            )
        });

        // Check function arity.
        if param_types.len() != arg_types.len() {
            compilation_error(
                self.context,
                ErrorPosition::Span(id_position),
                &format!(
                    "This function takes {} arguments but {} were supplied",
                    param_types.len(),
                    arg_types.len()
                ),
            );
        }

        // Param type matching
        for (param_type, (arg_type, arg_position)) in param_types.iter().zip(arg_types.iter().zip(arg_positions.iter()))
        {
            if *param_type != **arg_type {
                compilation_error(
                    self.context,
                    ErrorPosition::Position(arg_position),
                    &format!("Expected argument type `{param_type}`, found {arg_type}"),
                )
            }
        }

        return_type.clone()
    }
}
