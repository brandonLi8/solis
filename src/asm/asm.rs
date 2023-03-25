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

/// Operands for instructions.
#[derive(Debug, Clone)]
pub enum Operand {
    Reg(Register),
    FloatReg(FloatRegister),
    Imm(i64),
    FloatImm(f64),
    MemOffset(Box<Operand>, Box<Operand>),
}

/// Every instruction, annotated with allowed operand combinations.
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

    Mov(Operand, Operand), // move                      (reg|mem, reg|mem|imm) but not (mem, mem)
    MovByte(Operand, Operand), // move LSB              (reg|mem, reg|mem|imm) but not (mem, mem)
    Add(Operand, Operand), // +=                        (reg|mem, reg|mem|imm) but not (mem, mem)
    Sub(Operand, Operand), // -=                        (reg|mem, reg|mem|imm) but not (mem, mem)
    Div(Operand),          // / (src = rdx:rax) (IDIV)  (reg|mem)
    Mul(Operand, Operand), // * (src = rdx:rax) (IMUL)  (reg, reg|mem|imm)
    Mul3(Operand, Operand, Operand), // a = b * c       (reg, reg|mem, imm)
    Neg(Operand),          // negation                  (mem|reg)
    Cqo,                   // sign extend rax into rdx

    Shl(Operand, Operand),
    Shr(Operand, Operand),
    Sar(Operand, Operand),
    Cmp(Operand, Operand), // compare - (reg|mem, reg|mem|imm) but not (mem, mem)
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

    Movq(Operand, Operand),      // move for float operands      (xmm, reg|mem) | (reg|mem, xmm)
    Cvttsd2si(Operand, Operand), // convert scalar to Signed Int (reg, xmm)
    Cvtsi2sd(Operand, Operand),  // convert Signed Int to scalar (xmm, r/mem)
    Xorpd(Operand, Operand),     // xor for FloatRegisters       (xmm, xmm128/mem128)
    Addsd(Operand, Operand),     // add scalar floats            (xmm, xmm/mem)
    Subsd(Operand, Operand),     // subtract scalar floats       (xmm, xmm/mem)
    Mulsd(Operand, Operand),     // multiply scalar floats       (xmm, xmm/mem)
    Divsd(Operand, Operand),     // divide scalar floats         (xmm, xmm/mem)
    Cmpsd(Operand, Operand, u8), // compare scalar floats        (xmm, xmm) see https://c9x.me/x86/html/file_module_x86_id_39.html

    Comment(String),                    // Top level comment
    Annotate(Box<Instruction>, String), // Annotated Instruction with comment
}

impl Instruction {
    /// Creates a `Annotate` variant of an Instruction
    #[must_use]
    pub fn annotated(self, comment: &str) -> Self {
        Self::Annotate(Box::new(self), comment.to_string())
    }
}
