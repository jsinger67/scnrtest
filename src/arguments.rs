use std::path::PathBuf;

use clap::Parser;

/// Scangen test tool
#[derive(Parser)]
#[command(author, version, about)]
pub(crate) struct CliArgs {
    /// Input file to scan, PAR format if no modes are provided
    #[arg(short, long)]
    pub input: Option<PathBuf>,
    /// Input as text directly on the command line
    #[arg(short, long)]
    pub text: Option<String>,
    /// JSON file with scanner modes
    #[arg(short, long)]
    pub modes: Option<PathBuf>,
    /// Define whether to print the tokens and other information
    #[arg(short, long)]
    pub quiet: bool,
    /// Define whether to trace the compiled DFAs
    #[arg(short = 'r', long)]
    pub trace: bool,
    /// Paterens as a list of strings, optional
    #[arg(short, long)]
    pub patterns: Option<Vec<String>>,
    /// Ouput path for generated automata
    #[arg(short, long)]
    pub dot: Option<PathBuf>,
}
