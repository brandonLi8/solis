// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! A symbol table is a type that is created on the fly in the compiler. It maps variables to where they are stored at
//! runtime. This file contains definitions and helper functions for dealing with the symbol table.

use asm::asm::{
    FloatRegister,
    Operand::{self, *},
    Register,
};
use compiler::compiler::stack_address;
use std::collections::HashMap;

/// Symbol Table type
pub type SymbolTable = HashMap<String, Location>;

// Where the symbol is located at run time.
pub enum Location {
    Register(Register),
    FloatRegister(FloatRegister),
    StackIndex(i64),
}

impl Location {
    /// Converts a Location to a `asm::Operand`
    pub fn to_operand(&self) -> Operand {
        match self {
            Self::Register(register) => Reg(*register),
            Self::FloatRegister(register) => FloatReg(*register),
            Self::StackIndex(offset) => stack_address(*offset),
        }
    }
}
