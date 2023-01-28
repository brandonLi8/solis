// Copyright © 2022 Brandon Li. All rights reserved.

//! Unit tests for the `asm_writer`.

use expect_test::expect;
use solis::asm::asm::{Instruction::*, Operand::*, Register::*};
use solis::asm::asm_writer::write_instructions_to_file;
use std::fs;
use std::path::Path;

#[test]
fn test_basic() {
    let instructions = vec![
        Global("some_label".to_string()),
        Extern("some_label".to_string()),
        Label("some_label".to_string()),
        Mov(Reg(Rax), Imm(-123)),
        Mov(Reg(Rax), Reg(Rax)),
        Mov(Reg(Rsi), Reg(Rax)),
        Mov(Reg(Rdi), Reg(Rsi)),
        Mov(Reg(Rsp), Reg(Rdi)),
        Mov(Reg(Rbp), Reg(Rsp)),
        Mov(Reg(R8), Reg(Rbp)),
        Mov(Reg(R9), Reg(R8)),
        Mov(Reg(R10), Reg(R9)),
        Mov(Reg(R11), Reg(R10)),
        Mov(Reg(R12), Reg(R11)),
        Mov(Reg(R13), Reg(R12)),
        Mov(Reg(R14), Reg(R13)),
        Mov(Reg(R15), Reg(R14)),
        Label("some_label2".to_string()),
        Mov(MemOffset(Box::new(Reg(Rax)), Box::new(Imm(1))), Reg(Rax)),
        Mov(Reg(Rax), MemOffset(Box::new(Reg(Rax)), Box::new(Imm(1)))),
        Mov(MemOffset(Box::new(Reg(Rax)), Box::new(Imm(1))), *Box::new(Imm(1))),
        Add(MemOffset(Box::new(Reg(Rax)), Box::new(Imm(1))), Reg(Rax)),
        Add(Reg(Rax), MemOffset(Box::new(Reg(Rax)), Box::new(Imm(1)))),
        Add(MemOffset(Box::new(Reg(Rax)), Box::new(Imm(1))), *Box::new(Imm(1))),
        Sub(MemOffset(Box::new(Reg(Rax)), Box::new(Imm(1))), Reg(Rax)),
        Sub(Reg(Rax), MemOffset(Box::new(Reg(Rax)), Box::new(Imm(1)))),
        Sub(MemOffset(Box::new(Reg(Rax)), Box::new(Imm(1))), *Box::new(Imm(1))),
        Cmp(MemOffset(Box::new(Reg(Rax)), Box::new(Imm(1))), Reg(Rax)),
        Cmp(Reg(Rax), MemOffset(Box::new(Reg(Rax)), Box::new(Imm(1)))),
        Cmp(MemOffset(Box::new(Reg(Rax)), Box::new(Imm(1))), *Box::new(Imm(1))),
        And(MemOffset(Box::new(Reg(Rax)), Box::new(Imm(1))), Reg(Rax)),
        And(Reg(Rax), MemOffset(Box::new(Reg(Rax)), Box::new(Imm(1)))),
        And(MemOffset(Box::new(Reg(Rax)), Box::new(Imm(1))), *Box::new(Imm(1))),
        Or(MemOffset(Box::new(Reg(Rax)), Box::new(Imm(1))), Reg(Rax)),
        Or(Reg(Rax), MemOffset(Box::new(Reg(Rax)), Box::new(Imm(1)))),
        Or(MemOffset(Box::new(Reg(Rax)), Box::new(Imm(1))), *Box::new(Imm(1))),
        Jmp("some_label".to_string()),
        Je("some_label".to_string()),
        Jne("some_label".to_string()),
        Jl("some_label".to_string()),
        Jnl("some_label".to_string()),
        Jg("some_label".to_string()),
        Jng("some_label".to_string()),
        Ret,
        Push(Reg(Rax)),
        Pop(Imm(2)),
        Call("some_label".to_string()),
        Comment("some comment".to_string()),
    ];

    let temporary_file = "./build/tmp.s";
    write_instructions_to_file(instructions, Path::new(temporary_file));
    expect![[r#"
        global _some_label
        extern _some_label
        _some_label:
        	mov rax, -123
        	mov rax, rax
        	mov rsi, rax
        	mov rdi, rsi
        	mov rsp, rdi
        	mov rbp, rsp
        	mov r8, rbp
        	mov r9, r8
        	mov r10, r9
        	mov r11, r10
        	mov r12, r11
        	mov r13, r12
        	mov r14, r13
        	mov r15, r14
        _some_label2:
        	mov QWORD [rax + 1], rax
        	mov rax, QWORD [rax + 1]
        	mov QWORD [rax + 1], 1
        	add QWORD [rax + 1], rax
        	add rax, QWORD [rax + 1]
        	add QWORD [rax + 1], 1
        	sub QWORD [rax + 1], rax
        	sub rax, QWORD [rax + 1]
        	sub QWORD [rax + 1], 1
        	cmp QWORD [rax + 1], rax
        	cmp rax, QWORD [rax + 1]
        	cmp QWORD [rax + 1], 1
        	and QWORD [rax + 1], rax
        	and rax, QWORD [rax + 1]
        	and QWORD [rax + 1], 1
        	or QWORD [rax + 1], rax
        	or rax, QWORD [rax + 1]
        	or QWORD [rax + 1], 1
        	jmp _some_label
        	je _some_label
        	jne _some_label
        	jl _some_label
        	jnl _some_label
        	jg _some_label
        	jng _some_label
        	ret
        	push rax
        	pop 2
        	call _some_label
        ; some comment
    "#]]
    .assert_eq(&fs::read_to_string(temporary_file).unwrap());
}
