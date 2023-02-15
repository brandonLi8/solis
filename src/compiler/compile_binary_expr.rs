// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! This file compiles binary expressions into assembly instructions. See `compiler.rs` for background on the compiler
//! step. Binary expressions have only directs as operands because of the translator. We attempt to minimize
//! the number of instructions for each operation as much as possible.

use asm::asm::{Instruction, Instruction::*, Operand::*, Register::*};
use compiler::compiler::compile_direct;
use compiler::symbol_table::{Location, SymbolTable};
use error_messages::internal_compiler_error;
use ir::ir::{BinaryExprKind, DirectExpr};

/// Compiles a binary expression into assembly instructions, pushing the results into `instructions`
/// * `kind` - the type of binary expression
/// * `operand_1` - the first operand of the binary expression
/// * `operand_2` - the first operand of the binary expression
/// * `location` - where to put the result of the expression.
///
/// NOTE: this function must leave the state of the operands and location correct, or the output may be incorrect.
/// While maintaining it, watch out for:
///   - Do not modify the operand locations. The operands must remain the same before and after.
///
///   - DO NOT modify `location` within the operation. Flow is "do operation, then put result to location atomically".
///     This is because `operand_1` or `operand_2` or `location` could be the *same*, so if the `location` was modified
///     during the expression, you might modify one of the operands before you use it.
pub fn compile_binary_expr(
    kind: &BinaryExprKind,
    operand_1: &DirectExpr,
    operand_2: &DirectExpr,
    location: &Location,
    symbol_table: &mut SymbolTable,
    instructions: &mut Vec<Instruction>,
) {
    // Convert the operands to assembly operands
    let mut asm_operand_1 = compile_direct(operand_1, symbol_table);
    let mut asm_operand_2 = compile_direct(operand_2, symbol_table);

    instructions.push(Comment(format!("{kind:?}, {operand_1:?}, {operand_2:?}"))); // TODO: option?

    match kind {
        BinaryExprKind::Plus | BinaryExprKind::Minus => {
            // Move the first operand to a temporary register, and then operate on the temporary register.
            // We do this to avoid modifying the location before using operand_2.
            instructions.push(Mov(Reg(R14), asm_operand_1));
            instructions.push(match kind {
                BinaryExprKind::Plus => Add(Reg(R14), asm_operand_2),
                BinaryExprKind::Minus => Sub(Reg(R14), asm_operand_2),
                _ => internal_compiler_error("unreachable"),
            });

            instructions.push(Mov(location.to_operand(), Reg(R14)));
        }

        BinaryExprKind::Times => {
            // If the output location is a register, we can potentially take advantage of the `imul <reg>,<reg>,<imm>`
            // format of `imul`. Only one of the operands can be immediate.
            if matches!(&location.to_operand(), Reg(..))
                && (matches!(asm_operand_1, Imm(..)) ^ matches!(asm_operand_2, Imm(..)))
            {
                // Ensure second operand is the immediate, and swapping doesn't change the result of the operation
                if let Imm(..) = asm_operand_1 {
                    (asm_operand_1, asm_operand_2) = (asm_operand_2, asm_operand_1);
                }

                instructions.push(Mul3(location.to_operand(), asm_operand_1, asm_operand_2));
            } else {
                instructions.push(Mov(Reg(R14), asm_operand_1));
                instructions.push(Mul(Reg(R14), asm_operand_2));
                instructions.push(Mov(location.to_operand(), Reg(R14)));
            }
        }

        BinaryExprKind::Divide | BinaryExprKind::Mod => {
            // Division operates on the Rax register, so we move the first operand to Rax.
            // TODO: Do we need to save rax? Potentially not. because in the scenario that you set rax to something, and then call compile_binary_expr, shouldn't happen, it should just return if rax is meaningful.
            // What about Rdx. Probably worth it to make rdx a general purpose register, and just save it and restore for the purpose of this expression.
            instructions.push(Mov(Reg(Rax), asm_operand_1));

            // Sign extend rax into rdx
            instructions.push(Cqo);

            // The operand for division must not be a immediate. If asm_operand_2
            if let Imm(_) = asm_operand_2 {
                instructions.push(Mov(Reg(R14), asm_operand_2));
                asm_operand_2 = Reg(R14);
            }

            instructions.push(Div(asm_operand_2));

            instructions.push(Mov(
                location.to_operand(),
                Reg(if let BinaryExprKind::Divide = kind { Rax } else { Rdx }),
            ));
        }

        BinaryExprKind::LessThan
        | BinaryExprKind::MoreThan
        | BinaryExprKind::LessThanOrEquals
        | BinaryExprKind::MoreThanOrEquals => {
            // Doing `MoreThan` is the same as doing `LessThan` with swapped arguments.
            if let BinaryExprKind::MoreThan | BinaryExprKind::MoreThanOrEquals = kind {
                (asm_operand_1, asm_operand_2) = (asm_operand_2, asm_operand_1);
            }

            // The first operand of the `Cmp` instruction must be a Reg/MemOffset. Additionally, both operands cannot
            // be MemOffset. If either of these combinations are true, we can create a valid `Cmp` with a temp register.
            if matches!(asm_operand_1, Imm(..))
                || (matches!(asm_operand_1, MemOffset(..)) && matches!(asm_operand_2, MemOffset(..)))
            {
                instructions.push(Mov(Reg(R14), asm_operand_1));
                asm_operand_1 = Reg(R14);
            }

            instructions.push(Cmp(asm_operand_1, asm_operand_2));

            // Zero out location
            instructions.push(Mov(location.to_operand(), Imm(0)));

            if let BinaryExprKind::LessThan | BinaryExprKind::MoreThan = kind {
                instructions.push(Setl(location.to_operand()));
            } else {
                instructions.push(Setle(location.to_operand()));
            }
        }

        BinaryExprKind::EqualsEquals | BinaryExprKind::NotEquals => {
            // The first operand of the `Cmp` instruction must be a Reg/MemOffset. Additionally, both operands cannot
            // be MemOffset. If either of these combinations are true, we can create a valid `Cmp` with a temp register.
            if matches!(asm_operand_1, Imm(..))
                || (matches!(asm_operand_1, MemOffset(..)) && matches!(asm_operand_2, MemOffset(..)))
            {
                // If the first operand is a Imm and the second isn't, we can swap the operands, which doesn't change
                // the value of a equality expression.
                // TODO: do this for LT/GT? And use `setg` and `setge` if true.
                if matches!(asm_operand_1, Imm(..)) && !matches!(asm_operand_2, Imm(..)) {
                    (asm_operand_1, asm_operand_2) = (asm_operand_2, asm_operand_1);
                } else {
                    instructions.push(Mov(Reg(R14), asm_operand_1));
                    asm_operand_1 = Reg(R14);
                }
            }

            instructions.push(Cmp(asm_operand_1, asm_operand_2));

            // Zero out location
            instructions.push(Mov(location.to_operand(), Imm(0)));

            if let BinaryExprKind::EqualsEquals = kind {
                instructions.push(Setz(location.to_operand()));
            } else {
                instructions.push(Setnz(location.to_operand()));
            }
        }
    }
}
