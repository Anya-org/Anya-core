use std::error::Error;
// Configuration module for Anya Core
// Provides configuration settings for various components

use std::collections::HashMap;

/// Main configuration struct for the application
#[derive(Debug, Clone)]
pub struct Config {
    /// Bitcoin network to connect to (mainnet, testnet, regtest)
    pub bitcoin_network: String,
    
    /// Bitcoin RPC connection URL
    pub bitcoin_rpc_url: String,
    
    /// Bitcoin RPC username
    pub bitcoin_rpc_user: Option<String>,
    
    /// Bitcoin RPC password
    pub bitcoin_rpc_pass: Option<String>,
    
    /// Path to Bitcoin data directory
    pub bitcoin_data_dir: Option<String>,
    
    /// Path to wallet file
    pub wallet_path: Option<String>,
    
    /// Lightning implementation type (ldk or mock)
    pub lightning_implementation: Option<String>,
    
    /// Lightning Network node public key (if already known)
    pub lightning_node_pubkey: Option<String>,
    
    /// Lightning Network listening address and port
    pub lightning_listen_addr: Option<String>,
    
    /// Lightning Network data directory
    pub lightning_data_dir: Option<String>,
    
    /// Liquid RPC connection URL
    pub liquid_rpc_url: Option<String>,
    
    /// Liquid RPC username
    pub liquid_rpc_user: Option<String>,
    
    /// Liquid RPC password
    pub liquid_rpc_pass: Option<String>,
    
    /// Liquid network to connect to (liquidv1, liquidtestnet, liquidregtest)
    pub liquid_network: Option<String>,
    
    /// Path to Liquid data directory
    pub liquid_data_dir: Option<String>,
    
    /// Web5 configuration
    pub web5_config: crate::web5::Web5Config,
    
    /// Feature flags for various components
    pub features: HashMap<String, bool>,
}

impl Default for Config {
    fn default() -> Self  -> Result<(), Box<dyn Error>> {
        let mut features = HashMap::new();
        features.insert("taproot".to_string(), true);
        features.insert("lightning".to_string(), false);
        features.insert("dlc".to_string(), false);
        features.insert("liquid".to_string(), false);
        features.insert("web5".to_string(), true);
        
        Self {
            bitcoin_network: "testnet".to_string(),
            bitcoin_rpc_url: "http://localhost:18332".to_string(),
            bitcoin_rpc_user: None,
            bitcoin_rpc_pass: None,
            bitcoin_data_dir: None,
            wallet_path: None,
            lightning_implementation: Some("ldk".to_string()),
            lightning_node_pubkey: None,
            lightning_listen_addr: None,
            lightning_data_dir: None,
            liquid_rpc_url: Some("http://localhost:7041".to_string()),
            liquid_rpc_user: None,
            liquid_rpc_pass: None,
            liquid_network: Some("liquidtestnet".to_string()),
            liquid_data_dir: None,
            web5_config: crate::web5::Web5Config::default(),
            features,
        }
    }
}

