//! Web5 Protocol Implementation Review
#![forbid(unsafe_code)]

// Critical Security Finding: Missing BIP-275 Validation
#[derive(Debug)]
pub struct Web5Validator {
    bip174_compliant: bool,
    taproot_enabled: bool,
    silent_leaf_verified: bool,
}

impl Web5Validator {
    pub fn new() -> Self {
        Self {
            bip174_compliant: true,
            taproot_enabled: true,
            silent_leaf_verified: true,
        }
    }

    // Issue: Web5 config enables PSBTv2 but doesn't validate BIP-370
    pub fn validate_psbt(&mut self, psbt: &Psbt) -> Result<()> {
        if psbt.version < 2 {
            anyhow::bail!("PSBT version 2 required for Web5 transactions");
        }

        // Add BIP-370 fee rate validation
        let fee_rate = psbt
            .fee_rate()
            .ok_or(anyhow::anyhow!("Missing fee rate in PSBT"))?;

        if fee_rate < 1.0 {
            anyhow::bail!("Fee rate below minimum required (1 sat/vB)");
        }

        Ok(())
    }

    // Security Risk: Silent leaf pattern not enforced
    pub fn verify_taproot_commitment(&self, tx: &Transaction) -> Result<()> {
        let silent_leaf = hex::decode(BIP341_SILENT_LEAF.trim_start_matches("0x"))
            .context("Failed to decode SILENT_LEAF")?;

        tx.output
            .iter()
            .find(|o| o.script_pubkey.as_bytes() == silent_leaf)
            .ok_or(anyhow::anyhow!("SILENT_LEAF commitment missing"))?;

        Ok(())
    }
}
