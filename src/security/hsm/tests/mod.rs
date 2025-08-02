//! HSM Testing Module
//! [AIR-3][AIS-3][BPC-3][RES-3]
//!
//! Comprehensive testing suite for HSM providers, factory, and fallback mechanisms.

pub mod integration;
pub mod testnet_provider_tests;

// Re-export test utilities for other modules
#[cfg(test)]
pub use integration::*;
