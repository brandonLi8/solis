// Copyright © 2022 Brandon Li. All rights reserved.

//! TODO

use asm::asm::Operand::*;
use asm::asm::Register::*;
use asm::asm::{Instruction, Instruction::*};
use compiler::compiler::compile_direct;
use ir::ir::{BinaryExprKind, DirectExpr};
use std::collections::HashMap;

pub fn compile_binary_expr(
    kind: &BinaryExprKind,
    operand_1: &DirectExpr,
    operand_2: &DirectExpr,
    symbol_table: &mut HashMap<String, i64>,
    stack_index: &mut Box<i64>,
    instructions: &mut Vec<Instruction>,
) {
    compile_direct(operand_2, symbol_table, stack_index, instructions);

    instructions.push(Mov(Reg(R8), Reg(Rax)));

    compile_direct(operand_1, symbol_table, stack_index, instructions);

    match kind {
        BinaryExprKind::Plus => instructions.push(Add(Reg(Rax), Reg(R8))),
        BinaryExprKind::Minus => instructions.push(Sub(Reg(Rax), Reg(R8))),
        _ => todo!(),
    }
}
