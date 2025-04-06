use std::path::PathBuf;
use structopt::StructOpt;
use anyhow::{Result, Context};
use tokio::process::Command;
use tokio::fs;

#[derive(StructOpt)]
pub enum SetupCmd {
    /// Development environment setup (replaces dev-setup.ps1)
    #[structopt(name = "dev")]
    Dev {
        #[structopt(long)]
        skip_dependencies: bool,
        #[structopt(long)]
        skip_tests: bool,
    },

    /// Install Anya Core (replaces Install-AnyaCore.ps1)
    #[structopt(name = "install")]
    Install {
        #[structopt(long, default_value = "testnet")]
        network: String,
        #[structopt(long)]
        no_validate: bool,
    },

    /// Install build tools (replaces install-build-tools.ps1)
    #[structopt(name = "tools")]
    Tools {
        #[structopt(long)]
        force: bool,
    },

    /// Download and install Clarinet (replaces download-clarinet.ps1)
    #[structopt(name = "clarinet")]
    Clarinet {
        #[structopt(long)]
        version: Option<String>,
    },
}

impl SetupCmd {
    pub async fn run(&self) -> Result<()> {
        match self {
            SetupCmd::Dev { skip_dependencies, skip_tests } => {
                self.setup_dev_environment(*skip_dependencies, *skip_tests).await
            }
            SetupCmd::Install { network, no_validate } => {
                self.install_core(network, *no_validate).await
            }
            SetupCmd::Tools { force } => {
                self.install_build_tools(*force).await
            }
            SetupCmd::Clarinet { version } => {
                self.install_clarinet(version.as_deref()).await
            }
        }
    }

    async fn setup_dev_environment(&self, skip_dependencies: bool, skip_tests: bool) -> Result<()> {
        // Install dependencies if not skipped
        if !skip_dependencies {
            // Update Rust
            Command::new("rustup")
                .args(["update", "stable"])
                .status()
                .await
                .context("Failed to update Rust")?;

            // Install required components
            Command::new("rustup")
                .args(["component", "add", "rustfmt", "clippy"])
                .status()
                .await
                .context("Failed to install Rust components")?;

            // Install Node.js dependencies
            Command::new("npm")
                .args(["install", "-g", "yarn"])
                .status()
                .await
                .context("Failed to install yarn")?;

            // Install Python dependencies
            Command::new("pip")
                .args(["install", "-r", "requirements.txt"])
                .status()
                .await
                .context("Failed to install Python dependencies")?;
        }

        // Set up directory structure
        let directories = [
            "src/core",
            "src/mobile",
            "src/bitcoin",
            "packages/dash33",
            "docs/api",
            "docs/research"
        ];

        for dir in directories {
            fs::create_dir_all(dir).await
                .context(format!("Failed to create directory: {}", dir))?;
        }

        // Set up git hooks
        fs::create_dir_all(".git/hooks").await?;
        Command::new("cp")
            .args(["-r", "scripts/hooks/.", ".git/hooks/"])
            .status()
            .await
            .context("Failed to copy git hooks")?;

        // Run tests if not skipped
        if !skip_tests {
            Command::new("cargo")
                .args(["test", "--workspace"])
                .status()
                .await
                .context("Workspace tests failed")?;

            Command::new("sh")
                .args(["-c", "cd packages/dash33 && yarn test"])
                .status()
                .await
                .context("Dash33 tests failed")?;
        }

        Ok(())
    }

    async fn install_core(&self, network: &str, no_validate: bool) -> Result<()> {
        // Validate network type
        if !["mainnet", "testnet", "regtest"].contains(&network) {
            anyhow::bail!("Invalid network type. Must be mainnet, testnet, or regtest");
        }

        // Build the project
        Command::new("cargo")
            .args(["build", "--release"])
            .status()
            .await
            .context("Failed to build project")?;

        // Create configuration
        let config_dir = PathBuf::from("config");
        fs::create_dir_all(&config_dir).await?;

        let config_content = format!(
            r#"# Anya Core Configuration
[network]
network_type = "{}"
connect_peers = ["127.0.0.1:18333", "127.0.0.1:18334"]

[wallet]
enable_taproot = true
bip370_support = true

[system]
log_level = "info"
"#,
            network
        );

        fs::write(config_dir.join("anya.conf"), config_content).await
            .context("Failed to write configuration file")?;

        // Validate installation if requested
        if !no_validate {
            Command::new("cargo")
                .args(["run", "--bin", "anya_core", "--", "--validate"])
                .status()
                .await
                .context("Installation validation failed")?;
        }

        Ok(())
    }

    async fn install_build_tools(&self, force: bool) -> Result<()> {
        let tools = [
            ("rustfmt", vec!["rustup", "component", "add", "rustfmt"]),
            ("clippy", vec!["rustup", "component", "add", "clippy"]),
            ("cargo-audit", vec!["cargo", "install", "cargo-audit"]),
            ("cargo-watch", vec!["cargo", "install", "cargo-watch"]),
        ];

        for (tool, install_cmd) in tools {
            // Check if tool exists and force flag is not set
            if !force && Command::new("which")
                .arg(tool)
                .status()
                .await
                .is_ok() 
            {
                continue;
            }

            // Install tool
            Command::new(&install_cmd[0])
                .args(&install_cmd[1..])
                .status()
                .await
                .context(format!("Failed to install {}", tool))?;
        }

        Ok(())
    }

    async fn install_clarinet(&self, version: Option<&str>) -> Result<()> {
        let release_url = match version {
            Some(v) => std::path::Path::new(v).join("clarinet-linux-x64").to_string_lossy(),
            None => "https://github.com/hirosystems/clarinet/releases/latest/download/clarinet-linux-x64".to_string(),
        };

        // Create installation directory
        let install_dir = PathBuf::from(std::path::Path::new("/").join("usr/local/bin").to_string_lossy());
        fs::create_dir_all(&install_dir).await?;

        // Download Clarinet
        Command::new("curl")
            .args(["-L", &release_url, "-o", "clarinet"])
            .status()
            .await
            .context("Failed to download Clarinet")?;

        // Make executable and move to installation directory
        Command::new("chmod")
            .args(["+x", "clarinet"])
            .status()
            .await
            .context("Failed to make Clarinet executable")?;

        Command::new("mv")
            .args(["clarinet", std::path::Path::new("/").join("usr/local/bin/").to_string_lossy()])
            .status()
            .await
            .context("Failed to install Clarinet")?;

        // Verify installation
        let output = Command::new("clarinet")
            .arg("--version")
            .output()
            .await
            .context("Failed to verify Clarinet installation")?;

        if !output.status.success() {
            anyhow::bail!("Clarinet installation verification failed");
        }

        Ok(())
    }
}