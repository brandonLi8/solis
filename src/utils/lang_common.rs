// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! This file defines structs that are shared between the AST and the IR.
//! See the `parser` and `ir` modules for more context.

use derive_more::Display;

#[derive(PartialEq, Display, Clone, Debug)]
pub enum Type {
    #[display(fmt = "<unit>")]
    Unit,

    #[display(fmt = "int")]
    Int,

    #[display(fmt = "bool")]
    Bool,

    #[display(fmt = "float")]
    Float,
}

#[derive(Debug)]
pub enum UnaryExprKind {
    Not,
    Negative,
}

#[derive(Debug)]
pub enum BinaryExprKind {
    Plus,
    Minus,
    Times,
    Divide,
    Mod,
    LessThan,
    LessThanOrEquals,
    MoreThan,
    MoreThanOrEquals,
    EqualsEquals,
    NotEquals,
}
