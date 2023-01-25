// Copyright © 2022 Brandon Li. All rights reserved.

//! An AST (abstract syntax tree) is a tree representation of the *semantics* of any Solis program.
//! The job of the parser is to transform tokens (representation of *syntax*) into this representation (*semantics*).
//! This file contains the definitions of the AST that the Solis parser produces.

#[derive(PartialEq, Debug)]
pub struct Program {
    pub body: Block,
}

#[derive(PartialEq, Debug)]
pub struct Block {
    pub exprs: Vec<Expr>,
}

#[derive(PartialEq, Debug)]
pub enum Expr {
    Let {
        id: String,
        type_reference: String,
        init_expr: Box<Expr>,
    },
    Int {
        value: i64,
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
