pub trait BipValidator {
    fn validate_bip(&self, bip: &str) -> Result<ComplianceStatus>;
}

impl BipValidator for BitcoinConfig {
    fn validate_bip(&self, bip: &str) -> Result<ComplianceStatus> {
        match bip {
            "BIP-341" => Ok(self.taproot_enabled.into()),
            "BIP-174" => Ok((self.psbt_version >= 2).into()),
            "BIP-342" => Ok(self.tapscript_enabled.into()),
            "BIP-370" => Ok((self.psbt_version >= 2).into()),
            _ => Ok(ComplianceStatus::Missing)
        }
    }
} 