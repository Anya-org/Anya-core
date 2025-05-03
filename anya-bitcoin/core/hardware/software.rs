//! Software fallback implementation for hardware acceleration
//!
//! This module provides software implementations of cryptographic operations
//! for environments without specialized hardware acceleration, ensuring that
//! all functionality works consistently across all platforms while maintaining
//! full alignment with Bitcoin Core principles.

use super::{HardwareAccelerator, HardwareError, HardwareType, HardwareMetrics};
use bitcoin::secp256k1::{Secp256k1, XOnlyPublicKey, Message, Verification};
use bitcoin::hashes::{sha256, Hash};
use bitcoin::secp256k1::schnorr::Signature as SchnorrSignature;
use std::time::{Instant, Duration};
use std::sync::{Arc, Mutex};

/// Software fallback accelerator for cryptographic operations
pub struct SoftwareAccelerator {
    /// Secp256k1 context for signature verification
    secp: Secp256k1<Verification>,
    /// Performance metrics
    metrics: Arc<Mutex<HardwareMetrics>>,
    /// Initialization status
    initialized: bool,
}

impl SoftwareAccelerator {
    /// Create a new software accelerator
    pub fn new() -> Self {
        Self {
            secp: Secp256k1::verification_only(),
            metrics: Arc::new(Mutex::new(HardwareMetrics {
                hardware_type: HardwareType::Software,
                ops_per_second: 0,
                avg_latency_us: 0,
                memory_usage: 0,
                batch_size: 1,
            })),
            initialized: false,
        }
    }
    
    /// Update performance metrics based on operation
    fn update_metrics(&self, operation: &str, batch_size: usize, duration: Duration) {
        if let Ok(mut metrics) = self.metrics.lock() {
            let micros = duration.as_micros() as u64;
            let ops_per_second = if micros > 0 {
                (batch_size as u64) * 1_000_000 / micros
            } else {
                0
            };
            
            metrics.ops_per_second = ops_per_second;
            metrics.avg_latency_us = if batch_size > 0 {
                micros / (batch_size as u64)
            } else {
                0
            };
            
            // Log performance data
            log::debug!(
                "Software {} performance: {} ops/sec, {} µs/op, batch_size={}",
                operation,
                ops_per_second,
                metrics.avg_latency_us,
                batch_size
            );
        }
    }
}

impl HardwareAccelerator for SoftwareAccelerator {
    fn initialize(&mut self) -> Result<(), HardwareError> {
        // Log initialization
        log::info!("Initializing software fallback accelerator");
        
        self.initialized = true;
        Ok(())
    }
    
    fn is_available(&self) -> bool {
        // Software fallback is always available
        true
    }
    
    fn verify_schnorr_signature(
        &self,
        message: &[u8],
        signature: &SchnorrSignature,
        public_key: &XOnlyPublicKey,
    ) -> Result<bool, HardwareError> {
        let start = Instant::now();
        
        // Create message hash
        let msg_hash = sha256::Hash::hash(message);
        let msg = Message::from_digest(msg_hash.to_byte_array());
        
        // Verify signature
        let result = self.secp.verify_schnorr(signature, &msg, public_key).is_ok();
        
        // Update metrics
        self.update_metrics("SchnorrSingle", 1, start.elapsed());
        
        Ok(result)
    }
    
    fn verify_schnorr_batch(
        &self,
        messages: &[&[u8]],
        signatures: &[SchnorrSignature],
        public_keys: &[XOnlyPublicKey],
    ) -> Result<bool, HardwareError> {
        let start = Instant::now();
        
        // Verify input lengths match
        if messages.len() != signatures.len() || messages.len() != public_keys.len() {
            return Err(HardwareError::ValidationError(
                "Mismatched array lengths for batch verification".into()
            ));
        }
        
        // Create message objects
        let mut msg_objects = Vec::with_capacity(messages.len());
        for msg in messages {
            let hash = sha256::Hash::hash(msg);
            msg_objects.push(Message::from_digest(hash.to_byte_array()));
        }
        
        // Verify each signature individually
        for ((msg, sig), key) in msg_objects.iter().zip(signatures).zip(public_keys) {
            if !self.secp.verify_schnorr(sig, msg, key).is_ok() {
                // Update metrics before returning
                self.update_metrics("SchnorrBatch", messages.len(), start.elapsed());
                return Ok(false);
            }
        }
        
        // Update metrics
        self.update_metrics("SchnorrBatch", messages.len(), start.elapsed());
        
        Ok(true)
    }
    
