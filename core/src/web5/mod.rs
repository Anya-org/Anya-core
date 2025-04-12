//! Web5 Protocol Implementation Review
#![forbid(unsafe_code)]

// Critical Security Finding: Missing BIP-275 Validation
/// New BIP-275 compliant validator
#[derive(Debug, Bip275Compliant)]
pub struct Web5ValidatorV2 {
    #[bip275(version=2)]
    psbt_support: PsbtV2,
    #[bip341]
    taproot_verifier: TaprootVerifier,
    #[bip275(fee_rate)]
    min_fee_rate: u64,
}

impl Web5ValidatorV2 {
    pub fn new() -> Self {
        Self {
            psbt_support: PsbtV2::new(),
            taproot_verifier: TaprootVerifier::new(),
            min_fee_rate: MIN_FEE_RATE,
        }
    }

    // Issue: Web5 config enables PSBTv2 but doesn't validate BIP-370
    pub fn validate_psbt(&mut self, psbt: &Psbt) -> Result<()> {
        // Enforce BIP-370
        if psbt.version < 2 {
            anyhow::bail!("PSBTv2 required (BIP-370)");
        }
        
        // Add fee rate validation
        let fee_rate = psbt.fee_rate()
            .ok_or(anyhow!("Missing fee_rate (BIP-370)"))?;
        
        if fee_rate < self.min_fee_rate {
            anyhow::bail!("Fee rate below minimum {self.min_fee_rate} sat/vB");
        }
        
        // Validate Web5 extensions
        psbt.web5_extensions.iter()
            .try_for_each(|ext| ext.validate())?;
            
        Ok(())
    }

    // Security Risk: Silent leaf pattern not enforced
    pub fn verify_taproot_commitment(&self, tx: &Transaction) -> Result<()> {
        let silent_leaf = hex::decode(BIP341_SILENT_LEAF.trim_start_matches("0x"))
            .context("Failed to decode SILENT_LEAF")?;

        tx.output.iter()
            .find(|o| o.script_pubkey.as_bytes() == silent_leaf)
            .ok_or(anyhow::anyhow!("SILENT_LEAF commitment missing"))?;

        Ok(())
    }
}

#[bip341_safe]
pub fn validate_mobile_integration(config: &MobileConfig) -> Result<()> {
    let web5 = Web5ValidatorV2::new();
    let bitcoin = BitcoinValidator::new();
    
    // Enforce BIP-275 requirements
    web5.validate_psbt(config.psbt())?;
    bitcoin.verify_taproot_commitment(config.tx())?;
    
    // Web5-specific checks
    if !config.did_manager().verify_format(DIDFormat::DIDv1) {
        anyhow::bail!("DIDv1 format required");
    }
    
    config.vc_manager().validate_format(VCFormat::VC_DATA_MODEL)?;
    
    Ok(())
}