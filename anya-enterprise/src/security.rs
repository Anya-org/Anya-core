pub struct EnterpriseSecurity {
    hsm: HsmClient,
    audit: AuditLogger,
}

impl EnterpriseSecurity {
    pub fn verify_taproot(&self, tx: &Transaction) -> Result<()> {
        self.hsm.verify_taproot(tx)?;
        self.audit.log_verification(tx);
        Ok(())
    }
}