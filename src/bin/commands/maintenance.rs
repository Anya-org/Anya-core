use std::path::PathBuf;
use structopt::StructOpt;
use anyhow::{Result, Context};
use tokio::process::Command;
use tokio::fs;
use walkdir::WalkDir;
use regex::Regex;

#[derive(StructOpt)]
pub enum MaintenanceCmd {
    /// Enforce code style (replaces enforce_style.ps1)
    #[structopt(name = "style")]
    Style {
        #[structopt(long)]
        fix: bool,
    },

    /// Fix error handling patterns (replaces fix_error_handling.ps1)
    #[structopt(name = "errors")]
    Errors {
        #[structopt(long)]
        check_only: bool,
    },

    /// Manage documentation files (replaces file_management.ps1)
    #[structopt(name = "docs")]
    Docs {
        #[structopt(long)]
        update: bool,
    },

    /// Cleanup workspace (replaces cleanup.ps1)
    #[structopt(name = "cleanup")]
    Cleanup {
        #[structopt(long)]
        dry_run: bool,
    },
}

impl MaintenanceCmd {
    pub async fn run(&self) -> Result<()> {
        match self {
            MaintenanceCmd::Style { fix } => {
                self.enforce_style(*fix).await
            }
            MaintenanceCmd::Errors { check_only } => {
                self.fix_error_handling(*check_only).await
            }
            MaintenanceCmd::Docs { update } => {
                self.manage_docs(*update).await
            }
            MaintenanceCmd::Cleanup { dry_run } => {
                self.cleanup_workspace(*dry_run).await
            }
        }
    }

    async fn enforce_style(&self, fix: bool) -> Result<()> {
        // Run rustfmt
        let args = if fix {
            vec!["fmt"]
        } else {
            vec!["fmt", "--check"]
        };
        
        Command::new("cargo")
            .args(&args)
            .status()
            .await
            .context("Rustfmt failed")?;

        // Run clippy
        let mut clippy_args = vec!["clippy", "--all-targets"];
        if fix {
            clippy_args.extend_from_slice(&["--fix", "--allow-dirty"]);
        }
        clippy_args.extend_from_slice(&["--", "-D", "warnings"]);

        Command::new("cargo")
            .args(&clippy_args)
            .status()
            .await
            .context("Clippy failed")?;

        // Check documentation coverage
        Command::new("cargo")
            .args(["doc", "--no-deps", "--all-features"])
            .status()
            .await
            .context("Documentation check failed")?;

        // Find files missing documentation
        for entry in WalkDir::new("src").into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() && entry.path().extension().map_or(false, |ext| ext == "rs") {
                let content = fs::read_to_string(entry.path()).await?;
                if !content.contains("///") {
                    println!("Warning: No documentation found in {}", entry.path().display());
                }
            }
        }

        Ok(())
    }

    async fn fix_error_handling(&self, check_only: bool) -> Result<()> {
        let unsafe_patterns = [
            (r"\.unwrap\(\)", "?"),
            (r"\.expect\([^)]+\)", "?"),
            (r"panic!\([^)]+\)", "return Err(anyhow::anyhow!($1))"),
        ];

        for entry in WalkDir::new("src").into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() && entry.path().extension().map_or(false, |ext| ext == "rs") {
                let mut content = fs::read_to_string(entry.path()).await?;
                let original = content.clone();
                let mut modified = false;

                // Add error propagation imports if needed
                if !content.contains("use std::error::Error") && content.contains("-> Result<") {
                    content = format!("use std::error::Error;\n{}", content);
                    modified = true;
                }

                // Replace unsafe patterns
                for (pattern, replacement) in unsafe_patterns {
                    let re = Regex::new(pattern).unwrap();
                    if re.is_match(&content) {
                        if check_only {
                            println!("Found unsafe pattern {} in {}", pattern, entry.path().display());
                        } else {
                            content = re.replace_all(&content, replacement).to_string();
                            modified = true;
                        }
                    }
                }

                // Add Result return type if missing
                if !check_only && content.contains("fn ") && !content.contains("-> Result<") {
                    let re = Regex::new(r"fn ([^{]+)\{").unwrap();
                    content = re.replace_all(&content, "fn $1 -> Result<(), Box<dyn Error>> {").to_string();
                    modified = true;
                }

                // Write changes if modified
                if modified && !check_only {
                    fs::write(entry.path(), content).await?;
                    println!("Fixed error handling in {}", entry.path().display());
                }
            }
        }

        Ok(())
    }

    async fn manage_docs(&self, update: bool) -> Result<()> {
        let doc_files = [
            "docs/bitcoin/TAPROOT_ASSETS.md",
            "docs/dao/AUDIT_TRAIL.md",
        ];

        for path in doc_files {
            let path = PathBuf::from(path);
            if !path.exists() || update {
                fs::create_dir_all(path.parent().unwrap()).await?;
                if !path.exists() {
                    fs::write(&path, "").await?;
                    println!("Created documentation file: {}", path.display());
                } else if update {
                    // Update documentation logic here
                    println!("Updated documentation file: {}", path.display());
                }
            }
        }

        Ok(())
    }

    async fn cleanup_workspace(&self, dry_run: bool) -> Result<()> {
        let patterns = [
            "target/debug",
            "target/release",
            "**/node_modules",
            "**/*.pyc",
            "**/dist",
            "**/build",
        ];

        for pattern in patterns {
            for entry in glob::glob(pattern)? {
                match entry {
                    Ok(path) => {
                        if dry_run {
                            println!("Would remove: {}", path.display());
                        } else {
                            if path.is_dir() {
                                fs::remove_dir_all(&path).await?;
                            } else {
                                fs::remove_file(&path).await?;
                            }
                            println!("Removed: {}", path.display());
                        }
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }
        }

        Ok(())
    }
}