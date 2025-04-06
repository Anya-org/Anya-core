//! # Anya Privacy Library
//! 
//! Privacy enhancements for Anya Core, including BIP-353 Silent Payments implementation.
//! 
//! [AIR-3][AIS-3][AIP-3][BPC-3]
//!
//! This crate provides a complete implementation of the BIP-353 Silent Payments protocol,
//! which offers enhanced transaction privacy in Bitcoin by allowing receivers to publish
//! a static payment address while preventing address reuse and avoiding deterministic links
//! between payments.
//!
//! ## Features
//!
//! - Silent Payment address generation
//! - Transaction scanning for incoming payments
//! - Sending to Silent Payment addresses
//! - Hardware security module integration
//! - Constant-time cryptographic operations
//!
//! ## Example: Creating a Silent Payment address
//!
//! ```rust
//! use anya_privacy::silent_payments::{SilentPaymentAddress, KeyManager};
//!
//! // Initialize a new key manager
//! let key_manager = KeyManager::new_random();
//!
//! // Generate a Silent Payment address
//! let address = key_manager.generate_address();
//! println!("Silent Payment address: {}", address.to_string());
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms, unreachable_pub)]

use bitcoin::hashes::Hash;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use thiserror::Error;

pub mod silent_payments;

/// Common error types for the privacy library
#[derive(Debug, Error)]
pub enum Error {
    /// Error related to cryptographic operations
    #[error("Cryptographic error: {0}")]
    Crypto(String),

    /// Error related to Bitcoin operations
    #[error("Bitcoin error: {0}")]
    Bitcoin(#[from] bitcoin::Error),

    /// Error related to secp256k1 operations
    #[error("Secp256k1 error: {0}")]
    Secp(#[from] bitcoin::secp256k1::Error),

    /// Error related to serialization
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Error when parsing an invalid address
    #[error("Invalid address: {0}")]
    InvalidAddress(String),

    /// Error when transaction scanning fails
    #[error("Scanning error: {0}")]
    ScanningError(String),

    /// Error when key management operations fail
    #[error("Key management error: {0}")]
    KeyManagement(String),
}

/// Result type for privacy operations
pub type Result<T> = std::result::Result<T, Error>; 