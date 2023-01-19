// Copyright © 2022 Brandon Li. All rights reserved.

//! Starting point for the code generation phase of the compiler. TODO: backend x phase, impl notes, type checking, etc.

use asm::asm::Operand;
use asm::asm::Operand::Imm;
use asm::asm::Operand::MemOffset;
use asm::asm::Operand::Reg;
use asm::asm::Register::Rax;
use asm::asm::Register::Rsp;
use asm::asm::{Instruction, Instruction::*};
use compiler::compile_binary_expr::compile_binary_expr;
use parser::ast::Expr;
use parser::ast::Program;
use std::collections::HashMap;

pub fn compile(program: Program) -> Vec<Instruction> {
    let mut instructions = vec![Global("entry".to_string()), Label("entry".to_string())];
    compile_expr(&program.body, &mut HashMap::new(), &mut Box::new(-8), &mut instructions);
    instructions.push(Ret);
    instructions
}

pub fn compile_expr(
    expr: &Expr,
    symbol_table: &mut HashMap<String, i64>,
    stack_index: &mut Box<i64>,
    instructions: &mut Vec<Instruction>,
) {
    match expr {
        Expr::Do { exprs } => {
            for expr in exprs {
                compile_expr(expr, symbol_table, stack_index, instructions);
            }
        }
        Expr::Int { value } => instructions.push(Mov(Reg(Rax), Imm(*value))),
        Expr::Id { value } => {
            instructions.push(Mov(Reg(Rax), stack_address(*symbol_table.get(value).unwrap())));
        }
        Expr::Let { id, type_reference: _, init_expr } => {
            compile_expr(init_expr, symbol_table, stack_index, instructions);
            instructions.push(Mov(stack_address(**stack_index), Reg(Rax)));

            symbol_table.insert(id.to_string(), **stack_index);
            **stack_index -= 8;
        }
        Expr::BinaryExpr { kind, operand_1, operand_2 } => {
            compile_binary_expr(kind, operand_1, operand_2, symbol_table, stack_index, instructions);
        }
        Expr::UnaryExpr { .. } => {
            println!("{expr:#?}");
            todo!()
        }
    }
}

pub fn stack_address(stack_index: i64) -> Operand {
    MemOffset(Box::new(Reg(Rsp)), Box::new(Imm(stack_index)))
}
