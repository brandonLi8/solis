// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! TODO

use asm::asm::Operand::*;
use asm::asm::Register::*;
use asm::asm::{Instruction, Instruction::*};
use compiler::compiler::compile_direct;
use compiler::compiler::Location;
use ir::ir::{BinaryExprKind, DirectExpr};
use register_allocation::register_allocator::Assignment;
use register_allocation::register_allocator::Map;
use std::collections::HashMap;

pub fn compile_binary_expr(
    kind: &BinaryExprKind,
    operand_1: &DirectExpr,
    operand_2: &DirectExpr,
    symbol_table: &mut HashMap<String, Location>,
    _stack_index: &mut Box<i64>,
    _variable_assignment: &Map<&String, Assignment>,
    instructions: &mut Vec<Instruction>,
) -> Location {
    let asm_operand_1 = compile_direct(operand_1, symbol_table);

    instructions.push(Mov(Reg(Rax), asm_operand_1));

    let asm_operand_2 = compile_direct(operand_2, symbol_table);

    match kind {
        BinaryExprKind::Plus => instructions.push(Add(Reg(Rax), asm_operand_2)),
        BinaryExprKind::Minus => instructions.push(Sub(Reg(Rax), asm_operand_2)),
        BinaryExprKind::Times => instructions.push(Mul(Reg(Rax), asm_operand_2)),
        _ => todo!(),
    }
    Location::Register(Rax)
}
