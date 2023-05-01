// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! An AST (abstract syntax tree) is a tree representation of the *semantics* of any Solis program.
//! The job of the parser is to transform tokens (representation of *syntax*) into this representation (*semantics*).
//! This file contains the definitions of the AST that the Solis parser produces.

use utils::context::Position;
use utils::lang_common;

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
    pub id_position: Position,
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
        id_position: Position,
        type_reference: Type,
        init_expr: Box<Expr<'a>>,
    },
    If {
        condition: Box<Expr<'a>>,
        then_block: Block<'a>,
        else_block: Option<Block<'a>>,
        if_position: Position,
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
        position: Position,
    },
    UnaryExpr {
        kind: UnaryExprKind,
        operand: Box<Expr<'a>>,
        operator_position: Position,
    },
    BinaryExpr {
        kind: BinaryExprKind,
        operand_1: Box<Expr<'a>>,
        operand_2: Box<Expr<'a>>,
        operator_position: Position,
    },
    Call {
        id: &'a str,
        args: Vec<Expr<'a>>,
    },
}

pub type Type = lang_common::Type;
pub type UnaryExprKind = lang_common::UnaryExprKind;
pub type BinaryExprKind = lang_common::BinaryExprKind;
