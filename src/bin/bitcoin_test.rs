// use anya_core::bitcoin::protocol::testing::mock;
use anyhow::Result;
use std::env;
use std::error::Error;

fn main() -> Result<()> {
    // This binary is deprecated. All protocol tests are run via Rust #[cfg(test)] modules.
    // To run protocol tests, use: `cargo test --all`
    println!("[DEPRECATED] bitcoin_test binary is no longer used. Run `cargo test --all` for protocol tests.");
    Ok(())
}
