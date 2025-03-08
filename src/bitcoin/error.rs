use thiserror::Error;
use bitcoin::secp256k1;

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
}

/// Result type for Bitcoin operations
pub type BitcoinResult<T> = Result<T, BitcoinError>; 