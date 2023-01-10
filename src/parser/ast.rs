// Copyright © 2022 Brandon Li. All rights reserved.

//! An AST (abstract syntax tree) is a tree representation of the *semantics* of any Solis program.
//! The job of the parser is to transform tokens (representation of *syntax*) into this representation (*semantics*).
//! This file is the definitions of the AST that the Solis parser produces.

#[derive(PartialEq, Debug)]
pub struct Program {
    pub body: Expr,
}

#[derive(PartialEq, Debug)]
pub enum Expr {
    Let { id: String, type_reference: String, init_expr: Box<Expr> },
    Int { value: i32 },
    Id { value: String },
    Do { exprs: Vec<Expr> },
    Plus { operand_1: Box<Expr>, operand_2: Box<Expr> },
    Minus { operand_1: Box<Expr>, operand_2: Box<Expr> },
    Times { operand_1: Box<Expr>, operand_2: Box<Expr> },
    Divide { operand_1: Box<Expr>, operand_2: Box<Expr> },
    Mod { operand_1: Box<Expr>, operand_2: Box<Expr> },
}
