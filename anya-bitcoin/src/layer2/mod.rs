//! Layer 2 implementations
//!
//! This module contains implementations of various Layer 2 protocols
//! for Bitcoin, following the hexagonal architecture pattern.

// Re-export modules
pub mod types;
pub mod framework;
pub mod bob;
pub mod lightning;
pub mod rgb;
pub mod rsk;
pub mod dlc;
pub mod taproot_assets;
pub mod traits;
