use anyhow::Result;
use clap::Parser;
use std::process::Command;
use std::path::PathBuf;

#[derive(Parser)]
pub struct CodeQlArgs {
    #[clap(long, help = "Path to codeql executable")]
    pub codeql_cli: Option<PathBuf>,

    #[clap(long, help = "Path to codeql database")]
    pub database: Option<PathBuf>,
}

pub fn run_analysis(args: &CodeQlArgs) -> Result<()> {
    let codeql_path = args.codeql_cli
        .clone()
        .unwrap_or_else(|| PathBuf::from("codeql"));

    // Initialize database
    Command::new(&codeql_path)
        .args(&["database", "create", "anya-db", "--language=ruby"])
        .status()?;

    // Run analysis
    Command::new(&codeql_path)
        .args(&["database", "analyze", "anya-db", 
               "--format=sarif-latest", 
               "--output=anya-results.sarif"])
        .status()?;

    Ok(())
}
