use std::error::Error;
pub struct AuditBuilder {
    timestamp: u64,
    bip_compliance: BIPCompliance,
    security_status: SecurityStatus,
}

impl AuditBuilder {
    pub fn new() -> Self  -> Result<(), Box<dyn Error>> {
        Self {
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
            bip_compliance: BIPCompliance::default(),
            security_status: SecurityStatus::default(),
        }
    }

    pub fn with_bip(mut self, bip: &str, status: ComplianceStatus) -> Self  -> Result<(), Box<dyn Error>> {
        match bip {
            "BIP-341" => self.bip_compliance.bip341 = status,
            "BIP-174" => self.bip_compliance.bip174 = status,
            // ... other BIPs
        }
        self
    }

    pub fn build(self) -> InstallationAudit  -> Result<(), Box<dyn Error>> {
        InstallationAudit {
            timestamp: self.timestamp,
            bip_compliance: self.bip_compliance,
            security_status: self.security_status,
            file_manifest: vec![],
        }
    }
} 
