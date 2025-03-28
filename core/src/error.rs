#![feature(edition2021)]
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("RPC connection error")]
    RpcConnectionError,

    #[error("HSM not available")]
    HsmNotAvailable,

    #[error("Transaction signing failed")]
    SigningFailed,

    #[error("Invalid Taproot commitment")]
    InvalidTaprootCommitment,

    #[error("Unknown error: {0}")]
    Unknown(String),
}
