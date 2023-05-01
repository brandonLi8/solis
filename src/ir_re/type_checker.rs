// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Static type checker. It was decided that type checking would be done at the IR translation step. See
//! `https://github.com/brandonLi8/solis/issues/25`. This saves one pass (performance) and allows the IR to be
//! a simpler representation (does not need any `position` ranges). Thus, the `type_checker` also performs all other
//! compile time checks such as use of undefined variables/identifiers, arity checking, etc.
//!
//! The type checker works in conjunction with the translator (see `translator.rs`). The `translator` traverses down the
//! AST tree, and flattens by post-operating on the sub results. The type checker is written to post-operate as well,
//! and provides helper functions to ensure each sub result is correct.
//!
//! Note that for modularity, this file defines the basic TypeChecker types and the implementation of specific type
//! checking methods are distributed in the translator files.

use ir_re::ir::Type;
use std::collections::HashMap;
use std::rc::Rc;
use utils::context::{Context, Position};
use utils::error_messages::{compilation_error, internal_compiler_error, ErrorPosition};

/// Type Checker for each scope of the program.
pub struct TypeChecker<'a> {
    /// compilation context
    pub context: &'a Context,

    /// variables that are already been declared (added *after* translating `let`), as a map to their type
    defined_identifiers: HashMap<&'a str, Rc<Type>>,

    /// variables that are currently being declared (added *before* translating `let`), as a map to their type.
    /// this is needed for situations like `let a = a`, to catch using the identifier as it is being defined.
    reserved_identifiers: HashMap<&'a str, Rc<Type>>,
}

impl<'a> TypeChecker<'a> {
    /// Type Checker constructor.
    ///
    /// * context: compilation context
    pub fn new(context: &'a Context) -> Self {
        TypeChecker {
            context,
            defined_identifiers: HashMap::new(),
            reserved_identifiers: HashMap::new(),
        }
    }

    /// Gets the type of variable (declared). If the variable has not been declared, a `compilation_error` is created.
    ///
    /// * id - the identifier name
    /// * position - the position of this identifier
    pub fn get_variable_type(&self, id: &str, position: &Position) -> &Rc<Type> {
        match self.defined_identifiers.get(id) {
            None => compilation_error(
                self.context,
                ErrorPosition::Span(position),
                &format!("Undeclared variable `{id}`"),
            ),
            Some(identifier_type) => identifier_type,
        }
    }

    /// Marks the variable as reserved (currently being declared). If the variable has already been declared or reserved
    /// a `compilation_error` is created.
    ///
    /// * id - the identifier name
    /// * id_type - the type of the identifier
    /// * position - the position of this identifier
    pub fn reserve_variable(&mut self, id: &'a str, id_type: Type, position: &Position) {
        if self.reserved_identifiers.contains_key(id) || self.defined_identifiers.contains_key(id) {
            compilation_error(
                self.context,
                ErrorPosition::Span(position),
                &format!("Variable `{id}` is already declared in this scope"),
            )
        };

        self.reserved_identifiers.insert(id, Rc::new(id_type));
    }

    /// Moves the variable from reserved to declared (already declared). If the variable has already been declared a
    /// `compilation_error` is created. If the variable was not previously reserved, a `internal_compiler_error` is
    /// created.
    ///
    /// * id - the identifier name
    /// * position - the position of this identifier
    pub fn declare_reserved_variable(&mut self, id: &'a str, position: &Position) {
        if let Some(id_type) = self.reserved_identifiers.remove(id) {
            if self.defined_identifiers.contains_key(id) {
                compilation_error(
                    self.context,
                    ErrorPosition::Span(position),
                    &format!("Variable `{id}` is already declared in this scope"),
                )
            };

            self.force_declare_variable(id, id_type);
        } else {
            internal_compiler_error("variable was never reserved first")
        }
    }

    /// Marks the variable as declared, even if the variable has already been declared.
    ///
    /// * id - the identifier name
    /// * id_type - the type of the identifier
    /// * position - the position of this identifier
    pub fn force_declare_variable(&mut self, id: &'a str, id_type: Rc<Type>) {
        self.defined_identifiers.insert(id, id_type);
    }

    /// Gets the type of reserved variable (currently being declared). If the variable has not been reserved, a
    /// `internal_compiler_error` is created.
    ///
    /// * id - the identifier name
    pub fn get_reserved_variable_type(&self, id: &str) -> &Type {
        match self.reserved_identifiers.get(id) {
            None => internal_compiler_error("reserved variable not found"),
            Some(identifier_type) => identifier_type,
        }
    }
}

