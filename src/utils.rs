// Copyright © 2022 Brandon Li. All rights reserved.

//! Utilities used throughout this project.

use colored::Colorize;
use std::backtrace::Backtrace;
use std::fs;
use std::ops::Range;
use std::process::exit;

/// Information about the source Solis file, grouped together in a single struct to pass between stages of compilation.
pub struct File {
    pub name: String,
    pub contents: String,
}

/// Reads in the Solis file.
pub fn read_file(file_name: &String) -> File {
    File {
        name: file_name.to_string(),
        contents: fs::read_to_string(file_name).unwrap_or_else(|_| {
            println!("{}: no such file {file_name}", "Error".red().bold());
            exit(exitcode::DATAERR)
        }),
    }
}

/// Called when there is an error within the Solis **input program**. There are a variety of reasons for when
/// compilation errors occur, such as syntax errors, etc. This function aims to provide helpful error messages for the
/// user by pretty printing a snippet of the Solis input, pin pointing where the error is happening. This function was
/// inspired after rust's own error messages:
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

    // Compute the line number and the character_position within that line number
    let line_number = newline_indicies.len();
    let character_position = position.start - prev_newline;

    println!(
        "{error}: {message}\n {arrow} {filename}:{line_number}:{character_position}\n  \
                              {bar}\n\
        {display_line_number} {bar} {line}\n  \
                              {bar} {padding}{caret}\n",
        error = "Error".red().bold(),
        message = message.bold(),
        arrow = "-->".blue().bold(),
        filename = file.name,
        line_number = line_number,
        character_position = character_position,
        bar = "|".blue().bold(),
        display_line_number = line_number.to_string().blue().bold(),
        line = &file.contents[prev_newline..next_newline],
        padding = " ".repeat(character_position),
        caret = "^".repeat(position.len()).yellow().bold()
    );

    // For testing purposes, we don't want to exit() when we want to test that certain inputs raise errors.
    // Instead, we are able to test for panics.
    #[cfg(test)]
    panic!("{} at {:?}", message, position);

    #[cfg(not(test))]
    exit(exitcode::DATAERR)
}

/// Called when there is an error within the Solis **compiler** itself. Ideally, this should never be called at all.
pub fn internal_compiler_error(message: &str) -> ! {
    println!(
        "{error}: {message}\n\n\
      Please submit a full bug report at https://github.com/brandonLi8/solis/issues with the backtrace below.\n\n\
      Backtrace: \n {backtrace}",
        error = "Internal Compiler Error".red().bold(),
        backtrace = Backtrace::force_capture()
    );

    exit(exitcode::SOFTWARE)
}
