use std::path::PathBuf;
use structopt::StructOpt;
use anyhow::{Result, Context};
use tokio::process::Command;
use chrono::Local;
use regex::Regex;

#[derive(StructOpt)]
pub enum GitCmd {
    /// Create checkpoint (replaces create_checkpoint.ps1)
    #[structopt(name = "checkpoint")]
    Checkpoint {
        #[structopt(long)]
        name: String,
        #[structopt(long)]
        message: Option<String>,
        #[structopt(long)]
        ai_label: Option<String>,
        #[structopt(long)]
        push: bool,
    },

    /// Resolve git conflicts (replaces resolve_conflicts.ps1)
    #[structopt(name = "resolve")]
    Resolve {
        #[structopt(long)]
        auto_fix: bool,
    },

    /// Commit and push changes (replaces commit_push.ps1)
    #[structopt(name = "commit")]
    Commit {
        #[structopt(long)]
        message: String,
        #[structopt(long)]
        skip_checks: bool,
    },
}

impl GitCmd {
    pub async fn run(&self) -> Result<()> {
        match self {
            GitCmd::Checkpoint { name, message, ai_label, push } => {
                self.create_checkpoint(name, message.as_deref(), ai_label.as_deref(), *push).await
            }
            GitCmd::Resolve { auto_fix } => {
                self.resolve_conflicts(*auto_fix).await
            }
            GitCmd::Commit { message, skip_checks } => {
                self.commit_and_push(message, *skip_checks).await
            }
        }
    }

    async fn create_checkpoint(&self, name: &str, message: Option<&str>, ai_label: Option<&str>, push: bool) -> Result<()> {
        // Validate AI label if provided
        if let Some(label) = ai_label {
            let valid_labels = ["AIR", "AIS", "AIT", "AIM", "AIP", "AIE"];
            let label_pattern = format!(r"^({}))-\d{{3}}$", valid_labels.join("|"));
            let re = Regex::new(&label_pattern)?;
            if !re.is_match(label) {
                anyhow::bail!("Invalid AI label format. Must be one of {} followed by a 3-digit number", valid_labels.join(", "));
            }
        }

        // Create tag name
        let tag_name = format!("{}{}", 
            ai_label.map(|l| format!("{}-", l)).unwrap_or_default(),
            name.replace(|c: char| !c.is_alphanumeric() && c != '-' && c != '_', "_")
        );

        // Create tag with message
        let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S");
        let tag_message = format!("{} (Created at {})",
            message.unwrap_or("Automated checkpoint"),
            timestamp
        );

        Command::new("git")
            .args(["tag", "-a", &tag_name, "-m", &tag_message])
            .status()
            .await
            .context("Failed to create git tag")?;

        // Create checkpoint documentation
        let checkpoint_dir = PathBuf::from("docs/checkpoints");
        tokio::fs::create_dir_all(&checkpoint_dir).await?;
        
        let checkpoint_file = checkpoint_dir.join(format!("{}-{}.md", tag_name, timestamp));
        let content = self.generate_checkpoint_content(&tag_name, &tag_message, ai_label).await?;
        tokio::fs::write(&checkpoint_file, content).await?;

        // Push if requested
        if push {
            Command::new("git")
                .args(["push", "origin", &tag_name])
                .status()
                .await
                .context("Failed to push tag")?;

            Command::new("git")
                .args(["add", checkpoint_file.to_str().unwrap()])
                .status()
                .await
                .context("Failed to stage checkpoint file")?;

            Command::new("git")
                .args(["commit", "-m", &format!("Add checkpoint documentation for {}", tag_name)])
                .status()
                .await
                .context("Failed to commit checkpoint file")?;

            Command::new("git")
                .args(["push", "origin", "HEAD"])
                .status()
                .await
                .context("Failed to push checkpoint documentation")?;
        }

        Ok(())
    }

    async fn generate_checkpoint_content(&self, tag_name: &str, message: &str, ai_label: Option<&str>) -> Result<String> {
        let commit_info = Command::new("git")
            .args(["log", "-1", "--pretty=format:Commit: %H%nAuthor: %an <%ae>%nDate: %ad%n%n%s%n%n%b"])
            .output()
            .await?;

        let files_changed = Command::new("git")
            .args(["show", "--name-status", "HEAD"])
            .output()
            .await?;

        let repo_status = Command::new("git")
            .args(["status"])
            .output()
            .await?;

        Ok(format!(
            r#"# Checkpoint: {}
**Created**: {}
**AI Label**: {}
**Message**: {}

## Commit Information
{}

## Files Changed in Last Commit
{}

## Repository Status at Checkpoint
{}"#,
            tag_name,
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            ai_label.unwrap_or("None"),
            message,
            String::from_utf8_lossy(&commit_info.stdout),
            String::from_utf8_lossy(&files_changed.stdout),
            String::from_utf8_lossy(&repo_status.stdout),
        ))
    }

    async fn resolve_conflicts(&self, auto_fix: bool) -> Result<()> {
        // Get list of conflicted files
        let output = Command::new("git")
            .args(["diff", "--name-only", "--diff-filter=U"])
            .output()
            .await?;

        let conflicted_files = String::from_utf8_lossy(&output.stdout)
            .lines()
            .collect::<Vec<_>>();

        if conflicted_files.is_empty() {
            println!("No conflicts found");
            return Ok(());
        }

        println!("Found conflicts in {} files:", conflicted_files.len());
        for file in &conflicted_files {
            println!("  {}", file);
        }

        if auto_fix {
            // Try to automatically resolve using merge tools
            for file in conflicted_files {
                println!("Attempting to resolve conflicts in {}", file);
                
                // Try using merge tool
                Command::new("git")
                    .args(["mergetool", "--tool=vimdiff", file])
                    .status()
                    .await
                    .context(format!("Failed to resolve conflicts in {}", file))?;
            }
        } else {
            println!("\nTo resolve conflicts manually:");
            println!("1. Open each file and look for conflict markers (<<<<<<, =======, >>>>>>>)");
            println!("2. Edit the files to resolve conflicts");
            println!("3. Remove conflict markers");
            println!("4. Stage resolved files with: git add <file>");
            println!("5. Complete the merge with: git merge --continue");
        }

        Ok(())
    }

    async fn commit_and_push(&self, message: &str, skip_checks: bool) -> Result<()> {
        // Run pre-commit checks if not skipped
        if !skip_checks {
            // Run formatting check
            Command::new("cargo")
                .args(["fmt", "--all", "--check"])
                .status()
                .await
                .context("Code formatting check failed")?;

            // Run clippy
            Command::new("cargo")
                .args(["clippy", "--all-targets", "--", "-D", "warnings"])
                .status()
                .await
                .context("Clippy check failed")?;

            // Run tests
            Command::new("cargo")
                .args(["test"])
                .status()
                .await
                .context("Tests failed")?;
        }

        // Stage all changes
        Command::new("git")
            .args(["add", "."])
            .status()
            .await
            .context("Failed to stage changes")?;

        // Commit with message
        Command::new("git")
            .args(["commit", "-m", message])
            .status()
            .await
            .context("Failed to commit changes")?;

        // Push changes
        Command::new("git")
            .args(["push", "origin", "HEAD"])
            .status()
            .await
            .context("Failed to push changes")?;

        Ok(())
    }
}