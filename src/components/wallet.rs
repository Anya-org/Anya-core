use crate::bitcoin::endpoints::BitcoinRpcEndpoints;
use std::error::Error;
impl BitcoinWallet {
    pub fn new(config: &WalletConfig) -> Result<Self, Error> {
        // First check for custom RPC URL
        let rpc_url = if !config.bitcoin_custom_rpc_url.is_empty() {
            config.bitcoin_custom_rpc_url.clone()
        } else {
            let eps = BitcoinRpcEndpoints::resolve(
                config.bitcoin_mainnet_rpc_url.as_deref(),
                config.bitcoin_testnet_rpc_url.as_deref(),
            );
            eps.for_network(&config.network_type).to_string()
        };

        info!("Initializing Bitcoin wallet with RPC endpoint: {}", rpc_url);

        // Initialize wallet with the specified RPC endpoint
        // ... implementation ...
    }

    // ... other methods ...
}
