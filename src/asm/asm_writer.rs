// Copyright © 2022 Brandon Li. All rights reserved.

//! Transforms the in memory representation of assembly into an actual assembly file, using a buffered writer.

use asm::asm::{Instruction, Instruction::*, Operand, Operand::*, Register, Register::*};
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

// Converts a Operand to a string
fn operand_to_string(operand: Operand) -> String {
    match operand {
        Reg(reg) => register_to_string(reg),
        Imm(imm) => imm.to_string(),
        MemOffset(operand_1, operand_2) => format!(
            "QWORD [{}+ {}]",
            &operand_to_string(*operand_1),
            &operand_to_string(*operand_2)
        ),
    }
}

// Decorates labels on mac with `_`
fn label_name(label: String) -> String {
    if cfg!(macos) {
        format!("_{label}")
    } else {
        label
    }
}

// Converts a Instruction to a string
fn instruction_to_string(instruction: Instruction) -> String {
    match instruction {
        Global(label) => format!("global {}", label_name(label)),
        Extern(label) => format!("extern {}", label_name(label)),
        Label(label) => format!("{}:", label_name(label)),
        Mov(dest, src) => format!("\tmov {}, {}", operand_to_string(dest), operand_to_string(src)),
        Add(dest, src) => format!("\tadd {}, {}", operand_to_string(dest), operand_to_string(src)),
        Sub(dest, src) => format!("\tsub {}, {}", operand_to_string(dest), operand_to_string(src)),
        Cmp(dest, src) => format!("\tcmp {}, {}", operand_to_string(dest), operand_to_string(src)),
        And(dest, src) => format!("\tand {}, {}", operand_to_string(dest), operand_to_string(src)),
        Or(dest, src) => format!("\tor {}, {}", operand_to_string(dest), operand_to_string(src)),
        Jmp(dest) => format!("\tjmp {}", label_name(dest)),
        Je(dest) => format!("\tje {}", label_name(dest)),
        Jne(dest) => format!("\tjne {}", label_name(dest)),
        Jl(dest) => format!("\tjl {}", label_name(dest)),
        Jnl(dest) => format!("\tjnl {}", label_name(dest)),
        Jg(dest) => format!("\tjg {}", label_name(dest)),
        Jng(dest) => format!("\tjng {}", label_name(dest)),
        Ret => "\tret".to_string(),
        Push(operand) => format!("\tpush {}", operand_to_string(operand)),
        Pop(operand) => format!("\tpop {}", operand_to_string(operand)),
        Call(label) => format!("\tcall {}", label_name(label)),
        Comment(comment) => format!("; {comment}"),
    }
}
