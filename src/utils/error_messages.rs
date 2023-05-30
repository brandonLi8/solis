// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! This file is responsible for error messaging for compiler errors. A compiler error refers to a state when a
//! compiler fails to compile a piece of computer program source code.

use colored::Colorize;
use std::backtrace::Backtrace;
use utils::context::{Context, Position};

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

    /// Error occurred at any generic position of the source code.
    Position(&'a Position),
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
    #[cfg(feature = "test")]
    colored::control::SHOULD_COLORIZE.set_override(false);

    // Convenience aliases.
    let file = &context.file;
    let file_path = &context.file_path;
    let last_index = file.len() - 1;
    let is_not_whitespace = |c: char| !c.is_whitespace();

    // Convert the ErrorPosition to a `Range<usize>`
    let (error_start, error_end) = match error_position {
        ErrorPosition::EndOfFile => {
            // Get the index of where the white-space at the end of the file starts
            let whitespace_start_index = file.rfind(is_not_whitespace).unwrap_or(last_index) + 1;
            (whitespace_start_index, whitespace_start_index + 1)
        }
        ErrorPosition::Index(index) => (index, index + 1),
        ErrorPosition::Span(span) => (span.start, span.end),
        ErrorPosition::WhitespaceBefore(span) => {
            // Get the index of where the white-space before the token starts
            let whitespace_start_index = file[..span.start].rfind(is_not_whitespace).map_or(0, |i| i + 1);
            (whitespace_start_index, whitespace_start_index + 1)
        }
        ErrorPosition::Position(position) => (position.start, position.end),
    };

    // Get the entire code-block that is displayed.
    let next_newline = file[error_end - 1..].find('\n').map_or(last_index + 1, |i| error_end + i);
    let prev_newline = file[..error_start].rfind('\n').map_or(0, |i| i + 1);
    let error_block_raw = &context.file[prev_newline..next_newline];

    // Compute the line number and column for the start of the error.
    let line_number = if error_start > 0 { file[..error_start].matches('\n').count() } else { 0 } + 1;
    let column = error_start - prev_newline;

    let bar = "|".blue().bold();
    let bar_padding = " ".repeat((error_block_raw.lines().count() + line_number - 1).to_string().len());

    // Format the error_block_raw for the purposes of error messaging.
    let error_block: String = error_block_raw
        .lines()
        .enumerate()
        .scan(prev_newline, |line_start_index, (i, line)| {
            let line_end_index = *line_start_index + line.len();

            let carret_padding = " ".repeat(if i == 0 { column } else { 0 });
            let num_carets = if line_end_index >= error_end {
                error_end - carret_padding.len() - *line_start_index
            } else if carret_padding.len() >= line.len() {
                1
            } else {
                line.len() - carret_padding.len()
            };

            *line_start_index = line_end_index;

            Some(format!(
                "{: >bar_padding_len$} {bar} {line}\n{bar_padding} {bar} {carret_padding}{carets}\n",
                (line_number + i).to_string().blue().bold(),
                bar_padding_len = bar_padding.len(),
                carets = "^".repeat(num_carets).yellow().bold()
            ))
        })
        .collect();

    let error_message = format!(
        "{error}: {message}\n {arrow} {file_path}:{line_number}:{column}\n\
         {bar_padding} {bar}\n\
         {error_block}",
        error = "Error".red().bold(),
        message = message.bold(),
        arrow = "-->".blue().bold(),
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
