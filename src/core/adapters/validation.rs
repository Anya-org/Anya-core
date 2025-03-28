#![feature(edition2021)]
#[derive(Debug)]
pub struct HexValidator;

impl HexValidator {
    pub fn new() -> Self {
        Self
    }
    
    pub fn check_adapters(&self) -> bool {
        use crate::ports::node_communication;
        
        // Verify Bitcoin Core adapter
        let bitcoin_adapter = node_communication::BitcoinAdapter::new();
        if !bitcoin_adapter.verify_connection() {
            return false;
        }
        
        // Check Lightning Network adapter
        let lightning_adapter = node_communication::LightningAdapter::new();
        if !lightning_adapter.validate_bolt11_support() {
            return false;
        }
        
        true
    }
    
    pub fn check_hsm_integration(&self) -> bool {
        use crate::security::hsm::Yubihsm2Provider;
        
        // Test HSM connection
        let hsm = Yubihsm2Provider::new(BIP341_SILENT_LEAF);
        let test_sig = hsm.sign_taproot(&[0u8; 32]);
        
        // Verify signature format
        test_sig.len() == 64 && hsm.verify_taproot(&[0u8; 32], test_sig)
    }
} 