use anyhow::Result;
use clap::Parser;
use std::process::Command;

#[derive(Parser)]
pub struct SetupArgs {
    #[clap(long, help = "Skip Rust installation")]
    pub skip_rust: bool,
}

pub fn setup_dev_env(args: &SetupArgs) -> Result<()> {
    // Install Rust if needed
    if !args.skip_rust {
        Command::new("curl")
            .args(&["--proto", "=https", "--tlsv1.2", "-sSf", 
                   "https://sh.rustup.rs", "-o", "rustup.sh"])
            .status()?;

        Command::new("sh")
            .args(&["rustup.sh", "-y"])
            .status()?;
    }

    // Install system dependencies
    if cfg!(target_os = "linux") {
        Command::new("apt-get")
            .args(&["install", "-y", "build-essential", "pkg-config", "libssl-dev"])
            .status()?;
    }

    Ok(())
}
