// Copyright © 2022 Brandon Li. All rights reserved.

//! Starting point for the code generation phase of the compiler. TODO: backend x phase, impl notes, type checking, etc.

use asm::asm::Operand;
use asm::asm::Operand::Imm;
use asm::asm::Operand::MemOffset;
use asm::asm::Operand::Reg;
use asm::asm::Register::{self, *};
use asm::asm::{Instruction, Instruction::*};
use compiler::compile_binary_expr::compile_binary_expr;
use register_allocation::register_allocator::allocate_registers;
use register_allocation::register_allocator::Assignment;
use register_allocation::register_allocator::{Map, Set};

use ir::ir::Program;
use ir::ir::{Block, DirectExpr, Expr};
use std::collections::HashMap;

pub fn compile(program: Program) -> Vec<Instruction> {
    let mut instructions = vec![Global("entry".to_string()), Label("entry".to_string())];
    compile_block(&program.body, &mut HashMap::new(), &mut Box::new(-8), &mut instructions);
    instructions.push(Ret);
    instructions
}

fn compile_block(
    block: &Block,
    symbol_table: &mut HashMap<String, Location>,
    stack_index: &mut Box<i64>,
    instructions: &mut Vec<Instruction>,
) {
    let variable_assignment: Map<&String, Assignment> = allocate_registers(
        block,
        Set::from([
            // &Rsi,
            // &Rdi,
            // &Rbp,
            &R8, &R9, &R10, &R11, &R12, &R13,
        ]),
    );
    // println!("{block:#?} {variable_assignment:#?}");
    for expr in &block.exprs {
        compile_expr(expr, symbol_table, stack_index, &variable_assignment, instructions);
    }
}

pub enum Location {
    Register(Register),
    Stack(i64),
}

pub fn compile_direct(direct: &DirectExpr, symbol_table: &mut HashMap<String, Location>) -> Operand {
    match direct {
        DirectExpr::Int { value } => Imm(*value),
        DirectExpr::Id { value } => location_to_operand(symbol_table.get(value).unwrap()),
        DirectExpr::Bool { value } => Imm(i64::from(*value)),
    }
}

pub fn compile_expr(
    expr: &Expr,
    symbol_table: &mut HashMap<String, Location>,
    stack_index: &mut Box<i64>,
    variable_assignment: &Map<&String, Assignment>,
    instructions: &mut Vec<Instruction>,
) -> Location {
    match expr {
        Expr::Direct { expr } => {
            instructions.push(Mov(Reg(Rax), compile_direct(expr, symbol_table)));
            Location::Register(Rax)
        }
        Expr::Let { id, init_expr } => {
            let location = compile_expr(init_expr, symbol_table, stack_index, variable_assignment, instructions);

            match variable_assignment.get(id).unwrap() {
                Assignment::Register(register) => {
                    instructions.push(Mov(Reg(*register), location_to_operand(&location)));
                    symbol_table.insert(id.to_string(), Location::Register(*register));
                    Location::Register(*register)
                }
                Assignment::Spill => {
                    let res = stack_address(**stack_index);
                    instructions.push(Mov(res, location_to_operand(&location)));
                    symbol_table.insert(id.to_string(), Location::Stack(**stack_index));
                    **stack_index -= 8;
                    Location::Stack(**stack_index + 8)
                }
            }
        }
        Expr::BinaryExpr { kind, operand_1, operand_2 } => compile_binary_expr(
            kind,
            operand_1,
            operand_2,
            symbol_table,
            stack_index,
            variable_assignment,
            instructions,
        ),
        Expr::UnaryExpr { .. } => {
            println!("{expr:#?}");
            todo!()
        }
    }
}

pub fn location_to_operand(location: &Location) -> Operand {
    match location {
        Location::Register(register) => Reg(*register),
        Location::Stack(offset) => stack_address(*offset),
    }
}

pub fn stack_address(stack_index: i64) -> Operand {
    MemOffset(Box::new(Reg(Rsp)), Box::new(Imm(stack_index)))
}
