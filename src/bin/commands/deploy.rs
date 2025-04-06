use std::path::PathBuf;
use structopt::StructOpt;
use anyhow::{Result, Context};
use tokio::process::Command;
use tokio::fs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ServerConfig {
    host: String,
    port: u16,
    user: String,
    key_path: Option<String>,
    deploy_path: String,
}

#[derive(StructOpt)]
pub enum DeployCmd {
    /// Deploy to server (replaces deploy_to_server.ps1)
    #[structopt(name = "server")]
    Server {
        #[structopt(long)]
        config: PathBuf,
        #[structopt(long)]
        dry_run: bool,
    },

    /// Push updates to server (replaces push_to_server.ps1)
    #[structopt(name = "push")]
    Push {
        #[structopt(long)]
        server: String,
        #[structopt(long)]
        force: bool,
    },
}

impl DeployCmd {
    pub async fn run(&self) -> Result<()> {
        match self {
            DeployCmd::Server { config, dry_run } => {
                self.deploy_to_server(config, *dry_run).await
            }
            DeployCmd::Push { server, force } => {
                self.push_to_server(server, *force).await
            }
        }
    }

    async fn deploy_to_server(&self, config_path: &PathBuf, dry_run: bool) -> Result<()> {
        // Read and parse server config
        let config_str = fs::read_to_string(config_path).await
            .context("Failed to read server config")?;
        let config: ServerConfig = toml::from_str(&config_str)
            .context("Failed to parse server config")?;

        // Build release
        println!("Building release...");
        Command::new("cargo")
            .args(["build", "--release"])
            .status()
            .await
            .context("Build failed")?;

        if dry_run {
            println!("Dry run - would deploy to {}:{}", config.host, config.port);
            return Ok(());
        }

        // Create deployment archive
        println!("Creating deployment archive...");
        Command::new("tar")
            .args([
                "czf", "deploy.tar.gz",
                "target/release/anya_core",
                "config",
                "resources"
            ])
            .status()
            .await
            .context("Failed to create deployment archive")?;

        // Upload to server
        println!("Uploading to server...");
        let ssh_args = if let Some(key_path) = config.key_path {
            vec!["-i", &key_path]
        } else {
            vec![]
        };

        Command::new("scp")
            .args([&ssh_args[..], &["deploy.tar.gz", &format!("{}@{}:{}", config.user, config.host, config.deploy_path)]])
            .status()
            .await
            .context("Failed to upload deployment archive")?;

        // Execute deployment script on server
        println!("Executing deployment on server...");
        Command::new("ssh")
            .args([
                &ssh_args[..],
                &[
                    &format!("{}@{}", config.user, config.host),
                    &format!(
                        "cd {} && tar xzf deploy.tar.gz && ./deploy.sh",
                        config.deploy_path
                    )
                ]
            ])
            .status()
            .await
            .context("Deployment execution failed")?;

        // Clean up local archive
        fs::remove_file("deploy.tar.gz").await
            .context("Failed to clean up deployment archive")?;

        println!("Deployment completed successfully!");
        Ok(())
    }

    async fn push_to_server(&self, server: &str, force: bool) -> Result<()> {
        // Validate server format (user@host)
        if !server.contains('@') {
            anyhow::bail!("Server must be in format user@host");
        }

        // Run tests before pushing
        if !force {
            println!("Running pre-push tests...");
            Command::new("cargo")
                .args(["test"])
                .status()
                .await
                .context("Tests failed")?;
        }

        // Get list of changes
        let output = Command::new("git")
            .args(["status", "--porcelain"])
            .output()
            .await?;

        if !output.stdout.is_empty() {
            println!("Warning: You have uncommitted changes:");
            println!("{}", String::from_utf8_lossy(&output.stdout));
            if !force {
                anyhow::bail!("Refusing to push with uncommitted changes. Use --force to override.");
            }
        }

        // Push to server
        println!("Pushing to server...");
        Command::new("rsync")
            .args([
                "-avz",
                "--delete",
                ".",
                &std::path::Path::new(server).join("anya-core").to_string_lossy()
            ])
            .status()
            .await
            .context("Failed to push to server")?;

        // Restart services if needed
        println!("Restarting services...");
        Command::new("ssh")
            .args([
                server,
                "systemctl restart anya-core"
            ])
            .status()
            .await
            .context("Failed to restart services")?;

        println!("Push completed successfully!");
        Ok(())
    }
}