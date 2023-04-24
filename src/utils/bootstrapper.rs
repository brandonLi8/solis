// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! The bootstrapper is responsible for the entire process to create an executable for the input Solis program after
//! compilation.  Specifically, after compiling the Solis program, we:
//!  1. write the assembly to a file
//!  2. assemble the file to an object file, using `nasm`.
//!  3. link the object file with our runtime (runtime/runtime.c)
//!  4. Optionally run (load) the executable

use asm::asm::Instruction;
use asm::asm_writer::write_instructions_to_file;
use colored::Colorize;
use utils::error_messages::internal_compiler_error;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

/// Bootstrap the compiler output to an executable, and optionally run it.
/// * instructions - compiler output
/// * directory - the directory to create the intermediate files and resulting executable
/// * name - the name of the executable, within `directory`
/// * run - indicates if we should run the executable after creating it.
/// * clean - indicates if we should remove the contents of the directory before creating executable
pub fn bootstrap(instructions: Vec<Instruction>, directory: &Path, name: &str, run: bool, clean: bool) {
    let assembly_file_path = &directory.join(format!("{name}.s"));
    let object_file_path = &directory.join(format!("{name}.o"));
    let runtime_object_file_path = &directory.join("runtime.o");
    let executable_file_path = &directory.join(name);

    // Clean the output directory
    if clean {
        ensure_success(Command::new("rm").arg("-rf").arg(directory));
    }

    // Write the instructions to an assembly file.
    write_instructions_to_file(instructions, assembly_file_path);

    // Run the assembler to create an object file.
    ensure_success(
        Command::new("nasm")
            .arg(assembly_file_path)
            .arg("-f")
            .arg(if cfg!(target_os = "macos") { "macho64" } else { "elf64" })
            .arg("-o")
            .arg(object_file_path),
    );

    // Compile the Solis runtime.
    ensure_success(
        Command::new("cc")
            .arg("-c")
            .arg("./src/runtime/runtime.c")
            .arg("-o")
            .arg(runtime_object_file_path),
    );

    // Link the object file with the runtime.
    ensure_success(
        Command::new("cc")
            .arg(object_file_path)
            .arg(runtime_object_file_path)
            .arg("-o")
            .arg(executable_file_path),
    );

    // Optionally run (load) the executable.
    if run {
        let output = Command::new(executable_file_path)
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
