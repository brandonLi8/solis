// Copyright © 2022 Brandon Li. All rights reserved.

//! The bootstrapper is responsible for the entire process to create an executable for the input Solis program after
//! compilation.  Specifically, after compiling the Solis program, we:
//!  1. write the assembly to a file
//!  2. assemble the file to an object file, using `nasm`.
//!  3. link the object file with our runtime (runtime/runtime.c)
//!  4. Optionally run (load) the executable

use asm::asm::Instruction;
use asm::asm_writer::write_instructions_to_file;
use colored::Colorize;
use error_messages::internal_compiler_error;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

/// Bootstrap the compiler output to an executable, and optionally run it.
/// * instructions - compiler output
/// * directory - the directory to create the intermediate files and resulting executable
/// * name - the name of the executable, within `directory`
/// * run - indicates if we should run the executable after creating it.
pub fn bootstrap(instructions: Vec<Instruction>, directory: &Path, name: &str, run: bool) {
    // Write the instructions to an assembly file.
    write_instructions_to_file(instructions, &directory.join(format!("{name}.s")));

    // Run the assembler to create an object file.
    ensure_success(
        Command::new("nasm")
            .arg("./build/example.s")
            .arg("-f")
            .arg(if cfg!(target_os = "macos") { "macho64" } else { "elf64" })
            .arg("-o")
            .arg("./build/example.o"),
    );

    // Compile the Solis runtime
    ensure_success(
        Command::new("cc")
            .arg("-c")
            .arg("./src/runtime/runtime.c")
            .arg("-o")
            .arg("./build/runtime.o"),
    );

    // Link the object file with the runtime
    ensure_success(
        Command::new("cc")
            .arg("./build/example.o")
            .arg("./build/runtime.o")
            .arg("-o")
            .arg("./build/example"),
    );

    // Optionally run (load) the executable
    if run {
        let output = Command::new("./build/example")
            .output()
            .expect("failed to execute process");

        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
    }
}

// Ensures that a command runs, finishes, and succeeds.
fn ensure_success(command: &mut Command) {
    let output = command
        .status()
        .unwrap_or_else(|error| internal_compiler_error(&format!("bootstrap failed {command:?}: {error}")));

    if !output.success() {
        internal_compiler_error(&format!("`{command:?}` {failed}", failed = "failed".red()))
    }
}
