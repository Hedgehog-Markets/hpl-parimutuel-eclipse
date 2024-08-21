use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
enum Cli {
    /// Check program security.txt.
    Security {
        /// Path to the program binary.
        program: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli {
        Cli::Security { program } => security(&program),
    }
}

fn security(program: &Path) -> Result<()> {
    let program_data = std::fs::read(program).context("failed to read program binary")?;
    let security_txt =
        security_txt::parse::find_and_parse(&program_data).context("invalid security.txt")?;

    println!("{security_txt}");

    Ok(())
}
