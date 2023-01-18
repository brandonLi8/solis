// Copyright © 2022 Brandon Li. All rights reserved.

//! Unit tests for the `asm_writer`.

use asm::asm::{Instruction::*, Operand::*, Register::*};
use asm::asm_writer::write_instructions_to_file;
use expect_test::expect;
use std::fs;
use std::path::Path;

#[test]
fn test_basic() {
    let instructions = vec![
        Global("some_label".to_string()),
        Extern("some_label".to_string()),
        Label("some_label".to_string()),
        Mov(Reg(R15), Imm(2)),
        Add(Reg(R15), Imm(2)),
        Sub(Reg(R15), Imm(2)),
        Cmp(Reg(R15), Imm(2)),
        And(Reg(R15), Imm(2)),
        Or(Reg(R15), Imm(2)),
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
        global some_label
        extern some_label
        some_label:
        	mov r15, 2
        	add r15, 2
        	sub r15, 2
        	cmp r15, 2
        	and r15, 2
        	or r15, 2
        	jmp some_label
        	je some_label
        	jne some_label
        	jl some_label
        	jnl some_label
        	jg some_label
        	jng some_label
        	ret
        	push rax
        	pop 2
        	call some_label
        ; some comment
    "#]]
    .assert_eq(&fs::read_to_string(temporary_file).unwrap());
}
