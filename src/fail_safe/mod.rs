#[anya_fail_safe]
impl AnyaFailSafe {
    pub fn emergency_rollback(&self) -> Result<BlockHeight, FailSafeError> {
        // Preserved DeepSeek rollback logic
        let last_valid = self.consensus_interface.last_valid_block()?;
        let utxo_snapshot = rebuild_utxo_set(last_valid)?;
        
        self.prediction_engine.reset_to_checkpoint(
            utxo_snapshot.model_checkpoint
        )?;
        
        let activation_tx = create_psbt_activation(
            &self.backup_nodes,
            ActivationType::EmergencyOverride
        )?;
        
        Ok(last_valid)
    }
} 