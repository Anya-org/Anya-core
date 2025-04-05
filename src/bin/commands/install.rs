use std::path::PathBuf;
use structopt::StructOpt;
use anyhow::{Result, Context};
use tokio::process::Command;
use tokio::fs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct InstallConfig {
    network_type: String,
    features: Vec<String>,
    directories: Vec<String>,
    dependencies: Vec<String>,
}

#[derive(StructOpt)]
pub enum InstallCmd {
    /// Configure installation (replaces configure.ps1)
    #[structopt(name = "configure")]
    Configure {
        #[structopt(long)]
        config: PathBuf,
        #[structopt(long)]
        minimal: bool,
    },

    /// Setup operating system (replaces windows_setup.ps1)
    #[structopt(name = "os-setup")]
    OsSetup {
        #[structopt(long)]
        os_type: String,
        #[structopt(long)]
        skip_optional: bool,
    },

    /// Setup dashboard (replaces dashboard.ps1)
    #[structopt(name = "dashboard")]
    Dashboard {
        #[structopt(long)]
        port: Option<u16>,
    },
}

impl InstallCmd {
    pub async fn run(&self) -> Result<()> {
        match self {
            InstallCmd::Configure { config, minimal } => {
                self.configure_installation(config, *minimal).await
            }
            InstallCmd::OsSetup { os_type, skip_optional } => {
                self.setup_os(os_type, *skip_optional).await
            }
            InstallCmd::Dashboard { port } => {
                self.setup_dashboard(*port).await
            }
        }
    }

    async fn configure_installation(&self, config_path: &PathBuf, minimal: bool) -> Result<()> {
        // Read and parse configuration
        let config_str = fs::read_to_string(config_path).await
            .context("Failed to read install config")?;
        let config: InstallConfig = toml::from_str(&config_str)
            .context("Failed to parse install config")?;

        // Create required directories
        for dir in &config.directories {
            fs::create_dir_all(dir).await
                .context(format!("Failed to create directory: {}", dir))?;
        }

        // Install dependencies
        if !minimal {
            for dep in &config.dependencies {
                Command::new("cargo")
                    .args(["install", dep])
                    .status()
                    .await
                    .context(format!("Failed to install dependency: {}", dep))?;
            }
        }

        // Configure network
        let network_config = format!(
            r#"network_type = "{}"
features = {:?}
"#,
            config.network_type, config.features
        );

        fs::write("config/network.toml", network_config).await
            .context("Failed to write network configuration")?;

        println!("Installation configured successfully!");
        Ok(())
    }

    async fn setup_os(&self, os_type: &str, skip_optional: bool) -> Result<()> {
        match os_type {
            "windows" => {
                // Install Windows-specific dependencies
                Command::new("choco")
                    .args(["install", "git", "rust", "python3", "nodejs", "-y"])
                    .status()
                    .await
                    .context("Failed to install core dependencies")?;

                if !skip_optional {
                    Command::new("choco")
                        .args(["install", "vscode", "docker-desktop", "-y"])
                        .status()
                        .await
                        .context("Failed to install optional dependencies")?;
                }
            }
            "linux" => {
                // Install Linux-specific dependencies
                Command::new("apt-get")
                    .args(["update"])
                    .status()
                    .await
                    .context("Failed to update package list")?;

                Command::new("apt-get")
                    .args([
                        "install",
                        "-y",
                        "build-essential",
                        "pkg-config",
                        "libssl-dev",
                        "git",
                        "python3",
                        "python3-pip",
                        "nodejs",
                        "npm"
                    ])
                    .status()
                    .await
                    .context("Failed to install dependencies")?;
            }
            _ => anyhow::bail!("Unsupported OS type: {}", os_type),
        }

        Ok(())
    }

    async fn setup_dashboard(&self, port: Option<u16>) -> Result<()> {
        // Install dashboard dependencies
        Command::new("npm")
            .args(["install", "--prefix", "dashboard"])
            .status()
            .await
            .context("Failed to install dashboard dependencies")?;

        // Configure dashboard port
        let config = format!(
            r#"{{
    "port": {},
    "api_endpoint": "http://localhost:3000",
    "log_level": "info"
}}"#,
            port.unwrap_or(8080)
        );

        fs::write("dashboard/config.json", config).await
            .context("Failed to write dashboard configuration")?;

        // Build dashboard
        Command::new("npm")
            .args(["run", "build", "--prefix", "dashboard"])
            .status()
            .await
            .context("Failed to build dashboard")?;

        println!("Dashboard setup completed successfully!");
        Ok(())
    }
}