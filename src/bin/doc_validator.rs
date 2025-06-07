/// Documentation validator for Bitcoin compliance [AIS-3][BPC-3][DAO-3]
use std::error::Error;

use crate::tools::markdown::{DocumentationValidator, DocError};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[clap(
    name = "anya-doc-validator",
    about = "Documentation validation tool [AIS-3][BPC-3][DAO-3]",
    version
)]
struct Cli {
    /// Root directory to scan
    #[clap(short, long, default_value = ".")]
    dir: PathBuf,
    
    /// Fix issues automatically
    #[clap(short, long)]
    fix: bool,
    
    /// Only show summary
    #[clap(short, long)]
    summary: bool,
}

fn main() -> Result<(), DocError> {
    let cli = Cli::parse();
    
    let validator = DocumentationValidator::new(cli.dir);
    let report = validator.validate_all(cli.fix)?;
    report.print();
    
    if report.issue_count() > 0 && !cli.fix {
        std::process::exit(1);
    }
    
    Ok(())
} 
