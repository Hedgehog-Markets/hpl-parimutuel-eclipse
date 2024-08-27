use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};

#[derive(Parser)]
pub struct Cli {
    /// Type of IDL to check for.
    kind: IdlKind,
    /// Path to the program binary.
    program: PathBuf,
}

#[derive(Clone, Copy, PartialEq, Eq, ValueEnum)]
enum IdlKind {
    Anchor,
    Kinobi,
}

impl From<IdlKind> for include_idl::parse::IdlKind {
    fn from(value: IdlKind) -> Self {
        match value {
            IdlKind::Anchor => Self::Anchor,
            IdlKind::Kinobi => Self::Kinobi,
        }
    }
}

impl Cli {
    pub fn run(self) -> Result<()> {
        let program_data = fs::read(self.program).context("failed to read program binary")?;

        let idl = include_idl::parse::parse_from_program(&program_data, self.kind.into())
            .context("failed to find IDL in program binary")?;

        println!("{idl:#}");

        Ok(())
    }
}
