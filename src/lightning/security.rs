impl LightningNode {
    // BDF v2.5 §7.3: Static Key Security
    pub fn rotate_node_keys(&self) -> Result<()> {
        let new_secret = self.keys_manager.get_new_node_secret();
        self.channel_manager.update_node_secret(new_secret)
    }

    // BDF v2.5 §9.1: Watchtower Requirements
    pub fn validate_watchtower_coverage(&self) -> Result<()> {
        for channel in self.channel_manager.list_channels() {
            if channel.watchtower_registration.is_none() {
                return Err(anyhow!("Unprotected channel {}", channel.channel_id));
            }
        }
        Ok(())
    }

    // BDF v2.5 §12.3: Offer Validation
    pub fn validate_offer(&self, offer: &Bolt12Offer) -> Result<()> {
        // Verify signature and metadata
        offer.inner.verify(&self.network_graph)?;
        
        // Check against spam limits
        if offer.metadata.created_at < unix_time() - MAX_OFFER_AGE {
            return Err(anyhow!("Expired offer"));
        }
        
        Ok(())
    }

    // BOLT 14: Jamming Protection
    pub fn apply_anti_jamming(&self, channel_id: ChannelId) -> Result<()> {
        let config = AntiJammingConfig {
            min_funding: MIN_CHANNEL_SATS,
            max_htlc_count: MAX_HTLC_COUNT,
            fee_rate: self.fee_estimator.get_est_sat_per_1000_weight(),
        };
        
        self.channel_manager.update_channel_config(channel_id, config)
    }
} 