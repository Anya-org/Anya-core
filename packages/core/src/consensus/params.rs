//! Network parameters for Bitcoin
//! This module defines parameters specific to different Bitcoin networks

use bitcoin::Network;

/// Get the maximum block size for a given network
pub fn get_max_block_size(network: Network) -> usize {
    match network {
        Network::Bitcoin => 1_000_000, // 1MB for main network
        Network::Testnet | Network::Signet => 1_000_000,
        Network::Regtest => 1_000_000,
        _ => 1_000_000, // Default for unknown networks
    }
}

/// Get the maximum block weight for a given network (BIP-141)
pub fn get_max_block_weight(network: Network) -> usize {
    match network {
        Network::Bitcoin => 4_000_000, // 4MB weight for main network
        Network::Testnet | Network::Signet => 4_000_000,
        Network::Regtest => 4_000_000,
        _ => 4_000_000, // Default for unknown networks
    }
}
