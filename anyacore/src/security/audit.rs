#[derive(Debug, AIS3)]
pub struct AuditRunner {
    #[bip341]
    hsm_conn: HsmConnection,
    #[ais3]
    strict_mode: bool,
}

impl AuditRunner {
    pub fn new() -> Result<Self> {
        Ok(Self {
            hsm_conn: HsmConnection::establish()?,
            strict_mode: true,
        })
    }

    #[bpc3]
    pub fn validate_secrets(&self) -> Result<()> {
        let secrets = self.hsm_conn.list_keys()?;
        secrets.iter()
            .try_for_each(|k| k.validate_compliance())
            .map_err(|e| Error::AuditFailure(e.into()))
    }
}

#[derive(Debug, AIS3)]
pub struct ResearchValidator {
    #[bip341]
    hsm_conn: HsmConnection,
}

impl ResearchValidator {
    pub fn validate_tier(&self, tier: CodeTier) -> Result<()> {
        match tier {
            CodeTier::Core => self.validate_core_rules(),
            CodeTier::Project => self.validate_project_rules(),
            CodeTier::Experimental => Ok(()),
        }
    }

    #[bpc3]
    fn validate_core_rules(&self) -> Result<()> {
        // Core validation logic
        self.hsm_conn.verify_compliance(BitcoinStandard::V2_5)
    }
} 