    fn compute_sha256(&self, data: &[u8]) -> Result<[u8; 32], HardwareError> {
        let start = Instant::now();
        
        // Regular SHA256 computation
        let hash = sha256::Hash::hash(data);
        let mut result = [0u8; 32];
        result.copy_from_slice(hash.as_byte_array());
        
        // Update metrics
        self.update_metrics("SHA256", 1, start.elapsed());
        
        Ok(result)
    }
    
    fn compute_txid(&self, tx_data: &[u8]) -> Result<[u8; 32], HardwareError> {
        // Transaction ID is double SHA256
        let hash1 = self.compute_sha256(tx_data)?;
        self.compute_sha256(&hash1)
    }
    
    fn hardware_type(&self) -> HardwareType {
        HardwareType::Software
    }
    
    fn get_metrics(&self) -> HardwareMetrics {
        self.metrics.lock()
            .map(|m| m.clone())
            .unwrap_or_else(|_| HardwareMetrics {
                hardware_type: HardwareType::Software,
                ops_per_second: 0,
                avg_latency_us: 0,
                memory_usage: 0,
                batch_size: 1,
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin::secp256k1::{SecretKey, KeyPair};
    
    #[test]
    fn test_software_accelerator_initialization() {
        let mut accelerator = SoftwareAccelerator::new();
        let result = accelerator.initialize();
        assert!(result.is_ok());
        assert!(accelerator.is_available());
    }
    
    #[test]
    fn test_software_sha256() {
        let mut accelerator = SoftwareAccelerator::new();
        accelerator.initialize().unwrap();
        
        let data = b"test data for SHA256";
        let result = accelerator.compute_sha256(data).unwrap();
        
        // Verify against standard SHA256
        let expected = sha256::Hash::hash(data);
        assert_eq!(result, expected.to_byte_array());
    }
    
    #[test]
    fn test_software_schnorr_verification() {
        let mut accelerator = SoftwareAccelerator::new();
        accelerator.initialize().unwrap();
        
        // Create test keys and signature
        let secp = Secp256k1::new();
        let secret_key = SecretKey::from_slice(&[0x01; 32]).unwrap();
        let keypair = KeyPair::from_secret_key(&secp, &secret_key);
        let public_key = XOnlyPublicKey::from_keypair(&keypair).0;
        
        // Sign test message
        let message = b"test message for software verification";
        let msg_hash = sha256::Hash::hash(message);
        let msg = Message::from_digest(msg_hash.to_byte_array());
        let signature = secp.sign_schnorr(&msg, &keypair);
        
        // Verify with software accelerator
        let result = accelerator.verify_schnorr_signature(
            message,
            &signature,
            &public_key,
        ).unwrap();
        
        assert!(result);
    }
    
    #[test]
    fn test_software_batch_verification() {
        let mut accelerator = SoftwareAccelerator::new();
        accelerator.initialize().unwrap();
        
        // Create test data for batch verification
        let secp = Secp256k1::new();
        let mut messages = Vec::new();
        let mut signatures = Vec::new();
        let mut public_keys = Vec::new();
        
        // Create 5 test signatures
        for i in 0..5 {
            let secret_key = SecretKey::from_slice(&[i as u8 + 1; 32]).unwrap();
            let keypair = KeyPair::from_secret_key(&secp, &secret_key);
            let public_key = XOnlyPublicKey::from_keypair(&keypair).0;
            
            let message = format!("software test message {}", i).into_bytes();
            let msg_hash = sha256::Hash::hash(&message);
            let msg = Message::from_digest(msg_hash.to_byte_array());
            let signature = secp.sign_schnorr(&msg, &keypair);
            
            messages.push(message);
            signatures.push(signature);
            public_keys.push(public_key);
        }
        
        // Prepare message references
        let message_refs: Vec<&[u8]> = messages.iter().map(|m| m.as_slice()).collect();
        
        // Verify batch
        let result = accelerator.verify_schnorr_batch(
            &message_refs,
            &signatures,
            &public_keys,
        ).unwrap();
        
        assert!(result);
    }
}
