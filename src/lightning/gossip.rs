impl LightningNode {
    pub async fn sync_network_graph(&self) -> Result<()> {
        let rapid_sync = RapidGossipSync::new(self.network_graph.clone());
        rapid_sync.sync_from_file("latest_snapshot").await
    }
} 