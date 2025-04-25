use std::error::Error;
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
