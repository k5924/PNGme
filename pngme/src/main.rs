mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;
mod png_error;
mod utils;

use crate::png_error::PngError;
use clap::Parser;

fn main() -> anyhow::Result<(), PngError> {
    let cli = args::Cli::parse();

    return match &cli.command {
        args::Command::Encode(cmd) => commands::encode(cmd),
        args::Command::Decode(cmd) => commands::decode(cmd),
        args::Command::Remove(cmd) => commands::remove(cmd),
        args::Command::Print(cmd) => commands::print_chunks(cmd)
    }
}

