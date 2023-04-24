// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Defines the schema for the CLI and responsible for parsing command line arguments.

use clap::Parser;

#[derive(Parser)]
#[command(author = "Brandon Li <brandon.li@berkeley.edu>", version)]
pub struct CLIDriver {
    /// The input Solis file to compile
    pub file: String,

    /// Name of the executable. [default: file name of FILE]
    #[arg(short, long)]
    pub name: Option<String>,

    /// Output directory, including artifacts.
    #[arg(short = 'd', long = "dest", default_value = "build")]
    pub destination: String,

    /// Use to immediately run executable after compiling.
    #[arg(short, long)]
    pub run: bool,

    /// Use to remove the contents in DESTINATION before compiling.
    #[arg(short, long)]
    pub clean: bool,
}
