// Ports Module - Bitcoin Development Framework v2.5
// Hexagonal Architecture Implementation with strict BIP compliance
// [AIR-3][AIS-3][BPC-3][AIT-3][RES-3]

use std::error::Error;

/// Define all required ports according to BDF v2.5 Hexagonal Architecture
pub mod node_communication {
    use crate::bitcoin::protocol::BitcoinError;
    use async_trait::async_trait;
    
    /// Defines the interface for node communication
    #[async_trait]
    pub trait NodeCommunicationPort {
        /// Connect to the Bitcoin P2P network
        async fn connect(&self) -> Result<(), BitcoinError>;
        
        /// Broadcast a transaction to the network
        async fn broadcast_transaction(&self, tx_hex: &str) -> Result<String, BitcoinError>;
        
        /// Get mempool statistics
        async fn get_mempool_stats(&self) -> Result<MempoolStats, BitcoinError>;
        
        /// Check if node is in sync with the network
        async fn is_synced(&self) -> Result<bool, BitcoinError>;
    }
    
    /// Mempool statistics data structure
    pub struct MempoolStats {
        /// Number of transactions in mempool
        pub tx_count: usize,
        
        /// Total size of mempool in bytes
        pub size_bytes: usize,
        
        /// Fee statistics
        pub fee_stats: FeeStats,
    }
    
    /// Fee statistics for mempool
    pub struct FeeStats {
        /// Minimum fee rate (sat/vB)
        pub min_fee_rate: f64,
        
        /// Median fee rate (sat/vB)
        pub median_fee_rate: f64,
        
        /// Fee rate needed for next block (sat/vB)
        pub next_block_fee_rate: f64,
    }
}

/// Wallet interface port with BIP-174 (PSBT) support
pub mod wallet_interface {
    use crate::bitcoin::error::Error;
    use async_trait::async_trait;
    
    /// Wallet port for transaction operations
    #[async_trait]
    pub trait WalletPort {
        /// Create a new PSBT according to BIP-174
        async fn create_psbt(&self, 
                      recipients: &[Recipient], 
                      fee_rate: f64) -> Result<String, Error>;
        
        /// Sign a PSBT
        async fn sign_psbt(&self, psbt: &str) -> Result<String, Error>;
        
        /// Finalize a PSBT into a transaction
        async fn finalize_psbt(&self, psbt: &str) -> Result<String, Error>;
        
        /// Create a Taproot-enabled PSBT (BIP-341/342)
        async fn create_taproot_psbt(&self, 
                              script_tree: &ScriptTree, 
                              recipients: &[Recipient]) -> Result<String, Error>;
    }
    
    /// Recipient for a transaction
    pub struct Recipient {
        /// Address to send to
        pub address: String,
        
        /// Amount in satoshis
        pub amount: u64,
    }
    
    /// Script tree for Taproot
    pub struct ScriptTree {
        /// Internal key
        pub internal_key: String,
        
        /// Script branches
        pub script_branches: Vec<ScriptBranch>,
    }
    
    /// Script branch in a Taproot tree
    pub struct ScriptBranch {
        /// Script in hex
        pub script: String,
        
        /// Leaf version
        pub leaf_version: u8,
    }
}

/// Smart contract port with Miniscript support
pub mod smart_contract {
    use crate::bitcoin::error::Error;
    use async_trait::async_trait;
    
    /// Smart contract port for script operations
    #[async_trait]
    pub trait SmartContractPort {
        /// Compile Miniscript to Bitcoin Script
        async fn compile_miniscript(&self, miniscript: &str) -> Result<String, Error>;
        
        /// Analyze script for resource usage
        async fn analyze_script(&self, script: &str) -> Result<ScriptAnalysis, Error>;
        
        /// Execute script in test environment
        async fn test_execute(&self, script: &str, 
                       inputs: &[ScriptInput]) -> Result<ScriptExecutionResult, Error>;
    }
    
    /// Analysis of a script
    pub struct ScriptAnalysis {
        /// Size of the script in bytes
        pub size: usize,
        
        /// Opcode count
        pub opcode_count: usize,
        
        /// Signature operations count
        pub sigop_count: usize,
        
        /// Is the script Taproot compatible
        pub taproot_compatible: bool,
    }
    
    /// Input for script testing
    pub struct ScriptInput {
        /// Stack element as hex
        pub stack_element: String,
    }
    
    /// Result of script execution
    pub struct ScriptExecutionResult {
        /// Success or failure
        pub success: bool,
        
        /// Final stack
        pub final_stack: Vec<String>,
        
        /// Error message if any
        pub error: Option<String>,
    }
}

/// Metrics port for system monitoring
pub mod metrics {
    use crate::bitcoin::error::Error;
    use async_trait::async_trait;
    
    /// Metrics port for performance monitoring
    #[async_trait]
    pub trait MetricsPort {
        /// Get TPS (transactions per second) metric
        async fn get_tps(&self) -> Result<f64, Error>;
        
        /// Get block propagation time
        async fn get_block_propagation_time(&self) -> Result<f64, Error>;
        
        /// Get mempool depth analysis
        async fn get_mempool_depth(&self) -> Result<MempoolDepth, Error>;
        
        /// Export metrics to Prometheus
        async fn export_prometheus_metrics(&self) -> Result<String, Error>;
    }
    
    /// Mempool depth analysis
    pub struct MempoolDepth {
        /// Total size in bytes
        pub size_bytes: usize,
        
        /// Number of transactions
        pub tx_count: usize,
        
        /// Size categories
        pub size_categories: MempoolSizeCategories,
    }
    
    /// Mempool size categories
    pub struct MempoolSizeCategories {
        /// < 1 vB/sat transactions
        pub low_fee_bytes: usize,
        
        /// 1-5 vB/sat transactions
        pub medium_fee_bytes: usize,
        
        /// > 5 vB/sat transactions
        pub high_fee_bytes: usize,
    }
}

// Concrete implementations
pub mod p2p;
pub mod wallet;
pub mod contracts;

/// Common traits for all ports
pub trait Port {
    /// Get the port name
    fn name(&self) -> &'static str;
    
    /// Get the port version
    fn version(&self) -> &'static str;
    
    /// Check if the port is connected
    fn is_connected(&self) -> bool;
}

/// Port management for hexagonal architecture
pub struct PortManager {
    ports: Vec<Box<dyn Port>>,
}

impl PortManager {
    /// Create a new port manager
    pub fn new() -> Self {
        PortManager {
            ports: Vec::new(),
        }
    }
    
    /// Register a new port
    pub fn register_port(&mut self, port: Box<dyn Port>) {
        self.ports.push(port);
    }
    
    /// Get all registered ports
    pub fn get_ports(&self) -> &[Box<dyn Port>] {
        &self.ports
    }
    
    /// Get a port by name
    pub fn get_port_by_name(&self, name: &str) -> Option<&Box<dyn Port>> {
        self.ports.iter().find(|p| p.name() == name)
    }
} 
