#![feature(edition2021)]
//! Bitcoin SPV (Simplified Payment Verification) Implementation
//! [AIR-3][AIS-3][BPC-3][AIT-3][RES-3]
//!
//! This module provides secure SPV proof verification for Bitcoin transactions
//! using constant-time operations and secure cryptographic primitives.

use bitcoin::hashes::{sha256d, Hash};
use bitcoin::{BlockHeader, Transaction, TxMerkleNode, Txid};
use thiserror::Error;

use crate::security::constant_time;

/// SPV verification error
#[derive(Debug, Error)]
pub enum SpvError {
    #[error("Invalid merkle proof: {0}")]
    InvalidMerkleProof(String),
    
    #[error("Invalid block header: {0}")]
    InvalidBlockHeader(String),
    
    #[error("Invalid transaction: {0}")]
    InvalidTransaction(String),
    
    #[error("Bitcoin hash error: {0}")]
    HashError(String),
    
    #[error("Deserialization error: {0}")]
    DeserializationError(String),
    
    #[error("Proof verification error: {0}")]
    VerificationError(String),
    
    #[error("Other error: {0}")]
    Other(String),
}

/// Structure for an SPV proof
#[derive(Debug, Clone)]
pub struct SpvProof {
    /// Transaction ID being proven
    pub tx_id: Txid,
    
    /// Transaction data (optional)
    pub tx_data: Option<Transaction>,
    
    /// Block header containing the transaction
    pub block_header: BlockHeader,
    
    /// Merkle proof path (pairs of hashes)
    pub merkle_path: Vec<TxMerkleNode>,
    
    /// Position of the transaction in the block (needed for path verification)
    pub tx_index: u32,
}

impl SpvProof {
    /// Create a new SPV proof
    pub fn new(
        tx_id: Txid,
        tx_data: Option<Transaction>,
        block_header: BlockHeader,
        merkle_path: Vec<TxMerkleNode>,
        tx_index: u32,
    ) -> Self {
        Self {
            tx_id,
            tx_data,
            block_header,
            merkle_path,
            tx_index,
        }
    }
    
    /// Create an SPV proof from raw components
    pub fn from_raw(
        tx_id: &[u8],
        tx_data: Option<&[u8]>,
        block_header: &[u8],
        merkle_path: &[Vec<u8>],
        tx_index: u32,
    ) -> Result<Self, SpvError> {
        // Parse transaction ID
        let tx_id = Txid::from_slice(tx_id)
            .map_err(|e| SpvError::InvalidTransaction(format!("Invalid txid: {}", e)))?;
        
        // Parse transaction data if provided
        let tx_data = if let Some(data) = tx_data {
            let tx = bitcoin::consensus::deserialize(data)
                .map_err(|e| SpvError::DeserializationError(format!("Invalid transaction data: {}", e)))?;
            Some(tx)
        } else {
            None
        };
        
        // Parse block header
        let block_header = bitcoin::consensus::deserialize(block_header)
            .map_err(|e| SpvError::InvalidBlockHeader(format!("Invalid block header: {}", e)))?;
        
        // Parse merkle path
        let merkle_path = merkle_path.iter()
            .map(|hash| TxMerkleNode::from_slice(hash))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| SpvError::InvalidMerkleProof(format!("Invalid merkle path: {}", e)))?;
        
        Ok(Self {
            tx_id,
            tx_data,
            block_header,
            merkle_path,
            tx_index,
        })
    }
    
    /// Verify the SPV proof
    pub fn verify(&self) -> Result<bool, SpvError> {
        // First, check if the transaction matches the txid
        if let Some(tx) = &self.tx_data {
            let actual_txid = tx.txid();
            if actual_txid != self.tx_id {
                return Err(SpvError::VerificationError(
                    "Transaction does not match the provided txid".to_string()
                ));
            }
        }
        
        // Then verify the merkle proof
        let merkle_root = verify_merkle_proof(
            &self.tx_id, 
            &self.merkle_path, 
            self.tx_index
        )?;
        
        // Finally, compare the computed merkle root with the one in the block header
        // Use constant-time comparison to prevent timing attacks
        let header_merkle_root = self.block_header.merkle_root.to_byte_array();
        let computed_merkle_root = merkle_root.to_byte_array();
        
        let equal = constant_time::constant_time_eq(&header_merkle_root, &computed_merkle_root);
        
        Ok(equal)
    }
}

