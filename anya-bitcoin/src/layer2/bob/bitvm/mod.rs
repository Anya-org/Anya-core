// BitVM integration module for BOB
// Implements BitVM integration for Bitcoin Optimistic Blockchain
// as per official Bitcoin Improvement Proposals (BIPs) requirements

use crate::layer2::bob::{BitVMProof, BobConfig, BobError};
use reqwest::Client as HttpClient;
use std::time::Duration;
use tracing::{debug, info, warn};

/// Verification status for proofs
#[derive(Debug, Clone, PartialEq)]
pub enum VerificationStatus {
    /// Proof is valid
    Valid,
    /// Proof is invalid
    Invalid,
    /// Verification is pending
    Pending,
    /// Verification encountered an error
    Error(String),
}

/// BitVM validator for BOB
pub struct BitVMValidator {
    config: BobConfig,
    http_client: HttpClient,
    // Cache for verified proofs to avoid redundant verifications
    #[allow(dead_code)]
    verified_proofs: std::collections::HashMap<String, VerificationStatus>,
}

impl BitVMValidator {
    /// Create a new BitVM validator
    pub fn new(config: &BobConfig) -> Self {
        // Configure HTTP client with appropriate timeouts
        let http_client = HttpClient::builder()
            .timeout(Duration::from_millis(config.timeout_ms))
            .build()
            .expect("Failed to build HTTP client");
            
        Self {
            config: config.clone(),
            http_client,
            verified_proofs: std::collections::HashMap::new(),
        }
    }
    
    /// Check if the BitVM system is available
    pub async fn check_availability(&self) -> Result<bool, BobError> {
        let url = format!("{}/status", self.config.bitvm_url());
        debug!("Checking BitVM availability at {}", url);
        
        match self.http_client.get(&url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    debug!("BitVM system is available");
                    Ok(true)
                } else {
                    warn!("BitVM system returned non-success status: {}", response.status());
                    Ok(false)
                }
            },
            Err(e) => {
                warn!("Failed to connect to BitVM system: {}", e);
                Err(BobError::BitVMError(format!("Failed to connect to BitVM system: {}", e)))
            }
        }
    }

    /// Verify a BitVM proof
    pub async fn verify_proof(&self, proof: BitVMProof) -> Result<bool, BobError> {
        info!("Verifying BitVM proof {}", proof.id);
        
        // First check if we've already verified this proof
        if let Some(status) = self.verified_proofs.get(&proof.id) {
            match status {
                VerificationStatus::Valid => return Ok(true),
                VerificationStatus::Invalid => return Ok(false),
                VerificationStatus::Error(err) => {
                    return Err(BobError::BitVMError(format!("Previous verification error: {}", err)));
                },
                VerificationStatus::Pending => {
                    // Continue verification as status is still pending
                },
            }
        }
        
        // Prepare verification request
        let url = format!("{}/verify", self.config.bitvm_url());
        
        let payload = serde_json::json!({
            "proof_id": proof.id,
            "tx_hash": proof.tx_hash,
            "block_number": proof.block_number,
            "proof_data": hex::encode(&proof.proof_data),
        });
        
        debug!("Sending verification request to {}", url);
        
        // Send verification request
        let response = self.http_client
            .post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| BobError::BitVMError(format!("Failed to send verification request: {}", e)))?;
            
        // Process response
        if !response.status().is_success() {
            let error_text = response.text().await
                .unwrap_or_else(|_| "<failed to retrieve error text>".to_string());
                
            warn!("BitVM verification failed with status {}: {}", response.status(), error_text);
            return Err(BobError::BitVMError(format!("Verification API error: {}", error_text)));
        }
        
        // Parse response
        let result: serde_json::Value = response.json().await
            .map_err(|e| BobError::BitVMError(format!("Failed to parse verification response: {}", e)))?;
            
        // Extract verification result
        let is_valid = result.get("valid")
            .and_then(|v| v.as_bool())
            .ok_or_else(|| BobError::BitVMError("Missing 'valid' field in response".to_string()))?;
            
        info!("BitVM proof {} verification result: {}", proof.id, is_valid);
        
        Ok(is_valid)
    }
    
    /// Submit a BitVM proof for async verification
    pub async fn submit_proof(&self, proof: &BitVMProof) -> Result<String, BobError> {
        let url = format!("{}/submit", self.config.bitvm_url());
        
        let payload = serde_json::json!({
            "tx_hash": proof.tx_hash,
            "block_number": proof.block_number,
            "proof_data": hex::encode(&proof.proof_data),
        });
        
        // Submit proof for verification
        let response = self.http_client
            .post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| BobError::BitVMError(format!("Failed to submit proof: {}", e)))?;
            
        // Process response
        if !response.status().is_success() {
            let error_text = response.text().await
                .unwrap_or_else(|_| "<failed to retrieve error text>".to_string());
                
            return Err(BobError::BitVMError(format!("Submission API error: {}", error_text)));
        }
        
        // Parse response to get verification ID
        let result: serde_json::Value = response.json().await
            .map_err(|e| BobError::BitVMError(format!("Failed to parse submission response: {}", e)))?;
            
        // Extract verification ID
        let verification_id = result.get("verification_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BobError::BitVMError("Missing 'verification_id' field in response".to_string()))?;
            
        Ok(verification_id.to_string())
    }
    
    /// Check the status of an asynchronous verification
    pub async fn check_verification_status(&self, verification_id: &str) -> Result<VerificationStatus, BobError> {
        let url = format!("{}/verification/{}", self.config.bitvm_url(), verification_id);
        
        // Get verification status
        let response = self.http_client
            .get(&url)
            .send()
            .await
            .map_err(|e| BobError::BitVMError(format!("Failed to check verification status: {}", e)))?;
            
        // Process response
        if !response.status().is_success() {
            let error_text = response.text().await
                .unwrap_or_else(|_| "<failed to retrieve error text>".to_string());
                
            return Err(BobError::BitVMError(format!("Status API error: {}", error_text)));
        }
        
        // Parse response
        let result: serde_json::Value = response.json().await
            .map_err(|e| BobError::BitVMError(format!("Failed to parse status response: {}", e)))?;
            
        // Extract status
        let status = result.get("status")
            .and_then(|s| s.as_str())
            .ok_or_else(|| BobError::BitVMError("Missing 'status' field in response".to_string()))?;
            
        // Convert to enum
        let verification_status = match status {
            "valid" => VerificationStatus::Valid,
            "invalid" => VerificationStatus::Invalid,
            "pending" => VerificationStatus::Pending,
            "error" => {
                let error_msg = result.get("error")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown error")
                    .to_string();
                VerificationStatus::Error(error_msg)
            },
            _ => VerificationStatus::Error(format!("Unknown status: {}", status)),
        };
        
        Ok(verification_status)
    }
}
