#[bitcoin_network_aware]
pub fn train_model(
    dataset: BlockchainDataset,
    consensus_rules: Arc<ConsensusRules>
) -> Result<ModelUpdate, AiError> {
    let mut trainer = FederatedTrainer::new(
        dataset,
        ConsensusUpdatePolicy::new(consensus_rules)
    ).with_anya_optimizer();
    
    trainer.apply_constraints(|params| {
        params.validate_taproot_compatibility()?;
        params.ensure_psbt_safety()?;
        params.enforce_bip370()?;
        Ok(())
    });
    
    let mut metrics = TrainingMetrics::new();
    let update = trainer.run_epochs(5, &mut metrics)?;
    
    metrics.export_to_prometheus();
    Ok(update)
} 