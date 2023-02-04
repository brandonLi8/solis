// Copyright © 2022 Brandon Li. All rights reserved.

//! This file contains the definitions for an in memory representation of any assembly file. We work with this
//! representation in the final code generation stage. Working with this representation will be much easier to work with
//! compared to a string output.

/// Every register that we can use. Registers with special purposes have been annotated.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum Register {
    Rax, // Return value
    Rsi,
    Rdi,
    Rsp, // Stack Pointer
    Rbp,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14, // Scratch 1
    R15, // Scratch 2
}

/// Operands for instructions.
pub enum Operand {
    Reg(Register),
    Imm(i64),
    MemOffset(Box<Operand>, Box<Operand>),
}

/// Every instruction.
pub enum Instruction {
    Global(String),
    Extern(String),
    Label(String),
    Mov(Operand, Operand),
    Add(Operand, Operand),
    Sub(Operand, Operand),
    Cmp(Operand, Operand),
    And(Operand, Operand),
    Or(Operand, Operand),
    Jmp(String),
    Je(String),
    Jne(String),
    Jl(String),
    Jnl(String),
    Jg(String),
    Jng(String),
    Ret,
    Push(Operand),
    Pop(Operand),
    Call(String),
    Comment(String),
}
