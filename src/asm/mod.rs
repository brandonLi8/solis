// Copyright © 2022 Brandon Li. All rights reserved.

//! The asm module contains definitions for our representation of assembly and is responsible for transforming it into
//! an actual assembly file.

pub mod asm;
pub mod asm_writer;

// tests
#[cfg(test)]
mod asm_writer_tests;
