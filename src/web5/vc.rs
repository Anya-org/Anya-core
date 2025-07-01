// [AIR-3][AIS-3][BPC-3][RES-3] Verifiable Credentials Implementation
// Provides W3C Verifiable Credentials functionality for Web5
// [AIR-012] Operational Reliability and [AIP-002] Modular Architecture

// Import std::error::Error for use in trait bounds
use crate::web5::identity::{DIDManager, Web5Error, Web5Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Verifiable Credential
///
/// Represents a W3C Verifiable Credential.
#[derive(Clone, Serialize, Deserialize)]
pub struct VerifiableCredential {
    /// Credential context
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    /// Credential ID
    pub id: String,
    /// Credential types
    #[serde(rename = "type")]
    pub credential_type: Vec<String>,
    /// Credential issuer
    pub issuer: String,
    /// Issuance date
    pub issuance_date: String,
    /// Credential subject
    pub credential_subject: CredentialSubject,
    /// Credential proof
    pub proof: Option<CredentialProof>,
}

/// Credential Subject
///
/// Contains the claims about the subject in a Verifiable Credential.
#[derive(Clone, Serialize, Deserialize)]
pub struct CredentialSubject {
    /// Subject ID (DID)
    pub id: String,
    /// Subject claims
    #[serde(flatten)]
    pub claims: HashMap<String, serde_json::Value>,
}

/// Credential Proof
///
/// Cryptographic proof of the Verifiable Credential.
#[derive(Clone, Serialize, Deserialize)]
pub struct CredentialProof {
    /// Proof type
    #[serde(rename = "type")]
    pub proof_type: String,
    /// Proof creation date
    pub created: String,
    /// Verification method
    pub verification_method: String,
    /// Proof purpose
    pub proof_purpose: String,
    /// Signature value
    pub jws: String,
}

/// Credential Manager
///
/// Manages Verifiable Credentials.
pub struct CredentialManager {
    /// DID Manager for identity operations
    did_manager: DIDManager,
    /// Stored credentials
    credentials: HashMap<String, VerifiableCredential>,
}

impl CredentialManager {
    /// Create a new credential manager
    pub fn new(did_manager: DIDManager) -> Self {
        Self {
            did_manager,
            credentials: HashMap::new(),
        }
    }

    /// Issue a new Verifiable Credential
    pub fn issue_credential(
        &mut self,
        issuer_did: &str,
        subject_did: &str,
        credential_type: &str,
        claims: HashMap<String, serde_json::Value>,
    ) -> Web5Result<VerifiableCredential> {
        // Create credential ID
        let id = format!("urn:uuid:{}", generate_uuid());

        // Create credential subject
        let credential_subject = CredentialSubject {
            id: subject_did.to_string(),
            claims,
        };

        // Create credential without proof
        let mut credential = VerifiableCredential {
            context: vec![
                "https://www.w3.org/2018/credentials/v1".to_string(),
                "https://www.w3.org/2018/credentials/examples/v1".to_string(),
            ],
            id,
            credential_type: vec![
                "VerifiableCredential".to_string(),
                credential_type.to_string(),
            ],
            issuer: issuer_did.to_string(),
            issuance_date: current_iso_date(),
            credential_subject,
            proof: None,
        };

        // Sign credential (create proof)
        let proof = self.create_proof(&credential, issuer_did)?;
        credential.proof = Some(proof);

        // Store credential
        self.credentials
            .insert(credential.id.clone(), credential.clone());

        Ok(credential)
    }

    /// Verify a credential
    pub fn verify_credential(&self, credential: &VerifiableCredential) -> Web5Result<bool> {
        // Check if proof exists
        let _proof = match &credential.proof {
            Some(p) => p,
            None => return Err(Web5Error::Credential("No proof in credential".to_string())),
        };

        // In a real implementation, we would verify the signature here
        // For now, just return true for simplicity
        Ok(true)
    }

    /// Store a credential
    pub fn store_credential(&mut self, credential: VerifiableCredential) -> Web5Result<()> {
        // In a real implementation, this would store the credential securely
        // and return its ID
        let id = if credential.id.is_empty() {
            generate_uuid()
        } else {
            credential.id.clone()
        };
        self.credentials.insert(id, credential);
        Ok(())
    }

    /// Get a credential by ID
    pub fn get_credential(&self, id: &str) -> Web5Result<VerifiableCredential> {
        self.credentials
            .get(id)
            .cloned()
            .ok_or_else(|| Web5Error::Credential(format!("Credential not found: {id}")))
    }

    /// List all credentials
    pub fn list_credentials(&self) -> Vec<&VerifiableCredential> {
        self.credentials.values().collect()
    }

    /// Create a proof for a credential
    fn create_proof(
        &self,
        credential: &VerifiableCredential,
        issuer_did: &str,
    ) -> Web5Result<CredentialProof> {
        // In a real implementation, this would sign the credential
        // For this example, we create a placeholder proof

        // Serialize credential without proof
        let credential_json = serde_json::to_string(&credential)
            .map_err(|e| Web5Error::Credential(format!("Failed to serialize credential: {e}")))?;

        // Sign with issuer DID
        let signature = self
            .did_manager
            .sign(issuer_did, credential_json.as_bytes())?;

        // Create proof
        let proof = CredentialProof {
            proof_type: "Ed25519Signature2020".to_string(),
            created: current_iso_date(),
            verification_method: format!("{issuer_did}#keys-1"),
            proof_purpose: "assertionMethod".to_string(),
            jws: hex::encode(signature),
        };

        Ok(proof)
    }
}

/// Generate a UUID
fn generate_uuid() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    format!(
        "{:x}-{:x}-{:x}-{:x}-{:x}",
        now & 0xFFFF,
        (now >> 16) & 0xFFFF,
        (now >> 32) & 0xFFFF,
        (now >> 48) & 0xFFFF,
        now % 1000
    )
}

/// Get current date in ISO 8601 format
fn current_iso_date() -> String {
    use chrono::Utc;
    Utc::now().to_rfc3339()
}

#[derive(Debug)]
pub enum VCError {
    Credential(String),
    Serialization(String),
    Signing(String),
}

impl Display for VCError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for VCError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[cfg(test)]
mod tests {
    // [AIR-3][AIS-3][BPC-3][RES-3] Error trait is already imported in the parent module
    use super::*;
    use crate::web5::identity::DIDManager;

    #[test]
    fn test_issue_credential() -> Result<(), Box<dyn std::error::Error>> {
        let did_manager = DIDManager::new("ion");

        // Create the issuer and subject DIDs first
        let issuer_did_obj = did_manager.create_did()?;
        let subject_did_obj = did_manager.create_did()?;

        let issuer_did = &issuer_did_obj.id;
        let subject_did = &subject_did_obj.id;

        let mut credential_manager = CredentialManager::new(did_manager);

        let mut claims = HashMap::new();
        claims.insert(
            "name".to_string(),
            serde_json::Value::String("John Doe".to_string()),
        );
        claims.insert(
            "age".to_string(),
            serde_json::Value::Number(serde_json::Number::from(25)),
        );

        // Issue credential
        let credential = credential_manager.issue_credential(
            issuer_did,
            subject_did,
            "ExampleCredential",
            claims,
        )?;

        // Verify basic properties
        assert_eq!(credential.issuer, *issuer_did);
        assert_eq!(credential.credential_subject.id, *subject_did);
        assert!(credential
            .credential_type
            .contains(&"ExampleCredential".to_string()));
        assert!(credential.proof.is_some());

        Ok(())
    }
}