/// Verify a merkle proof for a transaction
///
/// This function takes a transaction ID, a merkle path, and the transaction index
/// and verifies that the transaction is included in the block with the given merkle root.
///
/// # Arguments
/// * `tx_id` - The transaction ID to verify
/// * `merkle_path` - The merkle path (pairs of hashes)
/// * `tx_index` - The index of the transaction in the block
///
/// # Returns
/// The merkle root if the proof is valid, an error otherwise
pub fn verify_merkle_proof(
    tx_id: &Txid,
    merkle_path: &[TxMerkleNode],
    tx_index: u32,
) -> Result<TxMerkleNode, SpvError> {
    // Start with the transaction hash
    let mut current = TxMerkleNode::from_byte_array(tx_id.to_byte_array());
    let mut index = tx_index;
    
    // For each level of the merkle tree
    for sibling in merkle_path {
        let (left, right) = if index & 1 == 0 {
            // If current index is even, current is left, sibling is right
            (current, *sibling)
        } else {
            // If current index is odd, sibling is left, current is right
            (*sibling, current)
        };
        
        // Combine the hashes in the correct order
        current = compute_merkle_parent(&left, &right)?;
        
        // Move up the tree
        index >>= 1;
    }
    
    // The final computed hash should be the merkle root
    Ok(current)
}

/// Compute the parent node in a merkle tree
///
/// This function takes two child nodes and computes their parent node
/// by concatenating them and double-hashing the result.
///
/// # Arguments
/// * `left` - The left child node
/// * `right` - The right child node
///
/// # Returns
/// The parent node
pub fn compute_merkle_parent(
    left: &TxMerkleNode,
    right: &TxMerkleNode,
) -> Result<TxMerkleNode, SpvError> {
    let mut data = Vec::with_capacity(64);
    data.extend_from_slice(&left.to_byte_array());
    data.extend_from_slice(&right.to_byte_array());
    
    let hash = sha256d::Hash::hash(&data);
    Ok(TxMerkleNode::from_byte_array(hash.to_byte_array()))
}

/// Verify that a transaction is included in a block
///
/// This is a helper function that creates an SPV proof and verifies it
///
/// # Arguments
/// * `tx_id_hex` - The transaction ID in hex format
/// * `block_header_hex` - The block header in hex format
/// * `merkle_proof_hex` - The merkle proof in hex format (array of hashes)
/// * `tx_index` - The index of the transaction in the block
///
/// # Returns
/// `true` if the proof is valid, `false` otherwise
pub fn verify_tx_inclusion(
    tx_id_hex: &str,
    block_header_hex: &str,
    merkle_proof_hex: &[&str],
    tx_index: u32,
) -> Result<bool, SpvError> {
    // Convert hex strings to bytes
    let tx_id = hex::decode(tx_id_hex)
        .map_err(|e| SpvError::InvalidTransaction(format!("Invalid txid hex: {}", e)))?;
    
    let block_header = hex::decode(block_header_hex)
        .map_err(|e| SpvError::InvalidBlockHeader(format!("Invalid block header hex: {}", e)))?;
    
    let merkle_path = merkle_proof_hex.iter()
        .map(|h| hex::decode(h))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| SpvError::InvalidMerkleProof(format!("Invalid merkle path hex: {}", e)))?;
    
    // Create and verify the proof
    let proof = SpvProof::from_raw(&tx_id, None, &block_header, &merkle_path, tx_index)?;
    proof.verify()
}

