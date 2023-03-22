// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Transforms the in memory representation of assembly into an actual assembly file, using a buffered writer.

use asm::asm::{
    FloatRegister, FloatRegister::*, Instruction, Instruction::*, Operand, Operand::*, Register, Register::*,
};
use error_messages::internal_compiler_error;
use std::fs::{create_dir_all, File};
use std::io::{BufWriter, Write};
use std::path::Path;

/// Writes a vector of instructions to a file, using a buffered writer.
pub fn write_instructions_to_file(instructions: Vec<Instruction>, file_path: &Path) {
    create_dir_all(file_path.parent().unwrap()).unwrap();

    let file = File::create(file_path)
        .unwrap_or_else(|error| internal_compiler_error(&format!("unable to open file {error}")));
    let mut buf_writer = BufWriter::new(file);

    for instruction in instructions {
        buf_writer
            .write_all(format!("{}\n", instruction_to_string(instruction)).as_bytes())
            .unwrap_or_else(|error| internal_compiler_error(&format!("unable to write to file {error}")));
    }
}

// Converts a Register to a string
fn register_to_string(register: Register) -> String {
    match register {
        Rax => "rax",
        Rsi => "rsi",
        Rdi => "rdi",
        Rsp => "rsp",
        Rbp => "rbp",
        Rbx => "rbx",
        Rcx => "rcx",
        Rdx => "rdx",
        R8 => "r8",
        R9 => "r9",
        R10 => "r10",
        R11 => "r11",
        R12 => "r12",
        R13 => "r13",
        R14 => "r14",
        R15 => "r15",
    }
    .to_string()
}

// Converts a Register to a string, where it must be corresponding to a BYTE
fn byte_register_to_string(register: Register) -> String {
    match register {
        Rax => "al",
        Rsi => "sil",
        Rdi => "dil",
        Rsp => "spl",
        Rbp => "bpl",
        Rbx => "bl",
        Rcx => "cl",
        Rdx => "dl",
        R8 => "r8b",
        R9 => "r9b",
        R10 => "r10b",
        R11 => "r11b",
        R12 => "r12b",
        R13 => "r13b",
        R14 => "r14b",
        R15 => "r15b",
    }
    .to_string()
}

// Converts a Float Register to a string
fn float_register_to_string(register: FloatRegister) -> String {
    match register {
        Xmm0 => "xmm0",
        Xmm1 => "xmm1",
        Xmm2 => "xmm2",
        Xmm3 => "xmm3",
        Xmm4 => "xmm4",
        Xmm5 => "xmm5",
        Xmm6 => "xmm6",
        Xmm7 => "xmm7",
        Xmm8 => "xmm8",
        Xmm9 => "xmm9",
        Xmm10 => "xmm10",
        Xmm11 => "xmm11",
        Xmm12 => "xmm12",
        Xmm13 => "xmm13",
        Xmm14 => "xmm14",
        Xmm15 => "xmm15",
    }
    .to_string()
}

// Converts a Operand to a string
fn operand_to_string(operand: Operand) -> String {
    match operand {
        Reg(reg) => register_to_string(reg),
        Imm(imm) => imm.to_string(),
        MemOffset(operand_1, operand_2) => format!(
            "QWORD [{} + {}]",
            &operand_to_string(*operand_1),
            &operand_to_string(*operand_2)
        ),
        FloatImm(imm) => format!("__?float64?__({imm:#?})"),
        FloatReg(reg) => float_register_to_string(reg),
    }
}

// Converts a Operand to a string, where it must be corresponding to a BYTE
fn byte_operand_to_string(operand: Operand) -> String {
    match operand {
        MemOffset(operand_1, operand_2) => format!(
            "BYTE [{} + {}]",
            &operand_to_string(*operand_1),
            &operand_to_string(*operand_2)
        ),
        Reg(reg) => byte_register_to_string(reg),
        Imm(..) | FloatImm(..) => operand_to_string(operand),
        FloatReg(..) => internal_compiler_error("float register as byte operand"),
    }
}

// Decorates labels on mac with `_`
fn label_name(label: String) -> String {
    if cfg!(target_os = "macos") && !cfg!(test) {
        format!("_{label}")
    } else {
        label
    }
}

