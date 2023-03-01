// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! This file contains the definitions for an in memory representation of any assembly file. We work with this
//! representation in the final code generation stage. Working with this representation will be much easier to work with
//! compared to a string output.

/// Every register that we can use. Registers with special purposes have been annotated.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum Register {
    Rax, // Return value
    Rbx,
    Rcx,
    Rdx, // Sign extend
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
#[derive(Clone)]
pub enum Operand {
    Reg(Register),
    Imm(i64),
    MemOffset(Box<Operand>, Box<Operand>),
    FloatImm(f64),
    FloatReg(FloatRegister),
}

/// Registers for floating point (SSE). Registers with special purposes have been annotated.
/// See `http://csapp.cs.cmu.edu/public/waside/waside-sse.pdf`
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum FloatRegister {
    Xmm0, // Return value
    Xmm1,
    Xmm2,
    Xmm3,
    Xmm4,
    Xmm5,
    Xmm6,
    Xmm7,
    Xmm8,
    Xmm9,
    Xmm10,
    Xmm11,
    Xmm12,
    Xmm13,
    Xmm14, // Scratch 1
    Xmm15, // Scratch 2
}

/// Every instruction.
pub enum Instruction {
    Global(String),
    Extern(String),
    Section(String),
    Label(String),
    DqLabel(String),
    DqString(String),
    DqInt(i64),
    Align(i64),
    LeaLabel(Operand, String),
    Mov(Operand, Operand),
    MovByte(Operand, Operand),
    Add(Operand, Operand),
    Sub(Operand, Operand),
    Div(Operand),
    Mul(Operand, Operand),
    Mul3(Operand, Operand, Operand),
    Cqo,
    Neg(Operand),
    Shl(Operand, Operand),
    Shr(Operand, Operand),
    Sar(Operand, Operand),
    Cmp(Operand, Operand),
    And(Operand, Operand),
    Or(Operand, Operand),
    Setz(Operand),
    Setnz(Operand),
    Setl(Operand),
    Setle(Operand),
    Jmp(String),
    Je(String),
    Jne(String),
    Jl(String),
    Jnl(String),
    Jg(String),
    Jng(String),
    ComputedJmp(Operand),
    Ret,
    Push(Operand),
    Pop(Operand),
    Call(String),

    Movq(Operand, Operand),      // Move for float operands
    Cvttsd2si(Operand, Operand), // Convert scalar to Signed Int (r64, xmm)
    Cvtsi2sd(Operand, Operand),  // Convert Signed Int to scalar (xmm, r/m64)
    Xorpd(Operand, Operand),     // xor for FloatRegisters
    Addsd(Operand, Operand),     // Add scalar floats            (xmm1, xmm2/m64)
    Subsd(Operand, Operand),     // Subtract scalar floats       (xmm1, xmm2/m64)
    Mulsd(Operand, Operand),     // Multiply scalar floats       (xmm1, xmm2/m64)
    Divsd(Operand, Operand),     // Divide scalar floats         (xmm1, xmm2/m64)
    Cmpsd(Operand, Operand, u8), // Compare scalar floats        (xmm1, xmm2) see https://c9x.me/x86/html/file_module_x86_id_39.html

    Comment(String),                    // Top level comment
    Annotate(Box<Instruction>, String), // Annotated Instruction with comment
}
