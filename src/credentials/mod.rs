use anyhow::Result;
use bitcoin::Address;
use serde::{Serialize, Deserialize};
use time::{Duration, OffsetDateTime}; 
use std::env;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CredentialError {
    #[error("Invalid BTC address: {0}")]
    InvalidBtcAddress(String),
    #[error("Missing environment variable: {0}")]
    MissingEnvVar(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CredentialStatus {
    Active,
    Expired,
    Revoked,
    Suspended
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credential {
    pub id: String,
    pub cred_type: CredentialType,
    pub issuer: String,
    pub subject: String,
    pub status: CredentialStatus,
    pub issued_at: OffsetDateTime,
    pub expires_at: Option<OffsetDateTime>,
    pub proof: CredentialProof,
    pub metadata: CredentialMetadata
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CredentialType {
    Wallet(WalletCredential),
    Web5(Web5Credential),
    Lightning(LightningCredential),
    Development(DevelopmentCredential)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialProof {
    pub verification_method: String,
    pub signature: Vec<u8>,
    pub nonce: Vec<u8>,
    pub created: OffsetDateTime
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialMetadata {
    pub schema_version: String,
    pub revocation_id: Option<String>,
    pub tags: Vec<String>,
    pub anchors: Vec<Anchor>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Anchor {
    Bitcoin {
        txid: String,
        block_height: u32,
        confirmations: u32
    },
    Web5 {
        did: String,
        registry: String
    }
}

pub trait CredentialVerifier {
    fn verify(&self, cred: &Credential) -> Result<bool>;
    fn revoke(&self, cred: &mut Credential) -> Result<()>;
    fn suspend(&self, cred: &mut Credential) -> Result<()>;
    fn reinstate(&self, cred: &mut Credential) -> Result<()>;
}

pub struct Credentials {
    btc_address: Address,
}

impl Credentials {
    pub fn new() -> Result<Self, CredentialError> {
        let btc_addr = env::var("ANYA_BTC_ADDRESS")
            .map_err(|_| CredentialError::MissingEnvVar("ANYA_BTC_ADDRESS".to_string()))?;
            
        let address = Address::from_str(&btc_addr)
            .map_err(|_| CredentialError::InvalidBtcAddress(btc_addr.clone()))?;

        Ok(Self {
            btc_address: address
        })
    }
    
    pub fn get_btc_address(&self) -> &Address {
        &self.btc_address
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_credentials() {
        std::env::set_var("ANYA_BTC_ADDRESS", 
            "bc1p2ej8neqcl40tswxt0q0gsm8hhzqkp6fuvj006zf75ld70030m2lq4apejj");
        
        let creds = Credentials::new().unwrap();
        assert_eq!(creds.get_btc_address().to_string(),
            "bc1p2ej8neqcl40tswxt0q0gsm8hhzqkp6fuvj006zf75ld70030m2lq4apejj");
    }
}

pub mod verifier;
pub mod wallet;
pub mod web5;
pub mod lightning;
pub mod development;
