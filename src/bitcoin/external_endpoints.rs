//! Central registry & resolver for non-RPC Bitcoin external endpoints (Electrum, explorers, Liquid)
//! Mirrors the precedence pattern used for RPC endpoints so the codebase is "environment-honest".
//! Order of precedence (highest first) for a given category:
//! 1. Global explicit override (e.g. BITCOIN_ELECTRUM_URL / BITCOIN_EXPLORER_API_URL)
//! 2. Legacy combined ANYA_* override (ANYA_ELECTRUM_URL / ANYA_EXPLORER_API_URL / ANYA_EXPLORER_WEB_URL)
//! 3. Network-specific overrides (e.g. ANYA_ELECTRUM_MAINNET_URL, ANYA_EXPLORER_API_TESTNET_URL)
//! 4. Config-file supplied values (not yet wired – placeholder for future integration)
//! 5. Built-in public defaults (safe, privacy‑respecting public infrastructure)
//!
//! All resolvers are infallible and will always yield usable endpoints.

use bitcoin::Network;
use std::env;

// Canonical default public endpoints
pub const DEFAULT_ELECTRUM_MAINNET: &str = "ssl://electrum.blockstream.info:50002";
pub const DEFAULT_ELECTRUM_TESTNET: &str = "ssl://electrum.blockstream.info:60002";

pub const DEFAULT_EXPLORER_API_MAINNET: &str = "https://mempool.space/api";
pub const DEFAULT_EXPLORER_API_TESTNET: &str = "https://mempool.space/testnet/api";
pub const DEFAULT_EXPLORER_WEB_MAINNET: &str = "https://mempool.space";
pub const DEFAULT_EXPLORER_WEB_TESTNET: &str = "https://mempool.space/testnet";

pub const DEFAULT_LIQUID_ASSET_REGISTRY: &str = "https://assets.blockstream.info";
pub const DEFAULT_LIQUID_FEDERATION_ENDPOINT: &str = "https://blockstream.info/liquid/api";

#[derive(Debug, Clone)]
pub struct ExternalBitcoinEndpoints {
    pub electrum_mainnet: String,
    pub electrum_testnet: String,
    pub explorer_api_mainnet: String,
    pub explorer_api_testnet: String,
    pub explorer_web_mainnet: String,
    pub explorer_web_testnet: String,
    pub liquid_asset_registry: String,
    pub liquid_federation_endpoint: String,
}

impl ExternalBitcoinEndpoints {
    /// Resolve all external endpoints using environment variable precedence.
    pub fn resolve() -> Self {
        // Electrum
        let electrum_global = env::var("BITCOIN_ELECTRUM_URL")
            .or_else(|_| env::var("ANYA_ELECTRUM_URL"))
            .ok();
        let electrum_mainnet = electrum_global
            .clone()
            .or_else(|| env::var("ANYA_ELECTRUM_MAINNET_URL").ok())
            .unwrap_or_else(|| DEFAULT_ELECTRUM_MAINNET.to_string());
        let electrum_testnet = electrum_global
            .or_else(|| env::var("ANYA_ELECTRUM_TESTNET_URL").ok())
            .unwrap_or_else(|| DEFAULT_ELECTRUM_TESTNET.to_string());

        // Explorer API
        let explorer_api_global = env::var("BITCOIN_EXPLORER_API_URL")
            .or_else(|_| env::var("ANYA_EXPLORER_API_URL"))
            .ok();
        let explorer_api_mainnet = explorer_api_global
            .clone()
            .or_else(|| env::var("ANYA_EXPLORER_API_MAINNET_URL").ok())
            .unwrap_or_else(|| DEFAULT_EXPLORER_API_MAINNET.to_string());
        let explorer_api_testnet = explorer_api_global
            .or_else(|| env::var("ANYA_EXPLORER_API_TESTNET_URL").ok())
            .unwrap_or_else(|| DEFAULT_EXPLORER_API_TESTNET.to_string());

        // Explorer Web
        let explorer_web_global = env::var("BITCOIN_EXPLORER_WEB_URL")
            .or_else(|_| env::var("ANYA_EXPLORER_WEB_URL"))
            .ok();
        let explorer_web_mainnet = explorer_web_global
            .clone()
            .or_else(|| env::var("ANYA_EXPLORER_WEB_MAINNET_URL").ok())
            .unwrap_or_else(|| DEFAULT_EXPLORER_WEB_MAINNET.to_string());
        let explorer_web_testnet = explorer_web_global
            .or_else(|| env::var("ANYA_EXPLORER_WEB_TESTNET_URL").ok())
            .unwrap_or_else(|| DEFAULT_EXPLORER_WEB_TESTNET.to_string());

        // Liquid endpoints
        let liquid_asset_registry = env::var("ANYA_LIQUID_ASSET_REGISTRY_URL")
            .unwrap_or_else(|_| DEFAULT_LIQUID_ASSET_REGISTRY.to_string());
        let liquid_federation_endpoint = env::var("ANYA_LIQUID_FEDERATION_ENDPOINT_URL")
            .unwrap_or_else(|_| DEFAULT_LIQUID_FEDERATION_ENDPOINT.to_string());

        Self {
            electrum_mainnet,
            electrum_testnet,
            explorer_api_mainnet,
            explorer_api_testnet,
            explorer_web_mainnet,
            explorer_web_testnet,
            liquid_asset_registry,
            liquid_federation_endpoint,
        }
    }

    pub fn electrum_for(&self, network: &Network) -> &str {
        match network {
            Network::Bitcoin => &self.electrum_mainnet,
            Network::Testnet => &self.electrum_testnet,
            _ => &self.electrum_testnet,
        }
    }

    pub fn explorer_api_for(&self, network: &Network) -> &str {
        match network {
            Network::Bitcoin => &self.explorer_api_mainnet,
            Network::Testnet => &self.explorer_api_testnet,
            _ => &self.explorer_api_testnet,
        }
    }

    pub fn explorer_web_for(&self, network: &Network) -> &str {
        match network {
            Network::Bitcoin => &self.explorer_web_mainnet,
            Network::Testnet => &self.explorer_web_testnet,
            _ => &self.explorer_web_testnet,
        }
    }
}

// Backwards compatible free functions (deprecated internally – prefer the resolver instance)
pub fn electrum_for(network: &Network) -> String {
    ExternalBitcoinEndpoints::resolve().electrum_for(network).to_string()
}
pub fn explorer_api_for(network: &Network) -> String {
    ExternalBitcoinEndpoints::resolve().explorer_api_for(network).to_string()
}
pub fn explorer_web_for(network: &Network) -> String {
    ExternalBitcoinEndpoints::resolve().explorer_web_for(network).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_defaults_present() {
        let eps = ExternalBitcoinEndpoints::resolve();
        assert!(eps.electrum_mainnet.contains("electrum."));
        assert!(eps.explorer_api_mainnet.contains("mempool.space"));
    }

    #[test]
    fn test_env_override_electrum() {
        env::set_var("BITCOIN_ELECTRUM_URL", "ssl://example.org:50001");
        let eps = ExternalBitcoinEndpoints::resolve();
        assert_eq!(eps.electrum_mainnet, "ssl://example.org:50001");
        assert_eq!(eps.electrum_testnet, "ssl://example.org:50001");
        env::remove_var("BITCOIN_ELECTRUM_URL");
    }
}
