// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! This file is responsible for error messaging for compiler errors. A compiler error refers to a state when a
//! compiler fails to compile a piece of computer program source code.

use colored::Colorize;
use context::{Context, Position};
use std::backtrace::Backtrace;

/// Used to determine behavior of the `compilation_error` function.
pub enum ErrorPosition {
    /// Error occurred at the end of the file
    EndOfFile,

    /// Error occurred at a specified index.
    Index(usize),

    /// Error occurred at a span, which is the Position of a token.
    Span(Position),

    /// Error occurred before this token (as a span) and the previous token. More specifically, if there is white-space
    /// before this token, the error position is there. If there is no white-space before (new-lines included), then
    /// the error occurred at the start of the position.
    WhitespaceBefore(Position),
}

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
/// * context: compilation context
/// * error_position: describes where the error is in the source code (for pin pointing)
/// * message: the error message to display
pub fn compilation_error(context: &Context, error_position: ErrorPosition, message: &str) -> ! {
    let position = match error_position {
        ErrorPosition::EndOfFile => {
            let last_non_whitespace_index = context.file.rfind(|c: char| !c.is_whitespace()).unwrap_or(usize::MAX);
            last_non_whitespace_index + 1..last_non_whitespace_index + 2
        }
        ErrorPosition::Index(index) => index..index + 1,
        ErrorPosition::Span(span) => span,
        ErrorPosition::WhitespaceBefore(span) => {
            let last_non_whitespace_index = context.file[..span.start]
                .rfind(|c: char| !c.is_whitespace())
                .unwrap_or(usize::MAX);
            last_non_whitespace_index + 1..last_non_whitespace_index + 2
        }
    };

    let next_newline_search = context.file[position.start..].find('\n');
    let mut next_newline = next_newline_search.unwrap_or(context.file.len() - 1 - position.start) + position.start;
    let mut newline_indicies: Vec<usize> = context.file[..=next_newline]
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

    // Disable coloring on unit tests.
    #[cfg(feature = "test")]
    {
        use colored::control::SHOULD_COLORIZE;
        SHOULD_COLORIZE.set_override(false);
    }

    let error_message = format!(
        "{error}: {message}\n {arrow} {file_path}:{line_number}:{column}\n  \
                 {bar_padding}{bar}\n\
        {display_line_number} {bar} {line}\n  \
                 {bar_padding}{bar} {padding}{caret}\n",
        error = "Error".red().bold(),
        message = message.bold(),
        arrow = "-->".blue().bold(),
        file_path = context.file_path,
        line_number = line_number,
        column = column,
        bar_padding = " ".repeat(line_number.to_string().len() - 1),
        bar = "|".blue().bold(),
        display_line_number = line_number.to_string().blue().bold(),
        line = &context.file[prev_newline..next_newline],
        padding = " ".repeat(column),
        caret = "^".repeat(position.len()).yellow().bold()
    );

    // For testing purposes, we don't want to exit() when we want to test that certain inputs raise errors.
    // Instead, we are able to test for panics.
    if cfg!(feature = "test") {
        panic!("{}", error_message.normal());
    } else {
        println!("{error_message}");
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