//     /// Constructs a `TypeChecker` from another `TypeChecker`, with the `identifier_types` cloned
//     pub fn inherited(type_checker: &TypeChecker<'a>) -> Self {
//         TypeChecker {
//             file: type_checker.file,
//             identifier_types: type_checker.identifier_types.clone(),
//             functions: type_checker.functions.clone(),
//         }
//     }

//     /// Type checks a let expression.
//     pub fn type_check_let(&mut self, id: &String, init_expr_type: Type, type_reference: Type, position: &Range<usize>) {
//         if type_reference != init_expr_type {
//             compilation_error(
//                 self.file,
//                 position,
//                 &format!("Mismatched types, expected `{init_expr_type}`, but found `{type_reference}`"),
//             )
//         }

//         self.set_declared_variable_type(id, type_reference, position);
//     }

//     /// Type checks a if expression.
//     /// * return - the type of the result expression
//     pub fn type_check_if(
//         &mut self,
//         condition_type: Type,
//         then_block_type: Type,
//         else_block_type: Option<Type>,
//         position: &Range<usize>,
//     ) -> Type {
//         if condition_type != Type::Bool {
//             compilation_error(
//                 self.file,
//                 position,
//                 &format!("`if` condition expected type `bool`, instead found `{condition_type}`"),
//             )
//         }
//         if let Some(else_block_type) = else_block_type {
//             if else_block_type != then_block_type {
//                 compilation_error(
//                     self.file,
//                     position,
//                     &format!("Mismatched types on `if` branches, `{then_block_type}` and `{else_block_type}`"),
//                 )
//             }
//             then_block_type
//         } else {
//             // If expressions with no else block evaluate to the unit type
//             Type::Unit
//         }
//     }

//     /// Type checks unary expressions
//     /// * return: (
//     ///     - the type of the result expression,
//     ///     - the type that the operand needs to be coerced into, if at all
//     /// )
//     pub fn type_check_unary_expr(
//         &self,
//         unary_expr_kind: &ir::UnaryExprKind,
//         operand_type: Type,
//         position: &Range<usize>,
//     ) -> (Type, Option<Type>) {
//         match unary_expr_kind {
//             ir::UnaryExprKind::Not => {
//                 if operand_type != Type::Bool {
//                     compilation_error(
//                         self.file,
//                         position,
//                         &format!(
//                             "Mismatched types. `{unary_expr_kind:?}` operator expected `bool`, found `{operand_type}`"
//                         ),
//                     )
//                 }
//                 (Type::Bool, None)
//             }

//             ir::UnaryExprKind::Negative => {
//                 if operand_type != Type::Int && operand_type != Type::Float {
//                     compilation_error(
//                         self.file,
//                         position,
//                         &format!(
//                             "Mismatched types. `{unary_expr_kind:?}` operator expected `int` or `float`, found `{operand_type}`"
//                         ),
//                     )
//                 }
//                 (operand_type, None)
//             }
//         }
//     }

//     /// Type checks binary expressions.
//     /// * return: (
//     ///     - the type of the result expression,
//     ///     - the type that the `operand_1` needs to be coerced into, if at all
//     ///     - the type that the `operand_2` needs to be coerced into, if at all
//     /// )
//     pub fn type_check_binary_expr(
//         &self,
//         binary_expr_kind: &ir::BinaryExprKind,
//         operand_1_type: Type,
//         operand_2_type: Type,
//         position: &Range<usize>,
//     ) -> (Type, Option<Type>, Option<Type>) {
//         match binary_expr_kind {
//             // For numerical operators, ensure both operands are integers/floats
//             ir::BinaryExprKind::Plus
//             | ir::BinaryExprKind::Minus
//             | ir::BinaryExprKind::Times
//             | ir::BinaryExprKind::Divide
//             | ir::BinaryExprKind::Mod => {
//                 if !matches!(operand_1_type, Type::Int | Type::Float)
//                     || !matches!(operand_2_type, Type::Int | Type::Float)
//                 {
//                     compilation_error(
//                       self.file,
//                       position,
//                       &format!("Bad operand types for `{binary_expr_kind:?}` operator: `{operand_1_type}` and `{operand_2_type}`")
//                     )
//                 }

