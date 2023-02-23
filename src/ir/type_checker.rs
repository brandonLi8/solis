// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Static type checker. It was decided that type checking would be done at the IR translation step. See
//! `https://github.com/brandonLi8/solis/issues/25`. This saves one pass (performance) and allows the IR to be
//! a simpler representation (does not need any `position` ranges). Thus, the `type_checker` also performs all other
//! compile time checks such as undefined variables, arity checking, etc.
//!
//! The type checker works in conjunction with the translator (see `translator.rs`). The `translator` traverses down the
//! AST tree, and flattens by post-operating on the sub results. The type checker is written to post-operate as well,
//! and provides helper functions to ensure each sub result is correct.

use error_messages::compilation_error;
use ir::ir;
use std::collections::HashMap;
use std::ops::Range;
use File;

/// Different Types for Solis
#[derive(PartialEq, Clone)]
pub enum SolisType {
    Int,
    Bool,
    Float,
    Custom(String),
}

/// Type Checker for each scope of the program.
pub struct TypeChecker<'a> {
    /// Maps identifiers/variables that have been seen to their types.
    pub identifier_types: HashMap<String, SolisType>,

    /// The original Solis input file, for error messaging purposes.
    file: &'a File,
}

impl<'a> TypeChecker<'a> {
    /// Type Checker constructor.
    /// * file: the original Solis file
    pub fn new(file: &'a File) -> Self {
        TypeChecker { file, identifier_types: HashMap::new() }
    }

    /// Type checks a let expression.
    pub fn type_check_let(
        &mut self,
        id: &String,
        init_expr_type: SolisType,
        type_reference: String,
        position: &Range<usize>,
    ) {
        // Convert the type_reference to a SolisType. TODO check that the type exists. TODO: should do this at parse step.
        let type_reference = match type_reference.as_str() {
            "int" => SolisType::Int,
            "bool" => SolisType::Bool,
            "float" => SolisType::Float,
            _ => SolisType::Custom(type_reference),
        };

        if type_reference != init_expr_type {
            compilation_error(
                self.file,
                position,
                &format!("Mismatched types, expected `{init_expr_type}`, but found `{type_reference}`"),
            )
        }

        // Variable cannot be re-declared
        if self.identifier_types.insert(id.to_string(), init_expr_type).is_some() {
            compilation_error(
                self.file,
                position,
                &format!("Variable `{id}` is already declared in this scope"),
            )
        }
    }

    /// Type checks unary expressions, returning the type of the result expression.
    pub fn type_check_unary_expr(
        &self,
        unary_expr_kind: &ir::UnaryExprKind,
        operand_type: SolisType,
        position: &Range<usize>,
    ) -> SolisType {
        match unary_expr_kind {
            ir::UnaryExprKind::Not => {
                if operand_type != SolisType::Bool {
                    compilation_error(
                        self.file,
                        position,
                        &format!(
                            "Mismatched types. `{unary_expr_kind:?}` operator expected `bool`, found `{operand_type}`"
                        ),
                    )
                }
                SolisType::Bool
            }

            ir::UnaryExprKind::Negative => {
                if operand_type != SolisType::Int && operand_type != SolisType::Float {
                    compilation_error(
                        self.file,
                        position,
                        &format!(
                            "Mismatched types. `{unary_expr_kind:?}` operator expected `int` or `float`, found `{operand_type}`"
                        ),
                    )
                }
                operand_type
            }
        }
    }

    /// Type checks binary expressions, returning the type of the result expression.
    pub fn type_check_binary_expr(
        &self,
        binary_expr_kind: &ir::BinaryExprKind,
        operand_1_type: SolisType,
        operand_2_type: SolisType,
        position: &Range<usize>,
    ) -> SolisType {
        match binary_expr_kind {
            // For numerical operators, ensure both operands are integers
            ir::BinaryExprKind::Plus
            | ir::BinaryExprKind::Minus
            | ir::BinaryExprKind::Times
            | ir::BinaryExprKind::Divide
            | ir::BinaryExprKind::Mod => {
                if !matches!(operand_1_type, SolisType::Int | SolisType::Float)
                    || !matches!(operand_2_type, SolisType::Int | SolisType::Float)
                {
                    compilation_error(
                      self.file,
                      position,
                      &format!("Bad operand types for `{binary_expr_kind:?}` operator: `{operand_1_type}` and `{operand_2_type}`")
                    )
                }

                if operand_1_type == SolisType::Float || operand_2_type == SolisType::Float {
                    SolisType::Float
                } else {
                    SolisType::Int
                }
            }

            // For comparison operators, ensure both operands are integers
            ir::BinaryExprKind::LessThan
            | ir::BinaryExprKind::LessThanOrEquals
            | ir::BinaryExprKind::MoreThan
            | ir::BinaryExprKind::MoreThanOrEquals => {
                if !matches!(operand_1_type, SolisType::Int | SolisType::Float)
                    || !matches!(operand_2_type, SolisType::Int | SolisType::Float)
                {
                    compilation_error(
                      self.file,
                      position,
                      &format!("Bad operand types for `{binary_expr_kind:?}` operator: `{operand_1_type}` and `{operand_2_type}`")
                    )
                }
                SolisType::Bool
            }

            // For equality, ensure that both operands are the same type.
            ir::BinaryExprKind::EqualsEquals | ir::BinaryExprKind::NotEquals => {
                if operand_1_type != operand_2_type {
                    compilation_error(
                      self.file,
                      position,
                      &format!("Mismatched types. `{binary_expr_kind:?}` cannot be used with `{operand_1_type}` and `{operand_2_type}`")
                    )
                }
                SolisType::Bool
            }
        }
    }

    /// Gets the type of variable. If the variable has not been declared, a `compilation_error` is created
    pub fn get_type(&self, id: &String, position: &Range<usize>) -> SolisType {
        let t = self.identifier_types.get(id);
        if t.is_none() {
            compilation_error(self.file, position, &format!("Undeclared variable `{id}`"))
        }
        t.unwrap().clone()
    }
}
