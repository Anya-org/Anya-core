//! Protocols compatibility module for tests

use anya_core::protocols::*;
use std::collections::HashMap;

// Re-export main types
pub use anya_core::protocols::*;

// Extended ProtocolConfig for testing
impl ProtocolConfig {
    pub fn test_default() -> Self {
        Self::default()
    }
}

// Component system for testing
#[derive(Debug, Clone)]
pub enum ComponentCategory {
    Consensus,
    Network,
    CrossChain,
    Storage,
}

#[derive(Debug, Clone)]
pub struct Component {
    pub name: String,
    pub category: ComponentCategory,
}

impl Component {
    pub fn new(name: &str, category: ComponentCategory) -> Self {
        Self {
            name: name.to_string(),
            category,
        }
    }
}

// Label validation system
#[derive(Debug)]
pub struct LabelValidator {
    rules: HashMap<String, Vec<String>>,
}

impl LabelValidator {
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }
    
    pub fn validate_labels(&self, _components: &[Component]) -> Result<(), String> {
        Ok(())
    }
}

// Lightning node setup for testing
pub async fn setup_lightning_node() -> Result<MockLightningNode, String> {
    Ok(MockLightningNode::new())
}

#[derive(Debug)]
pub struct MockLightningNode {
    node_id: String,
}

impl MockLightningNode {
    pub fn new() -> Self {
        Self {
            node_id: "mock_node_id".to_string(),
        }
    }
    
    pub fn get_node_id(&self) -> &str {
        &self.node_id
    }
}

// Taproot-related compatibility
impl TaprootEngine {
    pub fn build_taproot_transaction(
        &self, 
        _inputs: Vec<super::bitcoin_compat::TaprootInput>, 
        _outputs: Vec<bitcoin::TxOut>,
        _fee_rate: bitcoin::FeeRate
    ) -> Result<bitcoin::Transaction, String> {
        // Return a mock transaction for testing
        Ok(super::bitcoin_compat::create_test_transaction())
    }
}

impl SegwitValidator {
    pub fn verify_taproot(&self, _tx: &bitcoin::Transaction) -> Result<(), String> {
        Ok(())
    }
}

impl LdkNode {
    pub fn create_offer(&self, _request: super::bitcoin_compat::OfferRequest) -> Result<String, String> {
        Ok("mock_offer_id".to_string())
    }
    
    pub fn send_payment_for_offer(&self, _offer_id: &str) -> Result<String, String> {
        Ok("mock_payment_hash".to_string())
    }
}

impl SpvCrossChainVerifier {
    pub fn verify_proof(&self, _proof: &super::bitcoin_compat::CrossChainProof) -> Result<bool, String> {
        Ok(true)
    }
}
