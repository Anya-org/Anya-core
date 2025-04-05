use std::path::PathBuf;
use structopt::StructOpt;
use anyhow::{Result, Context};
use log::{info, warn, error};
use tokio;

mod commands;
use commands::*;

#[derive(StructOpt)]
#[structopt(name = "anya_tools", about = "Unified tooling for Anya Core")]
enum Cli {
    /// Testing related commands
    #[structopt(name = "test")]
    Test(TestCmd),

    /// Setup and configuration commands
    #[structopt(name = "setup")]
    Install(InstallCmd),

    /// Deployment related commands
    #[structopt(name = "deploy")]
    Deploy(DeployCmd),

    /// Contract management commands
    #[structopt(name = "contract")]
    Contract(ContractCmd),

    /// Git operations commands
    #[structopt(name = "git")]
    Git(GitCmd),

    /// Maintenance commands
    #[structopt(name = "maintenance")]
    Maintenance(MaintenanceCmd),

    /// Module configuration commands
    #[structopt(name = "module")]
    Module(ModuleCmd),

    /// Security analysis commands
    #[structopt(name = "security")]
    Security(SecurityCmd),
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::from_args();

    match cli {
        Cli::Test(cmd) => cmd.run().await,
        Cli::Install(cmd) => cmd.run().await,
        Cli::Deploy(cmd) => cmd.run().await,
        Cli::Contract(cmd) => cmd.run().await,
        Cli::Git(cmd) => cmd.run().await,
        Cli::Maintenance(cmd) => cmd.run().await,
        Cli::Module(cmd) => cmd.run().await,
        Cli::Security(cmd) => cmd.run().await,
    }
}