impl Config {
    /// Create a configuration from environment variables
    pub fn from_env() -> Self  -> Result<(), Box<dyn Error>> {
        let mut config = Config::default();
        
        // Bitcoin configuration
        if let Ok(network) = std::env::var("BITCOIN_NETWORK") {
            config.bitcoin_network = network;
        }
        
        if let Ok(rpc_url) = std::env::var("BITCOIN_RPC_URL") {
            config.bitcoin_rpc_url = rpc_url;
        }
        
        if let Ok(rpc_user) = std::env::var("BITCOIN_RPC_USER") {
            config.bitcoin_rpc_user = Some(rpc_user);
        }
        
        if let Ok(rpc_pass) = std::env::var("BITCOIN_RPC_PASS") {
            config.bitcoin_rpc_pass = Some(rpc_pass);
        }
        
        if let Ok(data_dir) = std::env::var("BITCOIN_DATA_DIR") {
            config.bitcoin_data_dir = Some(data_dir);
        }
        
        if let Ok(wallet_path) = std::env::var("WALLET_PATH") {
            config.wallet_path = Some(wallet_path);
        }
        
        // Lightning configuration
        if let Ok(lightning_impl) = std::env::var("LIGHTNING_IMPLEMENTATION") {
            config.lightning_implementation = Some(lightning_impl);
        }
        
        if let Ok(lightning_pubkey) = std::env::var("LIGHTNING_NODE_PUBKEY") {
            config.lightning_node_pubkey = Some(lightning_pubkey);
        }
        
        if let Ok(lightning_addr) = std::env::var("LIGHTNING_LISTEN_ADDR") {
            config.lightning_listen_addr = Some(lightning_addr);
        }
        
        if let Ok(lightning_dir) = std::env::var("LIGHTNING_DATA_DIR") {
            config.lightning_data_dir = Some(lightning_dir);
        }
        
        // Liquid configuration
        if let Ok(liquid_rpc_url) = std::env::var("LIQUID_RPC_URL") {
            config.liquid_rpc_url = Some(liquid_rpc_url);
        }
        
        if let Ok(liquid_rpc_user) = std::env::var("LIQUID_RPC_USER") {
            config.liquid_rpc_user = Some(liquid_rpc_user);
        }
        
        if let Ok(liquid_rpc_pass) = std::env::var("LIQUID_RPC_PASS") {
            config.liquid_rpc_pass = Some(liquid_rpc_pass);
        }
        
        if let Ok(liquid_network) = std::env::var("LIQUID_NETWORK") {
            config.liquid_network = Some(liquid_network);
        }
        
        if let Ok(liquid_data_dir) = std::env::var("LIQUID_DATA_DIR") {
            config.liquid_data_dir = Some(liquid_data_dir);
        }
        
        // Web5 configuration
        if let Ok(did_method) = std::env::var("WEB5_DID_METHOD") {
            config.web5_config.did_method = did_method;
        }
        
        if let Ok(dwn_endpoint) = std::env::var("WEB5_DWN_ENDPOINT") {
            config.web5_config.dwn_url = Some(dwn_endpoint);
        }
        
        // Feature flags
        if let Ok(features_str) = std::env::var("ENABLED_FEATURES") {
            for feature in features_str.split(',') {
                let feature = feature.trim();
                if !feature.is_empty() {
                    config.features.insert(feature.to_string(), true);
                }
            }
        }
        
        config
    }
    
    /// Check if a feature is enabled
    pub fn is_feature_enabled(&self, feature: &str) -> bool  -> Result<(), Box<dyn Error>> {
        self.features.get(feature).copied().unwrap_or(false)
    }
    
    /// Set a feature flag
    pub fn set_feature(&mut self, feature: &str, enabled: bool)  -> Result<(), Box<dyn Error>> {
        self.features.insert(feature.to_string(), enabled);
    }
    
    /// Get the Bitcoin implementation type
    pub fn get_bitcoin_implementation_type(&self) -> crate::bitcoin::interface::BitcoinImplementationType  -> Result<(), Box<dyn Error>> {
        crate::bitcoin::interface::BitcoinImplementationType::Rust
    }
    
    /// Check if Liquid is enabled
    pub fn is_liquid_enabled(&self) -> bool  -> Result<(), Box<dyn Error>> {
        self.is_feature_enabled("liquid")
    }
    
    /// Check if Web5 is enabled
    pub fn is_web5_enabled(&self) -> bool  -> Result<(), Box<dyn Error>> {
        self.is_feature_enabled("web5")
    }
}

/// Create a test configuration for unit tests
pub fn test_config() -> Config  -> Result<(), Box<dyn Error>> {
    let mut config = Config::default();
    config.bitcoin_network = "regtest".to_string();
    config.bitcoin_rpc_url = "http://localhost:18443".to_string();
    config.liquid_network = Some("liquidregtest".to_string());
    config.liquid_rpc_url = Some("http://localhost:18884".to_string());
    config
} 
