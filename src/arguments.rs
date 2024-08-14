use std::path::PathBuf;

use clap::Parser;

// Scangen test tool
#[derive(Parser)]
#[command(author, version, about)]
pub(crate) struct CliArgs {
    /// Par file to scan
    #[arg(short, long)]
    pub parfile: PathBuf,
    /// Define whether to print the tokens and other information
    #[arg(short, long)]
    pub quiet: bool,
}
