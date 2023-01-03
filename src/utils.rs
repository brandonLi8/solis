// Copyright © 2022 Brandon Li. All rights reserved.

// Utilities used throughout this project.

use colored::Colorize;

/// Error handler for Solis, which pretty prints a snippet of the Solis source code and pin points where
/// the error is happening. This function was designed after rustc's own error messages, which look like
/// ```
/// error: syntax error
///  --> input_file.sl:2:8
///   |
/// 2 | let var - int = 32
///   |         ^
/// ```
pub fn raise_code_error(file: String, cursor: usize, message: &str) -> ! {
    let filename = "examples/example.sl"; // TODO

    let next_newline_search = file[cursor..].find("\n");
    let next_newline = next_newline_search.unwrap_or(file.len() - 1 - cursor) + cursor;
    let mut newline_indicies: Vec<usize> = file[..next_newline + 1].match_indices("\n").map(|(i, _)| i).collect();
    if next_newline_search.is_none() { newline_indicies.push(next_newline); }

    let prev_newline = if newline_indicies.len() == 1 { 0 } else { newline_indicies[newline_indicies.len() - 2] + 1 };

    // Compute the line number and the character_position within that line number
    let line_number = newline_indicies.len();
    let character_position = cursor - prev_newline;

    println!("{error}: {message}\n {arrow} {filename}\n  {bar}\n{line_number} {bar} {line}\n  {bar} {padding}{caret}\n",
      error = format!("Error").red().bold(),
      message = format!("{}", message).bold(),
      arrow = format!("-->").blue().bold(),
      filename = format!("{}:{}:{}", filename, line_number, character_position),
      bar = format!("|").blue().bold(),
      line_number = format!("{}", line_number).blue().bold(),
      line = format!("{}", &file[prev_newline .. next_newline]),
      padding = " ".repeat(character_position),
      caret = format!("^").yellow().bold()
    );

    // For testing purposes, we don't want to exit() when we want to test that certain inputs raise errors.
    // Instead, we are able to test for panics.
    #[cfg(test)]
    panic!("{}", message);

    #[cfg(not(test))]
    std::process::exit(exitcode::DATAERR)
}
