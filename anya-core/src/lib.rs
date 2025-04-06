// Anya Core - Bitcoin Development Framework
// Main library entry point

// Module declarations
pub mod bdf_compliance;
pub mod bitcoin_internal {
    pub mod psbt;
}
pub mod hsm;
pub mod privacy;
pub mod psbt;
pub mod research;
pub mod security {
    pub mod audit;
    pub mod config;
    pub mod enforcement;
    pub mod error;
    pub mod hsm;
    pub mod secrets;
}
pub mod tapscript;

// Re-exports for convenient API access
pub use bitcoin;
// Expose our internal bitcoin module as bitcoin_core to avoid conflicts
pub use bitcoin_internal as bitcoin_core;
// Import anyhow for error handling
use anyhow::{Result, Error as AnyhowError};

// Library version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");

/// Initialize the anyacore library with default settings
pub fn init() -> Result<()> {
    // No verification needed for now
    // security::enforcement::verify_environment exists elsewhere
    Ok(())
}
