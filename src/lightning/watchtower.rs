use std::error::Error;
impl LightningNode {
    pub fn register_watchtower(&self, tower_pubkey: PublicKey) -> Result<()> {
        let tower_client = WatchtowerClient::new(tower_pubkey);
        self.channel_manager.add_watchtower(tower_client)
    }
} 
