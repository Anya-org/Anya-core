use std::error::Error;

pub mod advanced_analytics;
pub mod high_volume_trading;
pub mod ml;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// Initialize the anya-enterprise library
pub fn init() -> Result<()> {
    // Initialize logging
    env_logger::init();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        assert!(init().is_ok());
    }
}

#[cfg(feature = "enterprise")]
pub fn process_enterprise_tx(tx: BitcoinTransaction) -> Result<()> {
    // New protocol check
    if !tx.is_protocol_compliant() {
        return Err(EnterpriseError::ProtocolViolation);
    }
    
    // Existing processing logic
    internal_tx_processor(tx)
}
