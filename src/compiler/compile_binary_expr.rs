// Copyright © 2022 Brandon Li. All rights reserved.

//! TODO

use asm::asm::Operand::*;
use asm::asm::Register::*;
use asm::asm::{Instruction, Instruction::*};
use compiler::compiler::{compile_expr, stack_address};
use parser::ast::{BinaryExprKind, Expr};
use std::collections::HashMap;

pub fn compile_binary_expr(
    kind: &BinaryExprKind,
    operand_1: &Expr,
    operand_2: &Expr,
    symbol_table: &mut HashMap<String, i64>,
    stack_index: &mut Box<i64>,
    instructions: &mut Vec<Instruction>,
) {
    compile_expr(operand_2, symbol_table, stack_index, instructions);

    instructions.push(Mov(stack_address(**stack_index), Reg(Rax)));
    **stack_index -= 8;

    compile_expr(operand_1, symbol_table, stack_index, instructions);

    **stack_index += 8;

    match kind {
        BinaryExprKind::Plus => instructions.push(Add(Reg(Rax), stack_address(**stack_index))),
        BinaryExprKind::Minus => instructions.push(Sub(Reg(Rax), stack_address(**stack_index))),
        _ => todo!(),
    }
}
