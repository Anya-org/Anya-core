#![feature(edition2021)]
//! AIM-004: Layer 2 Integration Tests
//! 
//! Tests for the Layer 2 implementations:
//! - BOB (Bitcoin Optimistic Blockchain)
//! - RGB Protocol
//! - RSK Sidechain
//! - Layer 2 Framework

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use anya_bitcoin::layer2::ports::{Layer2Protocol, TransactionStatus};
    
    // BOB Protocol tests
    #[test]
    fn test_bob_protocol() {
        use anya_bitcoin::layer2::bob::{BobClient, BobConfig, BobNetwork};
        
        let config = BobConfig {
            endpoint: "https://bob-node.example.com".to_string(),
            auth_token: Some("test_token".to_string()),
            network: BobNetwork::Testnet,
        };
        
        let client = BobClient::new(config);
        
        // Test Layer2Protocol trait implementation
        assert!(client.initialize().is_ok());
        assert!(client.connect().is_ok());
        
        let tx_data = vec![0u8; 32]; // Mock transaction data
        let tx_id = client.submit_transaction(&tx_data).unwrap();
        assert!(!tx_id.is_empty());
        
        let status = client.get_transaction_status(&tx_id).unwrap();
        assert_eq!(status, TransactionStatus::Pending);
    }
    
    // RGB Protocol tests
    #[test]
    fn test_rgb_protocol() {
        use anya_bitcoin::layer2::rgb::{RgbClient, RgbConfig, RgbNetwork};
        
        let config = RgbConfig {
            network: RgbNetwork::Testnet,
            storage_path: "/tmp/rgb-test".to_string(),
        };
        
        let client = RgbClient::new(config);
        
        // Test Layer2Protocol trait implementation
        assert!(client.initialize().is_ok());
        assert!(client.connect().is_ok());
        
        // Test Taproot asset creation
        let asset = client.create_taproot_asset("TestAsset", 1000000, 8, Some("Test metadata")).unwrap();
        assert_eq!(asset.name, "TestAsset");
        assert_eq!(asset.supply, 1000000);
        assert_eq!(asset.precision, 8);
        
        // Test asset transfer
        let transfer_id = client.transfer_asset(&asset.id, "recipient", 100).unwrap();
        assert!(!transfer_id.is_empty());
    }
    
    // RSK Sidechain tests
    #[test]
    fn test_rsk_protocol() {
        use anya_bitcoin::layer2::rsk::{RskClient, RskConfig, RskNetwork, BitcoinSPV};
        
        let config = RskConfig {
            endpoint: "https://rsk-node.example.com".to_string(),
            network: RskNetwork::Testnet,
            private_key: None,
        };
        
        let client = RskClient::new(config);
        
        // Test Layer2Protocol trait implementation
        assert!(client.initialize().is_ok());
        assert!(client.connect().is_ok());
        
        // Test Bitcoin payment verification
        let proof = BitcoinSPV {
            tx_hash: "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            block_header: vec![0; 80],
            merkle_proof: vec!["0000000000000000000000000000000000000000000000000000000000000000".to_string()],
            block_height: 1,
        };
        
        assert!(client.verify_bitcoin_payment(proof).unwrap());
    }
    
    // Layer 2 Framework tests
    #[test]
    fn test_layer2_framework() {
        use anya_bitcoin::layer2::framework::{Layer2Factory, Layer2Registry, ProtocolConfig};
        use std::fmt::Debug;
        
        // Create factory and registry
        let factory = Arc::new(Layer2Factory::new());
        let registry = Arc::new(Layer2Registry::new(factory.clone()));
        
        #[derive(Debug, Clone)]
        struct TestConfig {
            name: String,
            network: String,
        }
        
        impl ProtocolConfig for TestConfig {
            fn protocol_name(&self) -> &str {
                &self.name
            }
            
            fn network_type(&self) -> &str {
                &self.network
            }
            
            fn clone_box(&self) -> Box<dyn ProtocolConfig> {
                Box::new(self.clone())
            }
        }
        
        struct TestProtocol;
        
        impl Layer2Protocol for TestProtocol {
            fn initialize(&self) -> anyhow::Result<()> {
                Ok(())
            }
            
            fn connect(&self) -> anyhow::Result<()> {
                Ok(())
            }
            
            fn submit_transaction(&self, _transaction: &[u8]) -> anyhow::Result<String> {
                Ok("test_tx".to_string())
            }
            
            fn get_transaction_status(&self, _tx_id: &str) -> anyhow::Result<TransactionStatus> {
                Ok(TransactionStatus::Confirmed)
            }
        }
        
        // Register a test protocol manually (normally done by factory)
        assert!(registry.register_protocol("test", Box::new(TestProtocol)).is_ok());
        
        // Get protocol instance
        let protocol = registry.get_protocol("test");
        assert!(protocol.is_some());
        
        // Test protocol methods
        let protocol = protocol.unwrap();
        assert!(protocol.initialize().is_ok());
        assert!(protocol.connect().is_ok());
        
        let tx_id = protocol.submit_transaction(&vec![0u8; 32]).unwrap();
        assert_eq!(tx_id, "test_tx");
        
        let status = protocol.get_transaction_status(&tx_id).unwrap();
        assert_eq!(status, TransactionStatus::Confirmed);
    }
} 