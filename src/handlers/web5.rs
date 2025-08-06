// Web5 Protocol Handler Implementation
// Author: Bo_theBig

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Web5 DID (Decentralized Identity) structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Web5Did {
    pub did: String,
    pub document: DidDocument,
    pub keys: Vec<DidKey>,
}

/// DID Document structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DidDocument {
    pub id: String,
    pub context: Vec<String>,
    pub verification_method: Vec<VerificationMethod>,
    pub authentication: Vec<String>,
    pub assertion_method: Vec<String>,
    pub key_agreement: Vec<String>,
    pub capability_invocation: Vec<String>,
    pub capability_delegation: Vec<String>,
    pub service: Vec<ServiceEndpoint>,
}

/// Verification Method structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationMethod {
    pub id: String,
    pub r#type: String,
    pub controller: String,
    pub public_key_jwk: Option<serde_json::Value>,
    pub public_key_multibase: Option<String>,
}

/// Service Endpoint structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    pub id: String,
    pub r#type: String,
    pub service_endpoint: String,
}

/// DID Key structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DidKey {
    pub key_id: String,
    pub key_type: String,
    pub public_key: Vec<u8>,
    pub private_key: Option<Vec<u8>>,
}

/// Web5 Verifiable Credential
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiableCredential {
    pub context: Vec<String>,
    pub id: String,
    pub r#type: Vec<String>,
    pub issuer: String,
    pub issuance_date: String,
    pub expiration_date: Option<String>,
    pub credential_subject: serde_json::Value,
    pub proof: Option<serde_json::Value>,
}

/// Web5 Verifiable Presentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiablePresentation {
    pub context: Vec<String>,
    pub id: String,
    pub r#type: Vec<String>,
    pub holder: String,
    pub verifiable_credential: Vec<VerifiableCredential>,
    pub proof: Option<serde_json::Value>,
}

/// Web5 Handler for decentralized identity management
#[derive(Debug)]
pub struct Web5Handler {
    dids: RwLock<HashMap<String, Web5Did>>,
    credentials: RwLock<HashMap<String, VerifiableCredential>>,
    presentations: RwLock<HashMap<String, VerifiablePresentation>>,
}

impl Default for Web5Handler {
    fn default() -> Self {
        Self::new()
    }
}

impl Web5Handler {
    /// Create new Web5 handler
    pub fn new() -> Self {
        Self {
            dids: RwLock::new(HashMap::new()),
            credentials: RwLock::new(HashMap::new()),
            presentations: RwLock::new(HashMap::new()),
        }
    }

    /// Create new DID
    pub async fn create_did(&self, method: &str) -> Result<String, String> {
        let did_id = format!("did:{}:{}", method, Uuid::new_v4());

        let document = DidDocument {
            id: did_id.clone(),
            context: vec!["https://www.w3.org/ns/did/v1".to_string()],
            verification_method: Vec::new(),
            authentication: Vec::new(),
            assertion_method: Vec::new(),
            key_agreement: Vec::new(),
            capability_invocation: Vec::new(),
            capability_delegation: Vec::new(),
            service: Vec::new(),
        };

        let did = Web5Did {
            did: did_id.clone(),
            document,
            keys: Vec::new(),
        };

        let mut dids = self.dids.write().await;
        dids.insert(did_id.clone(), did);
        Ok(did_id)
    }

    /// Get DID by ID
    pub async fn get_did(&self, did_id: &str) -> Result<Option<Web5Did>, String> {
        let dids = self.dids.read().await;
        Ok(dids.get(did_id).cloned())
    }

    /// Create verifiable credential
    pub async fn create_credential(
        &self,
        issuer: String,
        subject: serde_json::Value,
        credential_type: Vec<String>,
    ) -> Result<String, String> {
        let credential_id = format!("urn:uuid:{}", Uuid::new_v4());

        let credential = VerifiableCredential {
            context: vec!["https://www.w3.org/2018/credentials/v1".to_string()],
            id: credential_id.clone(),
            r#type: credential_type,
            issuer,
            issuance_date: chrono::Utc::now().to_rfc3339(),
            expiration_date: None,
            credential_subject: subject,
            proof: None,
        };

        let mut credentials = self.credentials.write().await;
        credentials.insert(credential_id.clone(), credential);
        Ok(credential_id)
    }

    /// Get credential by ID
    pub async fn get_credential(
        &self,
        credential_id: &str,
    ) -> Result<Option<VerifiableCredential>, String> {
        let credentials = self.credentials.read().await;
        Ok(credentials.get(credential_id).cloned())
    }

    /// Create verifiable presentation
    pub async fn create_presentation(
        &self,
        holder: String,
        credential_ids: Vec<String>,
    ) -> Result<String, String> {
        let presentation_id = format!("urn:uuid:{}", Uuid::new_v4());

        let mut verifiable_credentials = Vec::new();
        let credentials = self.credentials.read().await;

        for credential_id in credential_ids {
            if let Some(credential) = credentials.get(&credential_id) {
                verifiable_credentials.push(credential.clone());
            } else {
                return Err(format!("Credential {} not found", credential_id));
            }
        }

        let presentation = VerifiablePresentation {
            context: vec!["https://www.w3.org/2018/credentials/v1".to_string()],
            id: presentation_id.clone(),
            r#type: vec!["VerifiablePresentation".to_string()],
            holder,
            verifiable_credential: verifiable_credentials,
            proof: None,
        };

        let mut presentations = self.presentations.write().await;
        presentations.insert(presentation_id.clone(), presentation);
        Ok(presentation_id)
    }

