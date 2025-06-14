//! Bitcoin consensus parameters
//!
//! This module defines the consensus parameters used in Bitcoin's consensus rules,
//! following the Bitcoin Core implementation's standards and values.
//! These parameters can vary depending on the network (mainnet, testnet, etc.)

use bitcoin::Network;
use std::collections::HashMap;
use std::sync::Arc;

// Define our own Uint256 placeholder for now
// Later this should be properly integrated with the bitcoin crate
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Uint256(pub [u8; 32]);

impl Uint256 {
    pub fn from_u64(val: u64) -> Option<Self> {
        let mut bytes = [0u8; 32];
        bytes[24..32].copy_from_slice(&val.to_be_bytes());
        Some(Uint256(bytes))
    }

    pub fn from_be_bytes(bytes: [u8; 32]) -> Self {
        Uint256(bytes)
    }

    pub fn hex(&self) -> String {
        hex::encode(self.0)
    }
}

impl std::ops::Sub for Uint256 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        // Simple implementation for now, assuming we only use this for target calculation
        let mut result = self.0;
        // Implement basic subtraction (this is a simplified version)
        let mut borrow = 0u8;
        for i in (0..32).rev() {
            let diff = if self.0[i] >= (rhs.0[i] + borrow) {
                let diff = self.0[i] - rhs.0[i] - borrow;
                borrow = 0;
                diff
            } else {
                let diff = 256 + self.0[i] as u16 - rhs.0[i] as u16 - borrow as u16;
                borrow = 1;
                diff as u8
            };
            result[i] = diff;
        }
        Uint256(result)
    }
}

impl std::ops::Shr<u32> for Uint256 {
    type Output = Self;

    fn shr(self, rhs: u32) -> Self::Output {
        let mut result = [0u8; 32];
        let bytes_shift = (rhs / 8) as usize;
        let bits_shift = (rhs % 8) as usize;

        if bytes_shift >= 32 {
            return Uint256(result);
        }

        if bits_shift == 0 {
            // Simple case: just shift whole bytes
            result[bytes_shift..].copy_from_slice(&self.0[0..(32 - bytes_shift)]);
        } else {
            // Need to handle bit-level shifting
            for i in bytes_shift..32 {
                let src_idx = i - bytes_shift;
                let high_bits = if src_idx > 0 {
                    self.0[src_idx - 1] << (8 - bits_shift)
                } else {
                    0
                };
                let low_bits = self.0[src_idx] >> bits_shift;
                result[i] = high_bits | low_bits;
            }
        }

        Uint256(result)
    }
}

impl std::fmt::Display for Uint256 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "0x{}..{}",
            hex::encode(&self.0[0..4]),
            hex::encode(&self.0[28..32])
        )
    }
}

/// Consensus parameters for Bitcoin network
#[derive(Debug, Clone)]
pub struct ConsensusParams {
    /// Bitcoin network type (mainnet, testnet, etc.)
    pub network: Network,

    /// Maximum block weight (in weight units)
    pub max_block_weight: u32,

    /// Maximum block size (legacy, in bytes)
    pub max_block_size: u32,

    /// Minimum required proof of work difficulty
    pub pow_limit: Uint256,

    /// Whether Taproot is activated
    pub taproot_active: bool,

    /// Block height at which Taproot activated
    pub taproot_activation_height: u32,

    /// Custom parameters with string keys for extensibility
    pub custom_params: HashMap<String, String>,
}

impl ConsensusParams {
    /// Create a new set of consensus parameters for the specified network
    pub fn new(network: Network) -> Self {
        match network {
            Network::Bitcoin => Self::mainnet(),
            Network::Testnet => Self::testnet(),
            Network::Signet => Self::signet(),
            Network::Regtest => Self::regtest(),
            _ => Self::mainnet(), // Default to mainnet for unknown networks
        }
    }

    /// Get mainnet consensus parameters
    pub fn mainnet() -> Self {
        ConsensusParams {
            network: Network::Bitcoin,
            max_block_weight: 4_000_000, // 4M weight units
            max_block_size: 1_000_000,   // Legacy 1MB limit
            pow_limit: Uint256::from_u64(0).unwrap() - (Uint256::from_u64(1).unwrap() >> 32), // Mainnet target
            taproot_active: true,
            taproot_activation_height: 709_632, // Actual mainnet Taproot activation height
            custom_params: HashMap::new(),
        }
    }

