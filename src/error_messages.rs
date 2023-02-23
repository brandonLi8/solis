// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! This file is responsible for error messaging for compiler errors. A compiler error refers to a state when a
//! compiler fails to compile a piece of computer program source code.

use colored::Colorize;
use ir::type_checker::SolisType;
use std::backtrace::Backtrace;
use std::fmt::{self, Display};
use std::ops::Range;
use File;

/// Called when there is an error within the Solis **input program** at compile time. There are a variety of reasons for
/// when compilation errors occur, such as syntax errors, etc. This function aims to provide helpful error messages for
/// the user by pretty printing a snippet of the Solis input, pin pointing where the error is happening. This function
/// was inspired after rust's own error messages:
/// ```
/// error: syntax error
///  --> input_file.sl:2:8
///   |
/// 2 | let var - int = 32
///   |         ^
/// ```
/// * file: the original Solis file
/// * position: describes where the error is in the source code (for pin pointing), as a index range.
///             **It is assumed that position lies on 1 line and is valid and in bounds**
/// * message: the error message to display
pub fn compilation_error(file: &File, position: &Range<usize>, message: &str) -> ! {
    let next_newline_search = file.contents[position.start..].find('\n');
    let mut next_newline = next_newline_search.unwrap_or(file.contents.len() - 1 - position.start) + position.start;
    let mut newline_indicies: Vec<usize> = file.contents[..=next_newline]
        .match_indices('\n')
        .map(|(i, _)| i)
        .collect();
    if next_newline_search.is_none() {
        newline_indicies.push(next_newline);
        next_newline += 1;
    }

    let prev_newline = if newline_indicies.len() == 1 { 0 } else { newline_indicies[newline_indicies.len() - 2] + 1 };

    // Compute the line number and the column within that line number
    let line_number = newline_indicies.len();
    let column = position.start - prev_newline;

    println!(
        "{error}: {message}\n {arrow} {filename}:{line_number}:{column}\n  \
                 {bar_padding}{bar}\n\
        {display_line_number} {bar} {line}\n  \
                 {bar_padding}{bar} {padding}{caret}\n",
        error = "Error".red().bold(),
        message = message.bold(),
        arrow = "-->".blue().bold(),
        filename = file.name,
        line_number = line_number,
        column = column,
        bar_padding = " ".repeat(line_number.to_string().len() - 1),
        bar = "|".blue().bold(),
        display_line_number = line_number.to_string().blue().bold(),
        line = &file.contents[prev_newline..next_newline],
        padding = " ".repeat(column),
        caret = "^".repeat(position.len()).yellow().bold()
    );

    // For testing purposes, we don't want to exit() when we want to test that certain inputs raise errors.
    // Instead, we are able to test for panics.
    if cfg!(feature = "test") {
        panic!("{} at {:?}", message, position);
    } else {
        std::process::exit(exitcode::DATAERR)
    }
}

/// Called when there is an error within the Solis **compiler** itself, at compile time. Ideally, this should never be
/// called at all.
pub fn internal_compiler_error(message: &str) -> ! {
    let backtrace = Backtrace::force_capture();
    println!(
        "{error}: {message}\n\n\
      Please submit a full bug report at https://github.com/brandonLi8/solis/issues with the backtrace below.\n\n\
      Backtrace: \n {backtrace}",
        error = "Internal Compiler Error".red().bold(),
    );

    if cfg!(feature = "test") {
        panic!("Internal Compiler Error: {}\n{}", message, backtrace);
    } else {
        std::process::exit(exitcode::DATAERR)
    }
}

/// For user facing, create more precise display messages for `SolisTypes` for error messaging purposes.
impl Display for SolisType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Int => write!(f, "int"),
            Self::Bool => write!(f, "bool"),
            Self::Float => write!(f, "float"),
            Self::Custom(s) => write!(f, "{s}"),
        }
    }
}
