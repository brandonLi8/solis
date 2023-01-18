// Copyright © 2022 Brandon Li. All rights reserved.

//! TODO

use asm::asm::Operand::Imm;
use asm::asm::Operand::Reg;
use asm::asm::Register::Rax;
use asm::asm::{Instruction, Instruction::*};
use parser::ast::Expr;
use parser::ast::Program;

pub fn compile(program: Program) -> Vec<Instruction> {
    let mut a = vec![Global("entry".to_string()), Label("entry".to_string())];
    a.extend(compile_expr(&program.body));
    a.push(Ret);
    a
}

fn compile_expr(expr: &Expr) -> Vec<Instruction> {
    match expr {
        Expr::Do { exprs } => compile_expr(&exprs[0]),
        Expr::Int { value } => {
            vec![Mov(Reg(Rax), Imm(*value))]
        }
        _ => todo!(),
    }
}
