#![feature(edition2021)]
//! workspace crate
//! [AIR-3][AIS-3][BPC-3][AIP-3][RES-3]

/// Placeholder module for workspace
pub mod workspace {
    /// Returns the version of this crate
    pub fn version() -> &'static str {
        "0.1.0"
    }
}

pub use workspace::version;
