use std::path::PathBuf;
use structopt::StructOpt;
use anyhow::{Result, Context};
use tokio::process::Command;
use tokio::fs;
use regex::Regex;
use walkdir::WalkDir;

#[derive(StructOpt)]
pub enum SecurityCmd {
    /// Run CodeQL analysis (replaces run-codeql-analysis.ps1)
    #[structopt(name = "codeql")]
    CodeQL {
        #[structopt(long)]
        database: PathBuf,
        #[structopt(long)]
        queries: Vec<String>,
    },

    /// Reorganize code (replaces reorganize-code.ps1)
    #[structopt(name = "reorganize")]
    Reorganize {
        #[structopt(long)]
        src_dir: PathBuf,
        #[structopt(long)]
        dry_run: bool,
    },

    /// Run security audit
    #[structopt(name = "audit")]
    Audit {
        #[structopt(long)]
        report_file: Option<PathBuf>,
    },
}

impl SecurityCmd {
    pub async fn run(&self) -> Result<()> {
        match self {
            SecurityCmd::CodeQL { database, queries } => {
                self.run_codeql(database, queries).await
            }
            SecurityCmd::Reorganize { src_dir, dry_run } => {
                self.reorganize_code(src_dir, *dry_run).await
            }
            SecurityCmd::Audit { report_file } => {
                self.run_security_audit(report_file.as_deref()).await
            }
        }
    }

    async fn run_codeql(&self, database: &PathBuf, queries: &[String]) -> Result<()> {
        // Create CodeQL database
        Command::new("codeql")
            .args([
                "database", "create",
                database.to_str().unwrap(),
                "--language=rust",
                "--source-root=."
            ])
            .status()
            .await
            .context("Failed to create CodeQL database")?;

        // Run each query
        for query in queries {
            println!("Running query: {}", query);
            Command::new("codeql")
                .args([
                    "database", "analyze",
                    database.to_str().unwrap(),
                    query,
                    "--format=sarif-latest",
                    &format!("--output=results/{}.sarif", query.replace('/', "_"))
                ])
                .status()
                .await
                .context(format!("Failed to run query: {}", query))?;
        }

        // Generate summary report
        let mut summary = String::from("# CodeQL Analysis Summary\n\n");
        for entry in fs::read_dir("results").await? {
            let entry = entry?;
            if entry.path().extension().map_or(false, |ext| ext == "sarif") {
                let content = fs::read_to_string(entry.path()).await?;
                summary.push_str(&format!("## {}\n\n", entry.path().display()));
                summary.push_str(&extract_sarif_summary(&content));
                summary.push_str("\n\n");
            }
        }

        fs::write("results/summary.md", summary).await
            .context("Failed to write summary report")?;

        Ok(())
    }

    fn extract_sarif_summary(&self, sarif_content: &str) -> String {
        // Parse SARIF and extract relevant info
        // This is a simplified version - in practice you'd want to properly parse the JSON
        let mut summary = String::new();
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(sarif_content) {
            if let Some(runs) = json["runs"].as_array() {
                for run in runs {
                    if let Some(results) = run["results"].as_array() {
                        summary.push_str(&format!("Found {} issues\n", results.len()));
                        for result in results {
                            if let (Some(level), Some(message)) = (
                                result["level"].as_str(),
                                result["message"]["text"].as_str()
                            ) {
                                summary.push_str(&format!("- [{:^7}] {}\n", level, message));
                            }
                        }
                    }
                }
            }
        }
        summary
    }

    async fn reorganize_code(&self, src_dir: &PathBuf, dry_run: bool) -> Result<()> {
        // Define module patterns
        let module_patterns = [
            (Regex::new(r"bitcoin.*\.rs$")?, "bitcoin"),
            (Regex::new(r"dao.*\.rs$")?, "dao"),
            (Regex::new(r"web5.*\.rs$")?, "web5"),
            (Regex::new(r"dlc.*\.rs$")?, "dlc"),
        ];

        // Track files to move
        let mut moves = Vec::new();

        // Find files to reorganize
        for entry in WalkDir::new(src_dir).into_iter().filter_map(|e| e.ok()) {
            if !entry.file_type().is_file() {
                continue;
            }

            let path = entry.path();
            if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                for (pattern, module) in &module_patterns {
                    if pattern.is_match(filename) {
                        let new_path = PathBuf::from("src")
                            .join(module)
                            .join(path.strip_prefix(src_dir)?);
                        moves.push((path.to_path_buf(), new_path));
                        break;
                    }
                }
            }
        }

        // Print or execute moves
        for (src, dst) in moves {
            if dry_run {
                println!("Would move {} to {}", src.display(), dst.display());
            } else {
                if let Some(parent) = dst.parent() {
                    fs::create_dir_all(parent).await?;
                }
                fs::rename(&src, &dst).await
                    .context(format!("Failed to move {} to {}", src.display(), dst.display()))?;
                println!("Moved {} to {}", src.display(), dst.display());
            }
        }

        Ok(())
    }

    async fn run_security_audit(&self, report_file: Option<&PathBuf>) -> Result<()> {
        // Run cargo audit
        let output = Command::new("cargo")
            .args(["audit"])
            .output()
            .await
            .context("Failed to run cargo audit")?;

        let audit_report = String::from_utf8_lossy(&output.stdout).to_string();

        // Run additional security checks
        let mut report = String::from("# Security Audit Report\n\n");
        report.push_str("## Dependency Audit\n\n");
        report.push_str(&audit_report);
        report.push_str("\n\n## Additional Security Checks\n\n");

        // Check for common security issues
        let checks = [
            ("Unsafe code usage", r#"unsafe\s+fn|unsafe\s+\{"#),
            ("Unwrap usage", r#"unwrap\(\)"#),
            ("Expect usage", r#"expect\([^)]+\)"#),
            ("Hardcoded secrets", r#"password\s*=\s*["'][^"']+["']"#),
        ];

        for (check_name, pattern) in checks {
            report.push_str(&format!("### {}\n\n", check_name));
            let re = Regex::new(pattern)?;
            
            for entry in WalkDir::new("src").into_iter().filter_map(|e| e.ok()) {
                if !entry.file_type().is_file() || !entry.path().to_str().map_or(false, |s| s.ends_with(".rs")) {
                    continue;
                }

                let content = fs::read_to_string(entry.path()).await?;
                for (line_num, line) in content.lines().enumerate() {
                    if re.is_match(line) {
                        report.push_str(&format!(
                            "- {} (line {}): `{}`\n",
                            entry.path().display(),
                            line_num + 1,
                            line.trim()
                        ));
                    }
                }
            }
            report.push('\n');
        }

        // Write or print report
        match report_file {
            Some(path) => {
                fs::write(path, report).await
                    .context("Failed to write security report")?;
                println!("Security report written to: {}", path.display());
            }
            None => println!("{}", report),
        }

        Ok(())
    }
}