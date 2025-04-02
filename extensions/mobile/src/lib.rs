// Mobile Extensions for Anya Core
//
// This module provides mobile-specific features for Anya Core.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

/// Mobile Edition version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize the mobile extensions
pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    println!("Initializing Anya Core Mobile Extensions v{}", VERSION);
    Ok(())
}
