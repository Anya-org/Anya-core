#![feature(edition2021)]
#[derive(Debug, BPC3)]
pub struct FullCompliance {
    #[bip341]
    taproot: TaprootVerifier,
    #[bip342]
    tapscript: TapscriptEngine,
    #[ais3]
    security: HsmSecurityLayer,
}

impl FullCompliance {
    pub fn new() -> Result<Self> {
        Ok(Self {
            taproot: TaprootVerifier::with_global_context()?,
            tapscript: TapscriptEngine::new()?,
            security: HsmSecurityLayer::establish()?,
        })
    }

    #[bip370]
    pub fn validate_psbt_v2(&self, psbt: &Psbt) -> Result<()> {
        self.taproot.verify_psbt_structure(psbt)?;
        self.tapscript.validate_scripts(psbt)?;
        self.security.verify_signatures(psbt)
    }
} 