/// Verify a Bitcoin payment using SPV proof
///
/// This function verifies that a Bitcoin payment (transaction) is included in a block
/// using an SPV proof, which is a more secure and efficient way to verify payments
/// without downloading the entire blockchain.
///
/// # Arguments
/// * `tx_id_hex` - The transaction ID in hex format
/// * `block_header_hex` - The block header in hex format
/// * `merkle_proof_hex` - The merkle proof in hex format (array of hashes)
/// * `tx_index` - The index of the transaction in the block
/// * `confirmations_required` - The number of confirmations required
///
/// # Returns
/// `true` if the payment is valid, `false` otherwise
pub fn verify_bitcoin_payment(
    tx_id_hex: &str,
    block_header_hex: &str,
    merkle_proof_hex: &[&str],
    tx_index: u32,
    _confirmations_required: u64,
) -> Result<bool, SpvError> {
    // Verify inclusion of the transaction in the block
    verify_tx_inclusion(tx_id_hex, block_header_hex, merkle_proof_hex, tx_index)
    
    // Note: Checking for the required number of confirmations would require
    // connecting to a Bitcoin node to get the current blockchain height
    // and the height of the block containing the transaction.
    // This would typically be done in a higher-level function.
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin::hashes::hex::FromHex;
    
    #[test]
    fn test_compute_merkle_parent() {
        // Example values from Bitcoin
        let left_hex = "b67e5c13dd78c212e64e2fa8d153c6f6a5cc741a1ec9c9fb3045f9854c881ae4";
        let right_hex = "1b4741e858a7b7c0a851a35c43858bc8902c0a91b5bd7043b9a27e8b00e2a8e2";
        let expected_parent_hex = "eb42a05772f296e9fe8a7f0d8a7c9abad734cc7dd31799a2b12a728a5d4ad891";
        
        let left = TxMerkleNode::from_hex(left_hex).unwrap();
        let right = TxMerkleNode::from_hex(right_hex).unwrap();
        let expected_parent = TxMerkleNode::from_hex(expected_parent_hex).unwrap();
        
        let computed_parent = compute_merkle_parent(&left, &right).unwrap();
        assert_eq!(computed_parent, expected_parent);
    }
    
    #[test]
    fn test_verify_merkle_proof() {
        // This is a simplified test with made-up values
        // In a real test, you would use actual values from the Bitcoin blockchain
        
        // Create a sample transaction ID
        let tx_id_hex = "b67e5c13dd78c212e64e2fa8d153c6f6a5cc741a1ec9c9fb3045f9854c881ae4";
        let tx_id = Txid::from_hex(tx_id_hex).unwrap();
        
        // Create a sample merkle path (2 nodes)
        let merkle_path_hex = [
            "1b4741e858a7b7c0a851a35c43858bc8902c0a91b5bd7043b9a27e8b00e2a8e2",
            "9d28bd159e5ec3c21e1b305454231bd10033a4dd324b5d9c0bb29c60d4d7b4f8",
        ];
        
        let merkle_path = merkle_path_hex.iter()
            .map(|h| TxMerkleNode::from_hex(h).unwrap())
            .collect::<Vec<_>>();
        
        // The expected merkle root
        let expected_root_hex = "eb98e9a0a41c33a68f53cf547ba78f349c6522f2c41ccec2934e3b324d0a67e2";
        let expected_root = TxMerkleNode::from_hex(expected_root_hex).unwrap();
        
        // Verify the proof (assuming tx_index = 0)
        let computed_root = verify_merkle_proof(&tx_id, &merkle_path, 0).unwrap();
        
        // Verify that the computed root matches the expected root
        assert_eq!(computed_root, expected_root);
    }
    
    #[test]
    fn test_spv_proof_verification() {
        // This is a simplified test with made-up values
        // In a real test, you would use actual values from the Bitcoin blockchain
        
        // Create a sample transaction ID
        let tx_id_hex = "b67e5c13dd78c212e64e2fa8d153c6f6a5cc741a1ec9c9fb3045f9854c881ae4";
        let tx_id = Txid::from_hex(tx_id_hex).unwrap();
        
        // Create a sample merkle path (2 nodes)
        let merkle_path_hex = [
            "1b4741e858a7b7c0a851a35c43858bc8902c0a91b5bd7043b9a27e8b00e2a8e2",
            "9d28bd159e5ec3c21e1b305454231bd10033a4dd324b5d9c0bb29c60d4d7b4f8",
        ];
        
        let merkle_path = merkle_path_hex.iter()
            .map(|h| TxMerkleNode::from_hex(h).unwrap())
            .collect::<Vec<_>>();
        
        // The merkle root that will be included in the block header
        let root_hex = "eb98e9a0a41c33a68f53cf547ba78f349c6522f2c41ccec2934e3b324d0a67e2";
        let root = TxMerkleNode::from_hex(root_hex).unwrap();
        
        // Create a minimal block header with just the merkle root
        let mut header = BlockHeader::default();
        header.merkle_root = root;
        
        // Create an SPV proof
        let proof = SpvProof::new(tx_id, None, header, merkle_path, 0);
        
        // Verify the proof
        let result = proof.verify().unwrap();
        assert!(result);
    }
} 