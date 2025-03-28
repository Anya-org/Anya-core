#![feature(edition2021)]
// [AIR-3][AIS-3]
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("RPC connection error")]
    RpcConnectionError,
    
    #[error("Invalid PSBT version (requires v2)")]
    InvalidPsbtVersion,
    
    #[error("HSM not available")]
    HsmNotAvailable,
    
    #[error("Unsupported HSM type")]
    UnsupportedHsmType,
    
    #[error("Transaction signing failed")]
    SigningFailed,
    
    #[error("Invalid Taproot commitment")]
    InvalidTaprootCommitment,
    
    #[error("Transaction broadcast failed")]
    BroadcastFailed,
    
    #[error("Missing UTXO amount")]
    MissingUtxoAmount,
    
    #[error("Invalid fee")]
    InvalidFee,
    
    #[error("Fee too low")]
    FeeTooLow,
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("Hex decoding error: {0}")]
    HexError(#[from] hex::FromHexError),
    
    #[error("Unknown error: {0}")]
    Unknown(String),
} 