// Converts a Instruction to a string
fn instruction_to_string(instruction: Instruction) -> String {
    #[rustfmt::skip]
    let instruction = match instruction {
        Global(label) =>          format!("global {}", label_name(label)),
        Extern(label) =>          format!("extern {}", label_name(label)),
        Section(label) =>         format!("\tsection .{label}"),
        Label(label) =>           format!("{}:", label_name(label)),
        DqLabel(label) =>         format!("\tdq {}", label_name(label)),
        DqString(label) =>        format!("\tdq `{label}`, 0"),
        DqInt(src) =>             format!("\tdq {src}"),
        Align(src) =>             format!("align {src}"),
        Mov(dest, src) =>         format!("\tmov {}, {}", operand_to_string(dest), operand_to_string(src)),
        MovByte(dest, src) =>     format!("\tmov {}, {}", byte_operand_to_string(dest), byte_operand_to_string(src)),
        Add(dest, src) =>         format!("\tadd {}, {}", operand_to_string(dest), operand_to_string(src)),
        Sub(dest, src) =>         format!("\tsub {}, {}", operand_to_string(dest), operand_to_string(src)),
        Div(src) =>               format!("\tidiv {}", operand_to_string(src)),
        Mul(dest, src) =>         format!("\timul {}, {}", operand_to_string(dest), operand_to_string(src)),
        Mul3(dest, src, con) =>   format!("\timul {}, {}, {}", operand_to_string(dest), operand_to_string(src), operand_to_string(con)),
        Cqo =>                            "\tcqo".to_string(),
        Neg(operand) =>           format!("\tneg {}", operand_to_string(operand)),
        Shl(dest, src) =>         format!("\tshl {}, {}", operand_to_string(dest), operand_to_string(src)),
        Shr(dest, src) =>         format!("\tshr {}, {}", operand_to_string(dest), operand_to_string(src)),
        Sar(dest, src) =>         format!("\tsar {}, {}", operand_to_string(dest), operand_to_string(src)),
        Cmp(dest, src) =>         format!("\tcmp {}, {}", operand_to_string(dest), operand_to_string(src)),
        And(dest, src) =>         format!("\tand {}, {}", operand_to_string(dest), operand_to_string(src)),
        Or(dest, src) =>          format!("\tor {}, {}", operand_to_string(dest), operand_to_string(src)),
        Setz(dest) =>             format!("\tsetz {}", byte_operand_to_string(dest)),
        Setnz(dest) =>            format!("\tsetnz {}", byte_operand_to_string(dest)),
        Setl(dest) =>             format!("\tsetl {}", byte_operand_to_string(dest)),
        Setle(dest) =>            format!("\tsetle {}", byte_operand_to_string(dest)),
        LeaLabel(dest, label) =>  format!("\tlea {}, [{}]", operand_to_string(dest), label_name(label)),
        Jmp(dest) =>              format!("\tjmp {}", label_name(dest)),
        Je(dest) =>               format!("\tje {}", label_name(dest)),
        Jne(dest) =>              format!("\tjne {}", label_name(dest)),
        Jl(dest) =>               format!("\tjl {}", label_name(dest)),
        Jnl(dest) =>              format!("\tjnl {}", label_name(dest)),
        Jg(dest) =>               format!("\tjg {}", label_name(dest)),
        Jng(dest) =>              format!("\tjng {}", label_name(dest)),
        ComputedJmp(dest) =>      format!("\tjmp {}", operand_to_string(dest)),
        Push(operand) =>          format!("\tpush {}", operand_to_string(operand)),
        Pop(operand) =>           format!("\tpop {}", operand_to_string(operand)),
        Call(dest) =>             format!("\tcall {}", label_name(dest)),
        Ret =>                            "\tret".to_string(),

        Movq(dest, src) =>        format!("\tmovq {}, {}", operand_to_string(dest), operand_to_string(src)),
        Cvttsd2si(dest, src) =>   format!("\tcvttsd2si {}, {}", operand_to_string(dest), operand_to_string(src)),
        Cvtsi2sd(dest, src) =>    format!("\tcvtsi2sd {}, {}", operand_to_string(dest), operand_to_string(src)),
        Xorpd(dest, src) =>       format!("\txorpd {}, {}", operand_to_string(dest), operand_to_string(src)),
        Addsd(dest, src) =>       format!("\taddsd {}, {}", operand_to_string(dest), operand_to_string(src)),
        Subsd(dest, src) =>       format!("\tsubsd {}, {}", operand_to_string(dest), operand_to_string(src)),
        Mulsd(dest, src) =>       format!("\tmulsd {}, {}", operand_to_string(dest), operand_to_string(src)),
        Divsd(dest, src) =>       format!("\tdivsd {}, {}", operand_to_string(dest), operand_to_string(src)),
        Cmpsd(dest, src, mode) => format!("\tcmpsd {}, {}, {mode}", operand_to_string(dest), operand_to_string(src)),

        Comment(comment) =>       format!("\n; {comment}"),
        Annotate(instruction, comment) => {
            format!("{: <40} ; {comment}", instruction_to_string(*instruction))
        }
    };
    instruction
}
