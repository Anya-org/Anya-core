//! 
//! Protocol adapters for Anya Core
//! 
//! This crate provides protocol adapters for various blockchain protocols,
//! starting with Bitcoin. It implements conversions, validations, and utilities
//! needed to interact with blockchain protocols in a unified way.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

// Public modules
pub mod bitcoin;

// Re-exports
pub use bitcoin::BitcoinAdapter;
