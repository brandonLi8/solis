// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! The `compiler` module generates the assembly instructions for the input program.

pub mod compiler;

mod compile_binary_expr;
mod compile_unary_expr;
mod symbol_table;
