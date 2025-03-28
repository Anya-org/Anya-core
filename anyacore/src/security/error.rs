#![feature(edition2021)]
#[derive(Debug, thiserror::Error)]
pub enum SecretError {
    #[error("HSM communication failure")]
    HsmConnection(#[from] hsm::Error),
    
    #[error("Invalid key path format")]
    KeyPathFormat,
    
    #[error("Cryptographic operation failed")]
    CryptoError(#[from] secp256k1::Error),
    
    #[error("Insufficient HSM approvals")]
    InsufficientApprovals,
    
    #[error("Key type not allowed")]
    DisallowedKeyType,
} 