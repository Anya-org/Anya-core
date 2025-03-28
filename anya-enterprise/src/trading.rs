#![feature(edition2021)]
#[bitcoin_protocol]  // New attribute
impl EnterpriseTradeExecutor {
    /// Modified to include protocol validation
    pub fn execute_trade(&self, tx: BitcoinTransaction) -> Result<()> {
        // New protocol check
        if !tx.is_protocol_compliant() {
            return Err(EnterpriseError::ProtocolViolation);
        }
        
        // Existing execution logic
        self.inner_execute(tx)
    }
} 