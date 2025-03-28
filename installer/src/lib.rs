#![feature(edition2021)]
//! installer crate
//! [AIR-3][AIS-3][BPC-3][AIP-3][RES-3]

/// Placeholder module for installer
pub mod installer {
    /// Returns the version of this crate
    pub fn version() -> &'static str {
        "0.1.0"
    }
}

pub use installer::version;
