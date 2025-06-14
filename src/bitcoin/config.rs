// Bitcoin configuration module
use serde::{Deserialize, Serialize};

/// Bitcoin network configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BitcoinConfig {
    /// Whether Bitcoin functionality is enabled
    pub enabled: bool,
    /// Network to use (mainnet, testnet, regtest)
    pub network: String,
    /// RPC connection details
    pub rpc_url: Option<String>,
    /// Authentication credentials (username, password)
    pub auth: Option<(String, String)>,
    /// Minimum confirmations required for transactions
    pub min_confirmations: u32,
    /// Fee rate in satoshis per byte
    pub default_fee_rate: u64,
    /// Path to wallet file (if applicable)
    pub wallet_path: Option<String>,
}

impl Default for BitcoinConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            network: "testnet".to_string(),
            rpc_url: Some("http://127.0.0.1:18332".to_string()),
            auth: None,
            min_confirmations: 6,
            default_fee_rate: 10,
            wallet_path: None,
        }
    }
}

impl BitcoinConfig {
    /// Check if a specific BIP is supported
    pub fn supports_bip(&self, bip: &str) -> Result<bool, Box<dyn std::error::Error>> {
        match bip {
            "BIP-341" => Ok(true), // Taproot support
            "BIP-340" => Ok(true), // Schnorr signatures  
            "BIP-174" => Ok(true), // PSBT support
            _ => Ok(false),
        }
    }
}
