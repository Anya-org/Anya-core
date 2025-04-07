use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::*;
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Tool to fix common issues in the Anya Core codebase
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Fix edition inheritance in Cargo.toml files
    FixEdition {
        /// Apply changes (without this flag, only reports issues)
        #[arg(short, long)]
        apply: bool,
    },
    /// Fix package references in Cargo.toml files
    FixPackageRefs {
        /// Apply changes (without this flag, only reports issues)
        #[arg(short, long)]
        apply: bool,
    },
    /// Fix both edition inheritance and package references (recommended)
    FixAll {
        /// Apply changes (without this flag, only reports issues)
        #[arg(short, long)]
        apply: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::FixEdition { apply } => {
            fix_edition_inheritance(*apply)?;
        }
        Commands::FixPackageRefs { apply } => {
            fix_package_references(*apply)?;
        }
        Commands::FixAll { apply } => {
            fix_edition_inheritance(*apply)?;
            fix_package_references(*apply)?;
        }
    }

    Ok(())
}

/// Fix edition inheritance in Cargo.toml files
fn fix_edition_inheritance(apply: bool) -> Result<()> {
    println!("{}", "Checking edition inheritance in Cargo.toml files...".blue().bold());
    
    let edition_regex = Regex::new(r#"edition\.workspace\s*=\s*true"#)?;
    let workspace_regex = Regex::new(r#"(?ms)^\[workspace\].*?(\[.*?\])"#)?;
    
    let mut edition_fixes = 0;
    let mut workspace_fixes = 0;
    
    for entry in WalkDir::new(".")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_type().is_file() 
            && e.file_name().to_string_lossy() == "Cargo.toml"
            && e.path().to_string_lossy() != "./Cargo.toml" // Skip root Cargo.toml
        })
    {
        let path = entry.path();
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read {}", path.display()))?;
            
        let mut modified_content = content.clone();
        let mut file_modified = false;
        
        // Fix edition inheritance
        if edition_regex.is_match(&content) {
            edition_fixes += 1;
            if apply {
                modified_content = edition_regex.replace_all(&modified_content, r#"edition = "2021""#).to_string();
                file_modified = true;
                println!("  {}: Fixed edition inheritance", path.display().to_string().green());
            } else {
                println!("  {}: Found edition inheritance issue", path.display().to_string().yellow());
            }
        }
        
        // Fix workspace section
        if workspace_regex.is_match(&content) {
            workspace_fixes += 1;
            if apply {
                modified_content = workspace_regex.replace_all(&modified_content, "# REMOVED CONFLICTING WORKSPACE SECTION\n$1").to_string();
                file_modified = true;
                println!("  {}: Removed conflicting workspace section", path.display().to_string().green());
            } else {
                println!("  {}: Found conflicting workspace section", path.display().to_string().yellow());
            }
        }
        
        // Write changes to file
        if apply && file_modified {
            fs::write(path, modified_content)
                .with_context(|| format!("Failed to write to {}", path.display()))?;
        }
    }
    
    println!("{} {} edition inheritance issues", 
        if apply { "Fixed".green().bold() } else { "Found".yellow().bold() },
        edition_fixes);
    println!("{} {} conflicting workspace sections", 
        if apply { "Fixed".green().bold() } else { "Found".yellow().bold() },
        workspace_fixes);
        
    if !apply && (edition_fixes > 0 || workspace_fixes > 0) {
        println!("{} Run with --apply to fix these issues", "Note:".cyan().bold());
    }
    
    Ok(())
}

/// Fix package references in Cargo.toml files
fn fix_package_references(apply: bool) -> Result<()> {
    println!("{}", "Checking package references in Cargo.toml files...".blue().bold());
    
    let package_regex = Regex::new(r#"anya-core-core"#)?;
    let mut fixes = 0;
    
    for entry in WalkDir::new(".")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_type().is_file() 
            && e.file_name().to_string_lossy() == "Cargo.toml"
            && e.path().to_string_lossy() != "./Cargo.toml" // Skip root Cargo.toml
        })
    {
        let path = entry.path();
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read {}", path.display()))?;
            
        if package_regex.is_match(&content) {
            fixes += 1;
            
            if apply {
                let modified_content = package_regex.replace_all(&content, "anya-core-lib").to_string();
                fs::write(path, modified_content)
                    .with_context(|| format!("Failed to write to {}", path.display()))?;
                println!("  {}: Fixed package references", path.display().to_string().green());
            } else {
                println!("  {}: Found incorrect package references", path.display().to_string().yellow());
            }
        }
    }
    
    println!("{} {} incorrect package references", 
        if apply { "Fixed".green().bold() } else { "Found".yellow().bold() },
        fixes);
        
    if !apply && fixes > 0 {
        println!("{} Run with --apply to fix these issues", "Note:".cyan().bold());
    }
    
    Ok(())
} 