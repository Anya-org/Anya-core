use thiserror::Error;
use bitcoin::secp256k1;
use bitcoin::{
    taproot::TaprootBuilderError,
    sighash::TaprootError,
    taproot::TaprootBuilder,
    taproot::SigFromSliceError,
};
use hex::FromHexError;
use bitcoin::key::FromSliceError;
use futures_io;

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

    #[error("Invalid address")]
    InvalidAddress,

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
    Secp256k1Error(#[from] secp256k1::Error),

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
        BitcoinError::TaprootBuilderError
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

impl From<futures_io::Error> for BitcoinError {
    fn from(err: futures_io::Error) -> Self {
        BitcoinError::IOError(err.to_string())
    }
}

/// Result type for Bitcoin operations
pub type BitcoinResult<T> = Result<T, BitcoinError>; 