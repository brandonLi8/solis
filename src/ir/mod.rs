// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! The IR module is responsible for lowering the AST into the intermediate representation.

pub mod ir;
pub mod translator;

mod translate_binary_expr;
mod translate_expr;
mod translate_function;
mod translate_if;
mod translate_let;
mod translate_unary_expr;
mod type_checker;
