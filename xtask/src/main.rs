use anyhow::Result;
use clap::Parser;

mod idl;
mod security;

#[derive(Parser)]
enum Cli {
    /// Check program security.txt.
    Security(security::Cli),
    /// Check program embedded IDL.
    Idl(idl::Cli),
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli {
        Cli::Security(cli) => cli.run(),
        Cli::Idl(cli) => cli.run(),
    }
}
