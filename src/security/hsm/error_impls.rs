use crate::security::hsm::error::HsmError;
use serde_json::Error as JsonError;
use bitcoin::sighash::{PrevoutsIndexError, TaprootError};
use bitcoin::taproot::TaprootBuilderError;
use bitcoin::transaction::InputsIndexError;

// Don't implement From<std::io::Error> since it already exists

impl From<JsonError> for HsmError {
    fn from(err: JsonError) -> Self {
        HsmError::SerializationError(err.to_string())
    }
}

impl From<InputsIndexError> for HsmError {
    fn from(err: InputsIndexError) -> Self {
        HsmError::SigningError(format!("Bitcoin input index error: {:?}", err))
    }
}

impl From<PrevoutsIndexError> for HsmError {
    fn from(err: PrevoutsIndexError) -> Self {
        HsmError::SigningError(format!("Bitcoin prevout index error: {:?}", err))
    }
}

impl From<TaprootError> for HsmError {
    fn from(err: TaprootError) -> Self {
        HsmError::SigningError(format!("Bitcoin taproot error: {:?}", err))
    }
}

impl From<TaprootBuilderError> for HsmError {
    fn from(err: TaprootBuilderError) -> Self {
        HsmError::SigningError(format!("Bitcoin taproot builder error: {:?}", err))
    }
}

// Removed duplicate implementations for InputsIndexError, TaprootError, and TaprootBuilderError
