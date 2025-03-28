#![feature(edition2021)]
// [BPC-3][AIR-3][RES-3]
use crate::ports::{BitcoinPort, DIDPort, CryptoPort};
use super::{AnyaL4Protocol, ProtocolError};

/// L4 protocol adapter for hexagonal architecture
pub struct L4ProtocolAdapter {
    l4_protocol: AnyaL4Protocol,
    bitcoin_port: Arc<dyn BitcoinPort>,
    did_port: Arc<dyn DIDPort>,
    crypto_port: Arc<dyn CryptoPort>,
}

impl L4ProtocolAdapter {
    pub fn new(
        network: Network, 
        bitcoin_port: Arc<dyn BitcoinPort>,
        did_port: Arc<dyn DIDPort>,
        crypto_port: Arc<dyn CryptoPort>,
    ) -> Self {
        let mut l4_protocol = AnyaL4Protocol::new(network);
        
        // Connect to HSM via crypto port
        if let Ok(hsm_type) = crypto_port.get_hsm_type() {
            l4_protocol.init_hsm(&hsm_type).ok();
        }
        
        Self {
            l4_protocol,
            bitcoin_port,
            did_port,
            crypto_port,
        }
    }
    
    pub async fn execute_transaction(
        &mut self, 
        psbt_hex: &str
    ) -> Result<String, ProtocolError> {
        // Parse PSBT
        let psbt = match self.bitcoin_port.parse_psbt(psbt_hex) {
            Ok(psbt) => psbt,
            Err(e) => return Err(ProtocolError::Unknown(e.to_string())),
        };
        
        // Send via L4 protocol
        self.l4_protocol.send_private_transaction(psbt).await
    }
} 