    /// Get presentation by ID
    pub async fn get_presentation(
        &self,
        presentation_id: &str,
    ) -> Result<Option<VerifiablePresentation>, String> {
        let presentations = self.presentations.read().await;
        Ok(presentations.get(presentation_id).cloned())
    }

    /// Verify credential signature (placeholder implementation)
    pub async fn verify_credential(&self, credential_id: &str) -> Result<bool, String> {
        let credentials = self.credentials.read().await;
        match credentials.get(credential_id) {
            Some(_credential) => {
                // Placeholder: In a real implementation, this would verify the cryptographic proof
                Ok(true)
            }
            None => Err("Credential not found".to_string()),
        }
    }

    /// Verify presentation (placeholder implementation)
    pub async fn verify_presentation(&self, presentation_id: &str) -> Result<bool, String> {
        let presentations = self.presentations.read().await;
        match presentations.get(presentation_id) {
            Some(presentation) => {
                // Placeholder: In a real implementation, this would verify all credentials in the presentation
                let credentials = self.credentials.read().await;
                for credential in &presentation.verifiable_credential {
                    if !credentials.contains_key(&credential.id) {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            None => Err("Presentation not found".to_string()),
        }
    }
}

// HTTP API endpoints for Web5 functionality
#[cfg(feature = "api")]
pub mod api {
    use super::*;
    use axum::{
        extract::{Json, Path},
        http::StatusCode,
        response::Json as ResponseJson,
    };
    use serde_json::Value;
    use std::sync::Arc;

    /// Create new DID
    pub async fn create_did(
        handler: Arc<Web5Handler>,
        Json(request): Json<Value>,
    ) -> Result<ResponseJson<Value>, StatusCode> {
        let method = request["method"].as_str().unwrap_or("web");

        match handler.create_did(method).await {
            Ok(did_id) => Ok(ResponseJson(serde_json::json!({
                "did": did_id
            }))),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    /// Get DID document
    pub async fn get_did(
        handler: Arc<Web5Handler>,
        Path(did_id): Path<String>,
    ) -> Result<ResponseJson<Value>, StatusCode> {
        match handler.get_did(&did_id).await {
            Ok(Some(did)) => Ok(ResponseJson(serde_json::to_value(did).unwrap())),
            Ok(None) => Err(StatusCode::NOT_FOUND),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    /// Create verifiable credential
    pub async fn create_credential(
        handler: Arc<Web5Handler>,
        Json(request): Json<Value>,
    ) -> Result<ResponseJson<Value>, StatusCode> {
        let issuer = request["issuer"].as_str().ok_or(StatusCode::BAD_REQUEST)?;
        let subject = request["credentialSubject"].clone();
        let credential_type = request["type"]
            .as_array()
            .ok_or(StatusCode::BAD_REQUEST)?
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();

        match handler
            .create_credential(issuer.to_string(), subject, credential_type)
            .await
        {
            Ok(credential_id) => Ok(ResponseJson(serde_json::json!({
                "id": credential_id
            }))),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    /// Get verifiable credential
    pub async fn get_credential(
        handler: Arc<Web5Handler>,
        Path(credential_id): Path<String>,
    ) -> Result<ResponseJson<Value>, StatusCode> {
        match handler.get_credential(&credential_id).await {
            Ok(Some(credential)) => Ok(ResponseJson(serde_json::to_value(credential).unwrap())),
            Ok(None) => Err(StatusCode::NOT_FOUND),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_did_creation() {
        let handler = Web5Handler::new();
        let did_id = handler.create_did("web").await.unwrap();

        assert!(did_id.starts_with("did:web:"));

        let did = handler.get_did(&did_id).await.unwrap();
        assert!(did.is_some());
        let did = did.unwrap();
        assert_eq!(did.did, did_id);
    }

    #[tokio::test]
    async fn test_credential_creation() {
        let handler = Web5Handler::new();
        let issuer = "did:web:example.com".to_string();
        let subject = serde_json::json!({
            "id": "did:web:alice.example.com",
            "name": "Alice Smith"
        });
        let credential_type = vec![
            "VerifiableCredential".to_string(),
            "PersonCredential".to_string(),
        ];

        let credential_id = handler
            .create_credential(issuer, subject, credential_type)
            .await
            .unwrap();

        let credential = handler.get_credential(&credential_id).await.unwrap();
        assert!(credential.is_some());
        let credential = credential.unwrap();
        assert_eq!(credential.id, credential_id);
        assert!(credential.r#type.contains(&"PersonCredential".to_string()));
    }

    #[tokio::test]
    async fn test_presentation_creation() {
        let handler = Web5Handler::new();

        // First create a credential
        let issuer = "did:web:issuer.example.com".to_string();
        let subject = serde_json::json!({"id": "did:web:subject.example.com"});
        let credential_type = vec!["VerifiableCredential".to_string()];
        let credential_id = handler
            .create_credential(issuer, subject, credential_type)
            .await
            .unwrap();

        // Then create a presentation
        let holder = "did:web:holder.example.com".to_string();
        let presentation_id = handler
            .create_presentation(holder, vec![credential_id])
            .await
            .unwrap();

        let presentation = handler.get_presentation(&presentation_id).await.unwrap();
        assert!(presentation.is_some());
        let presentation = presentation.unwrap();
        assert_eq!(presentation.id, presentation_id);
        assert_eq!(presentation.verifiable_credential.len(), 1);
    }
}
