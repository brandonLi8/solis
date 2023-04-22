// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! This file is responsible for error messaging for compiler errors. A compiler error refers to a state when a
//! compiler fails to compile a piece of computer program source code.

use colored::Colorize;
use context::{Context, Position};
use std::backtrace::Backtrace;

/// Used to determine behavior of the `compilation_error` function.
pub enum ErrorPosition<'a> {
    /// Error occurred at the end of the file
    EndOfFile,

    /// Error occurred at a specified index.
    Index(usize),

    /// Error occurred at a span, which is the Position of a token.
    Span(&'a Position),

    /// Error occurred before this token (as a span) and the previous token. More specifically, if there is white-space
    /// before this token, the error position is there. If there is no white-space before (new-lines included), then
    /// the error occurred at the start of the position.
    WhitespaceBefore(&'a Position),
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
/// * `context`: compilation context
/// * `error_position`: describes where the error is in the source code (for pin pointing)
/// * `message`: the error message to display
pub fn compilation_error(context: &Context, error_position: ErrorPosition, message: &str) -> ! {
    // Convenience aliases.
    let file = &context.file;
    let file_path = &context.file_path;
    let last_index = file.len() - 1;
    let is_not_whitespace = |c: char| !c.is_whitespace();

    // Convert the ErrorPosition to a `Range<usize>`
    let (error_start, error_length) = match error_position {
        ErrorPosition::EndOfFile => {
            // Get the index of where the white-space at the end of the file starts
            let whitespace_start_index = file.rfind(is_not_whitespace).unwrap_or(last_index) + 1;
            (whitespace_start_index, 1)
        }
        ErrorPosition::Index(index) => (index, 1),
        ErrorPosition::Span(span) => (span.start, span.len()),
        ErrorPosition::WhitespaceBefore(span) => {
            // Get the index of where the white-space before the token starts
            let whitespace_start_index = file[..span.start].rfind(is_not_whitespace).map_or(0, |i| i + 1);
            (whitespace_start_index, 1)
        }
    };

    // Get the index of the next_newline (or last_index + 1)
    let next_newline = file[error_start..]
        .find('\n')
        .map_or(last_index + 1, |i| error_start + i);

    // Get the index of where the line starts (or 0)
    let line_start_index = file[..error_start].rfind('\n').map_or(0, |i| i + 1);

    // Compute the line number and the column within that line number
    let line_number = if error_start > 0 { file[..error_start].lines().count() } else { 1 };
    let column = error_start - line_start_index;

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
        bar_padding = " ".repeat(line_number.to_string().len() - 1),
        bar = "|".blue().bold(),
        display_line_number = line_number.to_string().blue().bold(),
        line = &context.file[line_start_index..next_newline],
        padding = " ".repeat(column),
        caret = "^".repeat(error_length).yellow().bold()
    );

    // For testing purposes, we don't want to exit() when we want to test that certain inputs raise errors.
    // Instead, we are able to test for panics.
    if cfg!(feature = "test") {
        panic!("{}", error_message.normal());
    } else {
        eprintln!("{error_message}");
        std::process::exit(exitcode::DATAERR)
    }
}

/// Called when there is an error within the Solis **compiler** itself, at compile time. Ideally, this should never be
/// called at all.
pub fn internal_compiler_error(message: &str) -> ! {
    let backtrace = Backtrace::force_capture();
    eprintln!(
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
