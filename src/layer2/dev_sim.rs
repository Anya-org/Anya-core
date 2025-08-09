//! Dev simulation helpers for Layer2, gated behind the `dev-sim` feature.
//! Keeping simulate_* code isolated reduces simulation markers in production builds.

use super::production::{PeerConnection, ProductionLayer2Protocol, SyncStatus};
use super::Layer2Error;
use log::info;
use std::time::{SystemTime, UNIX_EPOCH};

impl ProductionLayer2Protocol {
    /// Dev-only connect helper (replaces previous simulate_* naming)
    pub async fn try_dev_connect(&self) -> Result<(), Layer2Error> {
        info!(
            "Simulating network connection for protocol: {}",
            self.config.protocol_type
        );

        // Create dev peers
        let simulated_peers = vec![
            PeerConnection {
                peer_id: "sim_peer_1".to_string(),
                address: "127.0.0.1:8333".to_string(),
                connected_at: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
                last_seen: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
                protocol_version: "1.0.0".to_string(),
                is_synced: true,
                latency_ms: Some(10),
                bytes_sent: 0,
                bytes_received: 0,
            },
            PeerConnection {
                peer_id: "sim_peer_2".to_string(),
                address: "127.0.0.1:8334".to_string(),
                connected_at: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
                last_seen: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
                protocol_version: "1.0.0".to_string(),
                is_synced: true,
                latency_ms: Some(15),
                bytes_sent: 0,
                bytes_received: 0,
            },
        ];

        {
            let mut peers = self.peers.write().await;
            peers.extend(simulated_peers);
        }

        // Update network state
        let mut state = self.network_state.write().await;
        state.peer_count = 2;
        state.sync_status = SyncStatus::Synced;
        state.block_height = 800000; // Simulated current height
        state.latest_block_hash =
            "0000000000000000000000000000000000000000000000000000000000000000".to_string();
        state.is_primary = self.config.prefer_self_as_master;

        Ok(())
    }
}
