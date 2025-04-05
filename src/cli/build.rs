use anyhow::Result;
use clap::Parser;
use std::process::Command;

#[derive(Parser)]
pub struct BuildArgs {
    #[clap(long, help = "Build in release mode")]
    pub release: bool,
}

pub fn build(args: &BuildArgs) -> Result<()> {
    let mut cmd = Command::new("cargo");
    cmd.arg("build");
    
    if args.release {
        cmd.arg("--release");
    }

    cmd.status()?;
    Ok(())
}
