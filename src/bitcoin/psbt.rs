#![feature(edition2021)]
// [BPC-3][BIP-370] PSBTv2 Implementation
impl PsbtV2 {
    pub fn validate(&self) -> Result<(), PsbtError> {
        // Validate global transaction presence
        require!(self.global_tx.is_some(), PsbtError::MissingGlobalTx);
        
        // Validate input/output counts match metadata
        require!(
            self.inputs.len() == self.input_count as usize,
            PsbtError::InputCountMismatch
        );
        require!(
            self.outputs.len() == self.output_count as usize,
            PsbtError::OutputCountMismatch
        );
        
        // Validate version 2 requirement
        require!(self.version == 2, PsbtError::InvalidVersion);
        
        Ok(())
    }
}

impl Psbt {
    fn validate_fee(&self) -> Result<()> {
        // BIP-370 compliant fee validation
        let expected_fee = self.calculate_expected_fee()?;
        
        // Enforce minimum fee rate (1.0 sat/vByte)
        if self.fee_rate < MIN_RELAY_FEE_RATE {
            return Err(PsbtError::InsufficientFee.into());
        }
        
        // Allow 1% tolerance for fee validation
        let fee_difference = (self.actual_fee as f64 - expected_fee as f64).abs();
        if (fee_difference / expected_fee as f64) > 0.01 {
            return Err(PsbtError::FeeValidationFailed.into());
        }

        Ok(())
    }
} 