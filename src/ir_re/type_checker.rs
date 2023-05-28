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
use ir_re::translate_function::ProcedureTable;
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

    /// procedure table, for type checking `Call` expressions.
    pub procedure_table: Rc<ProcedureTable<'a>>,
}

impl<'a> TypeChecker<'a> {
    /// Type Checker constructor.
    ///
    /// * procedure_table
    /// * context: compilation context
    pub fn new(procedure_table: ProcedureTable<'a>, context: &'a Context) -> Self {
        TypeChecker {
            context,
            defined_identifiers: HashMap::new(),
            reserved_identifiers: HashMap::new(),
            procedure_table: Rc::new(procedure_table),
        }
    }

    /// Creates a type checker with the scope inherited from a already existing type checker.
    pub fn inherit_scope(type_checker: &TypeChecker<'a>) -> Self {
        TypeChecker {
            context: type_checker.context,
            defined_identifiers: type_checker.defined_identifiers.clone(),
            reserved_identifiers: type_checker.reserved_identifiers.clone(),
            procedure_table: Rc::clone(&type_checker.procedure_table),
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
