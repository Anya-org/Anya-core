//! Centralized resolution of Bitcoin RPC endpoints (mainnet & testnet)
//! Order of precedence (highest first):
//! 1. BITCOIN_RPC_URL (explicit override for both)
//! 2. ANYA_BITCOIN_RPC_URL (legacy combined override)
//! 3. ANYA_BITCOIN_MAINNET_RPC_URL / ANYA_BITCOIN_TESTNET_RPC_URL (network-specific)
//! 4. Config file values (passed in by caller optionally)
//! 5. Built-in public defaults (PublicNode privacy-preserving endpoints)
//!
//! These functions are environment-honest: they never fail, always returning a usable URL.
//! Users can supply self-hosted node endpoints without modifying source code.

use std::env;

/// Canonical default public endpoints (last-resort fallbacks)
pub const DEFAULT_MAINNET_RPC: &str = "https://bitcoin-rpc.publicnode.com";
pub const DEFAULT_TESTNET_RPC: &str = "https://bitcoin-testnet-rpc.publicnode.com"; // alt: https://bitcoin.publicnode.com/?testnet

#[derive(Debug, Clone)]
pub struct BitcoinRpcEndpoints {
    pub mainnet: String,
    pub testnet: String,
}

impl BitcoinRpcEndpoints {
    /// Resolve both mainnet & testnet endpoints with layered precedence.
    pub fn resolve(config_mainnet: Option<&str>, config_testnet: Option<&str>) -> Self {
        // Global explicit override (applies to both) takes top precedence.
        let global_any = env::var("BITCOIN_RPC_URL")
            .or_else(|_| env::var("ANYA_BITCOIN_RPC_URL"))
            .ok();

        // Network specific env overrides
        let env_mainnet = env::var("ANYA_BITCOIN_MAINNET_RPC_URL").ok();
        let env_testnet = env::var("ANYA_BITCOIN_TESTNET_RPC_URL").ok();

        let mainnet = global_any
            .clone()
            .or(env_mainnet)
            .or_else(|| config_mainnet.map(|s| s.to_string()))
            .unwrap_or_else(|| DEFAULT_MAINNET_RPC.to_string());
        let testnet = global_any
            .or(env_testnet)
            .or_else(|| config_testnet.map(|s| s.to_string()))
            .unwrap_or_else(|| DEFAULT_TESTNET_RPC.to_string());

        Self { mainnet, testnet }
    }

    /// Convenience: resolve without config fallback.
    pub fn resolve_simple() -> Self {
        Self::resolve(None, None)
    }

    /// Get appropriate endpoint based on network type string ("mainnet" else testnet)
    pub fn for_network(&self, network_type: &str) -> &str {
        if matches!(network_type, "mainnet" | "bitcoin" | "main") {
            &self.mainnet
        } else {
            &self.testnet
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defaults_present() {
        let eps = BitcoinRpcEndpoints::resolve(None, None);
        assert_eq!(eps.mainnet, DEFAULT_MAINNET_RPC);
        assert_eq!(eps.testnet, DEFAULT_TESTNET_RPC);
    }
}
