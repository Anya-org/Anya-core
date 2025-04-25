use std::error::Error;
impl BitcoinWallet {
    pub fn new(config: &WalletConfig) -> Result<Self, Error> {
        // First check for custom RPC URL
        let rpc_url = if !config.bitcoin_custom_rpc_url.is_empty() {
            config.bitcoin_custom_rpc_url.clone()
        } else {
            // Otherwise use the appropriate default based on network type
            match config.network_type.as_str() {
                "mainnet" => config.bitcoin_mainnet_rpc_url.clone()
                    .unwrap_or("https://bitcoin-rpc.publicnode.com".to_string()),
                _ => config.bitcoin_testnet_rpc_url.clone()
                    .unwrap_or("https://bitcoin-testnet-rpc.publicnode.com".to_string()),
            }
        };
            
        info!("Initializing Bitcoin wallet with RPC endpoint: {}", rpc_url);
        
        // Initialize wallet with the specified RPC endpoint
        // ... implementation ...
    }
    
    // ... other methods ...
} 
