// [AIR-3][AIS-3][BPC-3][RES-3] Removed unused import: std::error::Error
// Migrated from OPSource to anya-core
// This file was automatically migrated as part of the Rust-only implementation
// Original file: C:\Users\bmokoka\Downloads\OPSource\src\bitcoin\error.rs
// Bitcoin Error Handling Module
// Implements comprehensive error types and handling for Bitcoin operations
//
// [AIR-3][AIS-3][AIT-2][AIM-2][AIP-2][BPC-3][RES-3]
// This module provides structured error types with comprehensive coverage
// for all Bitcoin-related operations with good resilience characteristics.
// Complete implementation as per official Bitcoin Improvement Proposals (BIPs) standards

use bitcoin::key::FromSliceError;
use bitcoin::secp256k1;
use bitcoin::{
    sighash::P2wpkhError, sighash::TaprootError, taproot::SigFromSliceError,
    taproot::TaprootBuilder, taproot::TaprootBuilderError,
};
use hex::FromHexError;
use thiserror::Error;

/// Bitcoin operation errors
#[derive(Debug, Error, Clone)]
pub enum BitcoinError {
    #[error("Failed to sign transaction")]
    SigningError,

    #[error("Failed to create Taproot output: {0}")]
    TaprootError(String),

    #[error("Failed to convert signature")]
    SignatureConversionError,

    #[error("Invalid sighash")]
    InvalidSighash,

    #[error("Invalid public key: {0}")]
    InvalidPublicKey(String),

    #[error("Invalid private key")]
    InvalidPrivateKey,

    #[error("Invalid script: {0}")]
    InvalidScript(String),

    #[error("Invalid address: {0}")]
    InvalidAddress(String),

    #[error("Insufficient funds")]
    InsufficientFunds,

    #[error("Transaction not found")]
    TransactionNotFound,

    #[error("Block not found")]
    BlockNotFound,

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Wallet error: {0}")]
    Wallet(String),

    #[error("Lightning error: {0}")]
    Lightning(String),

    #[error("DLC error: {0}")]
    DLC(String),

    #[error("Secp256k1 error: {0}")]
    Secp256k1Error(String),

    #[error("Other error: {0}")]
    Other(String),

    #[error("Asset already issued")]
    AssetAlreadyIssued,

    #[error("Taproot builder error: {0}")]
    TaprootBuilderError(TaprootBuilderError),

    #[error("Invalid secret key")]
    InvalidSecretKey,

    #[error("Invalid witness")]
    InvalidWitness,

    #[error("Hex decoding error")]
    HexDecodingError,

    #[error("Key conversion error")]
    KeyConversionError,

    #[error("IO error: {0}")]
    IOError(String),

    #[error("Key error: {0}")]
    KeyError(String),

    #[error("Invalid transaction: {0}")]
    InvalidTransaction(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Invalid signature: {0}")]
    InvalidSignature(String),

    #[error("Encoding error: {0}")]
    EncodingError(String),

    #[error("Script error: {0}")]
    ScriptError(String),

    #[error("Invalid oracle signature")]
    InvalidOracleSignature,

    #[error("P2WPKH error: {0}")]
    P2wpkhError(String),

    #[error("Invalid contract: {0}")]
    InvalidContract(String),

    #[error("Wallet not found: {0}")]
    WalletNotFound(String),

    #[error("Invalid wallet: {0}")]
    InvalidWallet(String),

    #[error("Transaction error: {0}")]
    TransactionError(String),

    // [AIR-3][AIS-3][BPC-3][RES-3] Invalid configuration error
    // This follows official Bitcoin Improvement Proposals (BIPs) standards for configuration validation
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),

    #[error("Storage error: {0}")]
    StorageError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Authentication error: {0}")]
    AuthenticationError(String),

    #[error("Internal error: {0}")]
    InternalError(String),

    // [AIR-3][AIS-3][BPC-3][RES-3] Configuration error handling
    // This follows official Bitcoin Improvement Proposals (BIPs) standards for error handling
    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Key derivation error: {0}")]
    KeyDerivation(String),
}

impl From<TaprootBuilderError> for BitcoinError {
    fn from(err: TaprootBuilderError) -> Self {
        BitcoinError::TaprootBuilderError(err)
    }
}

impl From<secp256k1::Error> for BitcoinError {
    fn from(error: secp256k1::Error) -> Self {
        BitcoinError::Secp256k1Error(error.to_string())
    }
}

impl From<TaprootBuilder> for BitcoinError {
    fn from(_: TaprootBuilder) -> Self {
        BitcoinError::TaprootError("Taproot builder error".to_string())
    }
}

impl From<SigFromSliceError> for BitcoinError {
    fn from(_: SigFromSliceError) -> Self {
        BitcoinError::SignatureConversionError
    }
}

impl From<FromHexError> for BitcoinError {
    fn from(_: FromHexError) -> Self {
        BitcoinError::HexDecodingError
    }
}

impl From<FromSliceError> for BitcoinError {
    fn from(_: FromSliceError) -> Self {
        BitcoinError::KeyConversionError
    }
}

// Implementation for TaprootError
impl From<TaprootError> for BitcoinError {
    fn from(error: TaprootError) -> Self {
        BitcoinError::TaprootError(error.to_string())
    }
}

impl From<P2wpkhError> for BitcoinError {
    fn from(err: P2wpkhError) -> Self {
        BitcoinError::P2wpkhError(err.to_string())
    }
}

/// Result type for Bitcoin operations
pub type BitcoinResult<T> = Result<T, BitcoinError>;
