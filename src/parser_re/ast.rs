// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! An AST (abstract syntax tree) is a tree representation of the *semantics* of any Solis program.
//! The job of the parser is to transform tokens (representation of *syntax*) into this representation (*semantics*).
//! This file contains the definitions of the AST that the Solis parser produces.

#[derive(Debug)]
pub struct Program<'a> {
    pub functions: Vec<Function<'a>>,
    pub body: Block<'a>,
}

#[derive(Debug)]
pub struct Block<'a> {
    pub exprs: Vec<Expr<'a>>,
}

#[derive(Debug)]
pub struct Function<'a> {
    pub id: &'a str,
    pub params: Vec<Param<'a>>,
    pub return_type: Type,
    pub body: Block<'a>,
}

#[derive(Debug)]
pub struct Param<'a> {
    pub id: &'a str,
    pub type_reference: Type,
}

#[derive(Debug)]
pub enum Expr<'a> {
    Let {
        id: &'a str,
        type_reference: Type,
        init_expr: Box<Expr<'a>>,
    },
    If {
        condition: Box<Expr<'a>>,
        then_block: Block<'a>,
        else_block: Option<Block<'a>>,
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
        value: &'a str,
    },
    UnaryExpr {
        kind: UnaryExprKind,
        operand: Box<Expr<'a>>,
    },
    BinaryExpr {
        kind: BinaryExprKind,
        operand_1: Box<Expr<'a>>,
        operand_2: Box<Expr<'a>>,
    },
    Call {
        id: &'a str,
        args: Vec<Expr<'a>>,
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
