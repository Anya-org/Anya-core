//! Layer 2 Bitcoin Protocol Implementations
//!
//! This module contains implementations of various Layer 2 protocols
//! that can be used with Bitcoin.

pub mod rgb;

// Re-export commonly used items
pub use rgb::{
    AssetCreationParams, AssetTransfer, RGBAsset, RGBFactory, RGBManager, TransferStatus,
};
