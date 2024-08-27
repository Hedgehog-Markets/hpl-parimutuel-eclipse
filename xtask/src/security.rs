use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    /// Path to the program binary.
    program: PathBuf,
}

impl Cli {
    pub fn run(self) -> Result<()> {
        let program_data = std::fs::read(self.program).context("failed to read program binary")?;
        let security_txt = security_txt::parse::parse_from_program(&program_data)
            .context("failed to read security.txt in program binary")?;

        println!("{security_txt}");

        Ok(())
    }
}
