// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! The compiler is one of the final steps of the compilation process, which converts the intermediate representation
//! into assembly instructions (see `asm.rs`).
//!
//! It works by traveling down the IR and works with the register allocator. There are a variety of terms that are
//! used throughout this step:
//!   - symbol table: maps variables to where they are stored at runtime (see `symbol_table.rs`)
//!   - stack index: indicates the relative offset to the `RSP` register of where to put the next object (word).
//!                  Or equivalently, `stack_index + 8` represents where the bottom of the stack is.
//! These objects are kept and tracked while traveling through the IR.

use asm::asm::{Instruction, Instruction::*, Operand, Operand::*, Register, Register::*};
use compiler::compile_binary_expr::compile_binary_expr;
use compiler::compile_unary_expr::compile_unary_expr;
use compiler::symbol_table::{Location, SymbolTable};
use error_messages::internal_compiler_error;
use ir::ir::{Block, DirectExpr, Expr, Program};
use register_allocation::register_allocator::allocate_registers;
use register_allocation::register_allocator::{Assignment, Map, Set};

/// Compiles a Program into assembly instructions.
pub fn compile(program: Program) -> Vec<Instruction> {
    let mut instructions = vec![
        Global("entry".to_string()),
        Section("text".to_string()),
        Label("entry".to_string()),
    ];

    let mut symbol_table = SymbolTable::new();
    let mut stack_index = Box::new(-8);

    compile_block(&program.body, &mut symbol_table, &mut stack_index, &mut instructions);

    instructions.push(Ret);
    instructions
}

// Compiles a Block into assembly instructions, pushing the results into `instructions`.
fn compile_block(
    block: &Block,
    symbol_table: &mut SymbolTable,
    stack_index: &mut Box<i64>,
    instructions: &mut Vec<Instruction>,
) {
    // Run the register allocator
    let variable_assignment: Map<&String, Assignment> =
        allocate_registers(block, Set::from([&R8, &R9, &R10, &R11, &R12, &R13]));

    // Compile each expression
    for (i, expr) in block.exprs.iter().enumerate() {
        compile_expr(
            expr,
            // For the last expression only, compile the result into Rax (for implicit returns).
            if i == block.exprs.len() - 1 { Some(&Location::Register(Rax)) } else { None },
            symbol_table,
            stack_index,
            &variable_assignment,
            instructions,
        );
    }
}

/// Compiles an expression into assembly instructions, pushing the results into `instructions`
/// * expr - input expression
/// * location - where to put the result of the expression. If None, the result is not needed in the future.
/// * `variable_assignment` - the result from the register allocator for this block.
pub fn compile_expr(
    expr: &Expr,
    location: Option<&Location>,
    symbol_table: &mut SymbolTable,
    stack_index: &mut Box<i64>,
    variable_assignment: &Map<&String, Assignment>,
    instructions: &mut Vec<Instruction>,
) {
    match expr {
        Expr::Direct { expr } => {
            // If location is None, we can safely ignore the Direct as the result is not needed. This happens for
            // for example when a identifier appears in the top level of the block, like `let a: int = 0; a; ...`
            if let Some(location) = location {
                mov_instruction_safe(
                    location.to_operand(),
                    compile_direct(expr, symbol_table),
                    instructions,
                    R14,
                );
            }
        }
        Expr::Let { id, init_expr } => {
            // Convert assignment of let binding to a location
            let assignment_location = match variable_assignment.get(id).unwrap() {
                Assignment::Register(register) => Location::Register(*register),
                Assignment::Spill => {
                    let location = Location::StackIndex(**stack_index);
                    **stack_index -= 8;
                    location
                }
            };

            compile_expr(
                init_expr,
                Some(&assignment_location),
                symbol_table,
                stack_index,
                variable_assignment,
                instructions,
            );

            // Move the result of the init_expr to location if it is set.
            if let Some(location) = location {
                mov_instruction_safe(
                    location.to_operand(),
                    assignment_location.to_operand(),
                    instructions,
                    R14,
                );
            }

            // Add the identifier to the symbol_table, *after* compiling the init_expr.
            symbol_table.insert(id.to_string(), assignment_location);
        }
        Expr::BinaryExpr { kind, operand_1, operand_2 } => {
            // If location is None, we can safely ignore the BinaryExpr as well since it *cannot induce any side
            if let Some(location) = location {
                compile_binary_expr(kind, operand_1, operand_2, location, symbol_table, instructions);
            }
        }
        Expr::UnaryExpr { kind, operand } => {
            // If location is None, we can safely ignore the BinaryExpr as well since it *cannot induce any side
            if let Some(location) = location {
                compile_unary_expr(kind, operand, location, symbol_table, instructions);
            }
        }
    }
}

/// Compiles a `DirectExpr` into a assembly Operand.
pub fn compile_direct(direct: &DirectExpr, symbol_table: &mut SymbolTable) -> Operand {
    match direct {
        DirectExpr::Int { value } => Imm(*value),
        DirectExpr::Id { value } => symbol_table
            .get(value)
            .unwrap_or_else(|| internal_compiler_error(&format!("symbol `{value}` not in symbol_table")))
            .to_operand(),
        DirectExpr::Bool { value } => Imm(i64::from(*value)),
        DirectExpr::Float { .. } => todo!(),
    }
}

/// Converts a stack index into a assembly operand.
pub fn stack_address(stack_index: i64) -> Operand {
    MemOffset(Box::new(Reg(Rsp)), Box::new(Imm(stack_index)))
}

/// Same as adding a `Mov(asm_operand_1, asm_operand_2)`, but ensures that both operands are not `MemOffset`.
/// If both are, the second operand is moved to the `backup_temporary_register`.
pub fn mov_instruction_safe(
    asm_operand_1: Operand,
    asm_operand_2: Operand,
    instructions: &mut Vec<Instruction>,
    backup_temporary_register: Register,
) {
    if matches!(asm_operand_1, MemOffset(..)) && matches!(asm_operand_2, MemOffset(..)) {
        instructions.push(Mov(Reg(backup_temporary_register), asm_operand_2));
        instructions.push(Mov(asm_operand_1, Reg(backup_temporary_register)));
    } else {
        instructions.push(Mov(asm_operand_1, asm_operand_2));
    }
}
