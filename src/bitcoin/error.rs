#![feature(edition2021)]
// Migrated from OPSource to anya-core
// This file was automatically migrated as part of the Rust-only implementation
// Original file: C:\Users\bmokoka\Downloads\OPSource\src\bitcoin\error.rs
// Bitcoin Error Handling Module
// Implements comprehensive error types and handling for Bitcoin operations
//
// [AIR-3][AIS-2][AIT-2][AIM-1][AIP-1][BPC-2][RES-2]
// This module provides structured error types with comprehensive coverage
// for all Bitcoin-related operations with good resilience characteristics.

use thiserror::Error;
use bitcoin::secp256k1;
use bitcoin::{
    taproot::TaprootBuilderError,
    sighash::TaprootError,
    sighash::P2wpkhError,
    taproot::TaprootBuilder,
    taproot::SigFromSliceError,
};
use hex::FromHexError;
use bitcoin::key::FromSliceError;

/// Bitcoin operation errors
#[derive(Error, Debug)]
pub enum BitcoinError {
    #[error("Failed to sign transaction")]
    SigningError,

    #[error("Failed to create Taproot output: {0}")]
    TaprootError(String),

    #[error("Failed to convert signature")]
    SignatureConversionError,

    #[error("Invalid sighash")]
    InvalidSighash,

    #[error("Invalid public key")]
    InvalidPublicKey,

    #[error("Invalid private key")]
    InvalidPrivateKey,

    #[error("Invalid script")]
    InvalidScript,

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

    #[error("Invalid transaction: {0}")]
    InvalidTransaction(String),

    #[error("Invalid signature: {0}")]
    InvalidSignature(String),

    #[error("Invalid oracle signature")]
    InvalidOracleSignature,

    #[error("Key error: {0}")]
    KeyError(String),

    #[error("P2WPKH error: {0}")]
    P2wpkhError(String),

    #[error("Invalid contract: {0}")]
    InvalidContract(String),
}

impl From<TaprootBuilderError> for BitcoinError {
    fn from(err: TaprootBuilderError) -> Self {
        BitcoinError::TaprootBuilderError(err)
    }
}

impl From<TaprootError> for BitcoinError {
    fn from(err: TaprootError) -> Self {
        BitcoinError::TaprootError(err.to_string())
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

impl From<secp256k1::Error> for BitcoinError {
    fn from(error: secp256k1::Error) -> Self {
        BitcoinError::Secp256k1Error(error.to_string())
    }
}

impl From<&str> for BitcoinError {
    fn from(error: &str) -> Self {
        BitcoinError::Other(error.to_string())
    }
}

impl From<P2wpkhError> for BitcoinError {
    fn from(err: P2wpkhError) -> Self {
        BitcoinError::P2wpkhError(err.to_string())
    }
}

/// Result type for Bitcoin operations
pub type BitcoinResult<T> = Result<T, BitcoinError>; 