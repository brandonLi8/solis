// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! The IR (intermediate representation) for Solis programs. In compiler design, an IR is a middle representation
//! that is derived ("lowered"") from the AST, with a reduced set of features and is more conducive to optimizations.
//!
//! For Solis, the purpose of the IR is to perform register allocation optimizations. The rest of this comment provides
//! context for how it is useful for this optimization.
//!
//! ## Register Allocation Overview
//!   Register allocation attempts to minimize stack use in favor for registers. For example, the naive way to compile
//!   `(+ (+ (+ 1 0) 2) 3)`, writes the result of each sub-expression to the stack, a total of 4 times.
//!
//!   Register allocation assigns every (sub)expression to either a register or the stack. At run time, we place the
//!   result of the expression in the assigned location. It achieves the assignment by scanning each expression in a
//!   function block and creates a conflict graph, where we color the conflict graph to assign registers. We add every
//!   (sub)expression to this graph (since we are assigning every sub-expression a location), and we can do so by
//!   calculating when any pair of expressions are "live" at the same time.
//!
//!   However, the problem can be illustrated with this following example:
//!     Block [
//!       Let a = `(+ (+ (+ 1 0) 2) 3)`
//!       ...
//!     ]
//!   At the first expression, `0`, `1`, `2`, `3`, `(+ 1 0)`, `(+ (+ 1 0) 2)` and `(+ (+ (+ 1 0) 2) 3)` are all live at
//!   the first line, and are added to the conflict graph as a strongly connected component. Since they are direct
//!   neighbors, each of these sub expressions are assigned to 4 different locations. However, this is not optimal
//!   (can be done with 1 register!).
//!
//! ## IR
//!   The IR reconciles this problem by "flattening" deep sub expressions into a more expressions (called "let-lifting")
//!   For this example:
//!     Block [
//!       Let temp1 = (+ 1 0)             # Live: {temp1}
//!       Let temp2 = (+ temp1 2)         # Live: {temp2, temp1}
//!       Let temp3 = (+ temp2 3)         # Live: {temp3, temp2} (temp1 is not live anymore!)
//!       Let a = (+ temp2 3)
//!       ...
//!     ]
//!   Annotated on the right are the live variables at each expression. Since this expression is flattened, only 2
//!   temporary variables are live at the same time, which means the conflict graph looks like temp1 - temp2 - temp3,
//!   and only 2 registers are needed!
//!
//!   More formally, the IR enforces every operand to be either a literal or a (temp) variable. We call these `Directs`.
//!   Working with directs means that the operator does not have to do any more computation (naively would have to store
//!   results on the stack).

use std::rc::Rc;
use utils::{lang_common, Set};

#[derive(Debug)]
pub struct Program<'a> {
    pub functions: Vec<Function<'a>>,
    pub body: Block<'a>,
}

#[derive(Debug)]
pub struct Function<'a> {
    pub id: &'a str,
    pub params: Vec<&'a str>,
    pub body: Block<'a>,
}

#[derive(Debug)]
pub struct Block<'a> {
    pub exprs: Vec<Expr<'a>>,
}

#[derive(Debug)]
pub enum Expr<'a> {
    Direct {
        expr: DirectExpr<'a>,
    },
    Let {
        id: &'a str,
        init_expr: Box<Expr<'a>>,
    },
    If {
        condition: Box<DirectExpr<'a>>,
        then_block: Block<'a>,
        else_block: Option<Block<'a>>,
    },
    UnaryExpr {
        kind: UnaryExprKind,
        operand: Box<DirectExpr<'a>>,
        operand_type: Rc<Type>,
    },
    BinaryExpr {
        kind: BinaryExprKind,
        operand_1: Box<DirectExpr<'a>>,
        operand_2: Box<DirectExpr<'a>>,
        operand_type: Rc<Type>,
    },
    Call {
        id: &'a str,
        args: Vec<DirectExpr<'a>>,

        /// Variables that are live before the call occurs. This is filled in in the register_allocation analysis, and
        /// since they are live, they are the caller saved variables.
        live_variables: Set<&'a str>,
    },

    // Converts one type to another type. We do this in the IR layer instead of the compiler layer
    TypeCoercion {
        expr: Box<DirectExpr<'a>>,
        from_type: Rc<Type>,
        to_type: Rc<Type>,
    },
}

#[derive(Debug)]
pub enum DirectExpr<'a> {
    Int { value: i64 },
    Bool { value: bool },
    Float { value: f64 },
    Id { value: &'a str, id_type: Rc<Type> },
}

pub type Type = lang_common::Type;
pub type UnaryExprKind = lang_common::UnaryExprKind;
pub type BinaryExprKind = lang_common::BinaryExprKind;
