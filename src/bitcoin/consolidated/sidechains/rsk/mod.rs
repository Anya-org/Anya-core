// [AIR-3][AIS-3][AIM-3][BPC-3][RES-3]
//! RSK Sidechain integration module
//! 
//! Implementation following Bitcoin Development Framework v2.5

mod bitcoin_verification;

pub use bitcoin_verification::{BitcoinSPV, BlockHeader, RskBitcoinVerifier, verify_merkle_proof};

/// RSK sidechain configuration according to hexagonal architecture requirements
pub struct RskConfig {
    /// URL of the RSK node
    pub node_url: String,
    /// Contract address for Bitcoin verification
    pub contract_address: String,
    /// Verification mode (SPV, Full Node)
    pub verification_mode: VerificationMode,
}

/// Verification modes for RSK Bitcoin verification
pub enum VerificationMode {
    /// Simple Payment Verification (merkle proofs)
    SPV,
    /// Full node verification
    FullNode,
    /// Federated verification through threshold signatures
    Federated,
}

/// RSK adapter as required by hexagonal architecture
#[derive(Clone)]
pub struct RskAdapter {
    config: RskConfig,
    verifier: Option<RskBitcoinVerifier>,
}

impl RskAdapter {
    /// Create new RSK adapter
    pub fn new(config: RskConfig) -> Self {
        Self {
            config,
            verifier: None,
        }
    }
    
    /// Initialize the adapter
    pub fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.verifier = Some(RskBitcoinVerifier::new(
            &self.config.node_url,
            &self.config.contract_address,
        ));
        Ok(())
    }
    
    /// Verify Bitcoin payment through RSK
    #[rsk_bind]
    pub fn verify_bitcoin_payment(&self, proof: BitcoinSPV) -> Result<bool, Box<dyn std::error::Error>> {
        match &self.verifier {
            Some(v) => v.verify_bitcoin_payment(proof),
            None => Err("RSK adapter not initialized".into()),
        }
    }
}
