use std::error::Error;
use std::fmt;
use secp256k1;

// Define a simple placeholder HSM error type
#[derive(Debug)]
pub struct HsmError;

impl fmt::Display for HsmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HSM error")
    }
}

impl Error for HsmError {}

#[derive(Debug)]
pub enum SecretError {
    // HSM communication failure
    HsmConnection(HsmError),
    
    // Invalid key path format
    KeyPathFormat,
    
    // Cryptographic operation failed
    CryptoError(secp256k1::Error),
    
    // Insufficient HSM approvals
    InsufficientApprovals,
    
    // Key type not allowed
    DisallowedKeyType,
}

impl fmt::Display for SecretError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HsmConnection(_) => write!(f, "HSM communication failure"),
            Self::KeyPathFormat => write!(f, "Invalid key path format"),
            Self::CryptoError(_) => write!(f, "Cryptographic operation failed"),
            Self::InsufficientApprovals => write!(f, "Insufficient HSM approvals"),
            Self::DisallowedKeyType => write!(f, "Key type not allowed"),
        }
    }
}

impl Error for SecretError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::HsmConnection(err) => Some(err),
            Self::CryptoError(err) => Some(err),
            _ => None,
        }
    }
}

// From implementations for error conversions
impl From<HsmError> for SecretError {
    fn from(err: HsmError) -> Self {
        Self::HsmConnection(err)
    }
}

impl From<secp256k1::Error> for SecretError {
    fn from(err: secp256k1::Error) -> Self {
        Self::CryptoError(err)
    }
} 