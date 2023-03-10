// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! An AST (abstract syntax tree) is a tree representation of the *semantics* of any Solis program.
//! The job of the parser is to transform tokens (representation of *syntax*) into this representation (*semantics*).
//! This file contains the definitions of the AST that the Solis parser produces.

use std::ops::Range;

#[derive(PartialEq, Debug)]
pub struct Program {
    pub body: Block,
}

#[derive(PartialEq, Debug)]
pub struct Block {
    pub exprs: Vec<Expr>,
}

#[derive(PartialEq, Debug)]
pub struct Expr {
    pub kind: ExprKind,

    /// For error messaging purposes, linking the `position` of the expression to where it was in the source code.
    /// Note that this range doesn't correspond to the entire expression, as expressions consist of many tokens.
    /// Instead, the position might correspond to a singular key token for the expression (like the "let" for lets).
    pub position: Range<usize>,
}

#[derive(PartialEq, Debug)]
pub enum ExprKind {
    Let {
        id: String,
        type_reference: Type,
        init_expr: Box<Expr>,
    },
    If {
        condition: Box<Expr>,
        then_block: Block,
        else_block: Option<Block>,
    },
    Int {
        value: i64,
    },
    Bool {
        value: bool,
    },
    Float {
        value: f64,
    },
    Id {
        value: String,
    },
    UnaryExpr {
        kind: UnaryExprKind,
        operand: Box<Expr>,
    },
    BinaryExpr {
        kind: BinaryExprKind,
        operand_1: Box<Expr>,
        operand_2: Box<Expr>,
    },
}

#[derive(PartialEq, Clone, Debug)]
pub enum Type {
    Unit,
    Int,
    Bool,
    Float,
}

#[derive(PartialEq, Debug)]
pub enum UnaryExprKind {
    Not,
    Negative,
}

#[derive(PartialEq, Debug)]
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