    /// Get testnet consensus parameters
    pub fn testnet() -> Self {
        ConsensusParams {
            network: Network::Testnet,
            max_block_weight: 4_000_000, // Same as mainnet
            max_block_size: 1_000_000,   // Same as mainnet
            pow_limit: Uint256::from_u64(0).unwrap() - (Uint256::from_u64(1).unwrap() >> 28), // Easier than mainnet
            taproot_active: true,
            taproot_activation_height: 2_010_000, // Testnet Taproot activation height
            custom_params: HashMap::new(),
        }
    }

    /// Get signet consensus parameters
    pub fn signet() -> Self {
        ConsensusParams {
            network: Network::Signet,
            max_block_weight: 4_000_000,
            max_block_size: 1_000_000,
            pow_limit: Uint256::from_u64(0).unwrap() - (Uint256::from_u64(1).unwrap() >> 30),
            taproot_active: true,
            taproot_activation_height: 43, // Signet Taproot activation height
            custom_params: HashMap::new(),
        }
    }

    /// Get regtest consensus parameters
    pub fn regtest() -> Self {
        ConsensusParams {
            network: Network::Regtest,
            max_block_weight: 4_000_000,
            max_block_size: 1_000_000,
            pow_limit: Uint256::from_u64(0).unwrap() - (Uint256::from_u64(1).unwrap() >> 24), // Very easy
            taproot_active: true,
            taproot_activation_height: 0, // Always active in regtest
            custom_params: HashMap::new(),
        }
    }

    /// Check if Taproot is active at the given height
    pub fn is_taproot_active(&self, height: u32) -> bool {
        self.taproot_active && height >= self.taproot_activation_height
    }

    /// Add a custom parameter
    pub fn add_custom_param(&mut self, key: &str, value: &str) {
        self.custom_params
            .insert(key.to_string(), value.to_string());
    }

    /// Get a custom parameter value
    pub fn get_custom_param(&self, key: &str) -> Option<&String> {
        self.custom_params.get(key)
    }
}

/// Provides thread-safe access to consensus parameters
#[derive(Clone)]
pub struct ConsensusParamsProvider {
    params: Arc<ConsensusParams>,
}

impl ConsensusParamsProvider {
    /// Create a new provider for the specified network
    pub fn new(network: Network) -> Self {
        Self {
            params: Arc::new(ConsensusParams::new(network)),
        }
    }

    /// Get the consensus parameters
    pub fn get_params(&self) -> Arc<ConsensusParams> {
        self.params.clone()
    }

    /// Create a new provider with custom parameters
    pub fn with_custom_params(
        network: Network,
        custom_fn: impl FnOnce(&mut ConsensusParams),
    ) -> Self {
        let mut params = ConsensusParams::new(network);
        custom_fn(&mut params);
        Self {
            params: Arc::new(params),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consensus_parameters() {
        let mainnet = ConsensusParams::mainnet();
        assert_eq!(mainnet.network, Network::Bitcoin);
        assert_eq!(mainnet.max_block_weight, 4_000_000);
        assert!(mainnet.is_taproot_active(800_000));
        assert!(!mainnet.is_taproot_active(700_000));

        let testnet = ConsensusParams::testnet();
        assert_eq!(testnet.network, Network::Testnet);

        // Test custom parameters
        let mut params = ConsensusParams::mainnet();
        params.add_custom_param("max_ancestors", "25");
        assert_eq!(
            params.get_custom_param("max_ancestors"),
            Some(&"25".to_string())
        );
    }

    #[test]
    fn test_consensus_params_provider() {
        let provider = ConsensusParamsProvider::new(Network::Bitcoin);
        let params = provider.get_params();
        assert_eq!(params.network, Network::Bitcoin);

        // Test with custom parameters
        let custom_provider = ConsensusParamsProvider::with_custom_params(Network::Testnet, |p| {
            p.max_block_weight = 8_000_000
        });
        assert_eq!(custom_provider.get_params().max_block_weight, 8_000_000);
    }
}
