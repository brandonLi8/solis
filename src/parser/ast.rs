// Copyright © 2022 Brandon Li. All rights reserved.

//! Definition of the AST that the Solis parser produces.

#[derive(PartialEq, Debug)]
pub struct Program {
    pub body: Expr,
}

#[derive(PartialEq, Debug)]
pub enum Expr {
    Let {
        id: String,
        static_type: String,
        expr: Box<Expr>,
    },
    Int {
        value: i32,
    },
    Id {
        value: String,
    },
    Do {
        exprs: Vec<Expr>,
    },
    Plus {
        operand1: Box<Expr>,
        operand2: Box<Expr>,
    },
    Minus {
        operand1: Box<Expr>,
        operand2: Box<Expr>,
    },
    Times {
        operand1: Box<Expr>,
        operand2: Box<Expr>,
    },
    Divide {
        operand1: Box<Expr>,
        operand2: Box<Expr>,
    },
    Mod {
        operand1: Box<Expr>,
        operand2: Box<Expr>,
    },
}
