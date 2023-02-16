// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! This file compiles unary expressions into assembly instructions. See `compiler.rs` for background on the compiler
//! step. Unary expressions have only directs as operands because of the translator. We attempt to minimize
//! the number of instructions for each operation as much as possible.

use asm::asm::{Instruction, Instruction::*, Operand::*, Register::*};
use compiler::compiler::{compile_direct, mov_instruction_safe};
use compiler::symbol_table::{Location, SymbolTable};
use ir::ir::{DirectExpr, UnaryExprKind};

/// Compiles a unary expression into assembly instructions, pushing the results into `instructions`
/// * kind - the type of unary expression
/// * operand - the operand of the unary expression
/// * location - where to put the result of the expression.
///
/// NOTE: this function must leave the state of the operand and location correct, or the output may be incorrect.
/// While maintaining it, watch out for:
///   - Do not modify the operand location. The operand must remain the same before and after.
///
///   - DO NOT modify `location` before using `operand`. Flow is "do operation, then put result to location atomically".
///     This is because `operand` and `location` could be the *same*, so if the `location` was modified
///     during the expression, you might modify the operand before you use it.
pub fn compile_unary_expr(
    kind: &UnaryExprKind,
    operand: &DirectExpr,
    location: &Location,
    symbol_table: &mut SymbolTable,
    instructions: &mut Vec<Instruction>,
) {
    // Convert the operand to a assembly operand
    let mut asm_operand = compile_direct(operand, symbol_table);

    instructions.push(Comment(format!("{kind:?}, {operand:?}"))); // TODO: option?

    match kind {
        UnaryExprKind::Not => {
            // The first operand of the `Cmp` instruction must be a Reg/MemOffset
            if matches!(asm_operand, Imm(..)) {
                instructions.push(Mov(Reg(R14), asm_operand));
                asm_operand = Reg(R14);
            }

            instructions.push(Cmp(asm_operand, Imm(0)));

            // Zero out location
            instructions.push(Mov(location.to_operand(), Imm(0)));

            instructions.push(Setz(location.to_operand()));
        }
        UnaryExprKind::Negative => {
            // No temporary registers are needed since asm_operand is not modified.
            mov_instruction_safe(location.to_operand(), asm_operand, instructions, R14);
            instructions.push(Neg(location.to_operand()));
        }
    }
}
