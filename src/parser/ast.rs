// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! An AST (abstract syntax tree) is a tree representation of the *semantics* of any Solis program.
//! The job of the parser is to transform tokens (representation of *syntax*) into this representation (*semantics*).
//! This file contains the definitions of the AST that the Solis parser produces.

use std::ops::Range;

#[derive(Debug)]
pub struct Program {
    pub functions: Vec<Function>,
    pub body: Block,
}

#[derive(Debug)]
pub struct Block {
    pub exprs: Vec<Expr>,
}

#[derive(Debug)]
pub struct Function {
    pub id: String,
    pub params: Vec<Param>,
    pub return_type: Type,
    pub body: Block,
    pub position: Range<usize>,
}

#[derive(Debug)]
pub struct Param {
    pub id: String,
    pub type_reference: Type,
}

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,

    /// For error messaging purposes, linking the `position` of the expression to where it was in the source code.
    /// Note that this range doesn't correspond to the entire expression, as expressions consist of many tokens.
    /// Instead, the position might correspond to a singular key token for the expression (like the "let" for lets).
    pub position: Range<usize>,
}

#[derive(Debug)]
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
    Call {
        id: String,
        args: Vec<Expr>,
    },
}

#[derive(Debug)]
pub enum Type {
    Unit,
    Int,
    Bool,
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