//                 let operand_1_is_float = matches!(operand_1_type, Type::Float);
//                 let operand_2_is_float = matches!(operand_2_type, Type::Float);

//                 if operand_1_is_float || operand_2_is_float {
//                     (
//                         Type::Float,
//                         if operand_1_is_float { None } else { Some(Type::Float) },
//                         if operand_2_is_float { None } else { Some(Type::Float) },
//                     )
//                 } else {
//                     (Type::Int, None, None)
//                 }
//             }

//             // For comparison operators, ensure both operands are integers/Floats
//             ir::BinaryExprKind::LessThan
//             | ir::BinaryExprKind::LessThanOrEquals
//             | ir::BinaryExprKind::MoreThan
//             | ir::BinaryExprKind::MoreThanOrEquals => {
//                 if !matches!(operand_1_type, Type::Int | Type::Float)
//                     || !matches!(operand_2_type, Type::Int | Type::Float)
//                 {
//                     compilation_error(
//                       self.file,
//                       position,
//                       &format!("Bad operand types for `{binary_expr_kind:?}` operator: `{operand_1_type}` and `{operand_2_type}`")
//                     )
//                 }

//                 let operand_1_is_float = matches!(operand_1_type, Type::Float);
//                 let operand_2_is_float = matches!(operand_2_type, Type::Float);

//                 if operand_1_is_float || operand_2_is_float {
//                     (
//                         Type::Bool,
//                         if operand_1_is_float { None } else { Some(Type::Float) },
//                         if operand_2_is_float { None } else { Some(Type::Float) },
//                     )
//                 } else {
//                     (Type::Bool, None, None)
//                 }
//             }

//             // For equality, ensure that both operands are the same type.
//             ir::BinaryExprKind::EqualsEquals | ir::BinaryExprKind::NotEquals => {
//                 if operand_1_type != operand_2_type {
//                     compilation_error(
//                       self.file,
//                       position,
//                       &format!("Mismatched types. `{binary_expr_kind:?}` cannot be used with `{operand_1_type}` and `{operand_2_type}`")
//                     )
//                 }
//                 (Type::Bool, None, None)
//             }
//         }
//     }

//     /// Type checks a function declaration.
//     pub fn type_check_function(&mut self, id: &String, found_return_type: Type, position: &Range<usize>) {
//         match self.functions.get(id) {
//             Some((return_type, _)) => {
//                 if *return_type != found_return_type {
//                     compilation_error(
//                         self.file,
//                         position,
//                         &format!("Mismatched return types, expected `{return_type}`, but found `{found_return_type}`"),
//                     )
//                 }
//             }
//             _ => internal_compiler_error("function not found"),
//         }
//     }

//     /// Type checks a call expression (specifically the parameters), and returns the return type of the function.
//     pub fn type_check_call(
//         &mut self,
//         id: &String,
//         position: &Range<usize>,
//         arg_types: Vec<Type>,
//         arg_positions: Vec<Range<usize>>,
//     ) -> Type {
//         // Check function existence
//         let (return_type, param_types) = self
//             .functions
//             .get(id)
//             .unwrap_or_else(|| compilation_error(self.file, position, &format!("Unknown function `{id}`")));

//         // Check function arity.
//         if param_types.len() != arg_types.len() {
//             compilation_error(
//                 self.file,
//                 position,
//                 &format!(
//                     "This function takes {} arguments but {} were supplied",
//                     param_types.len(),
//                     arg_types.len()
//                 ),
//             );
//         }

//         // Param type matching
//         for (param_type, (arg_type, arg_position)) in param_types.iter().zip(arg_types.iter().zip(arg_positions.iter()))
//         {
//             if param_type != arg_type {
//                 compilation_error(
//                     self.file,
//                     arg_position,
//                     &format!("Expected argument type `{param_type}`, found {arg_type}"),
//                 )
//             }
//         }

//         return_type.clone()
//     }

//     /// Registers a function. If the function has already been declared, a `compilation_error` is created.
//     pub fn register_function(
//         &mut self,
//         id: &'a String,
//         return_type: Type,
//         param_types: Vec<Type>,
//         position: &Range<usize>,
//     ) {
//         if self.functions.insert(id, (return_type, param_types)).is_some() {
//             compilation_error(
//                 self.file,
//                 position,
//                 &format!("Function`{id}` has already been declared"),
//             )
//         }
//     }
// }
