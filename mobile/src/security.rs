#![feature(edition2021)]
pub fn verify_psbt_signature(&self, psbt: &Psbt) -> Result<()> {
    let local_sig = self.derive_signature(psbt)
        .context("Failed to derive local signature")?;
    let external_sig = external::sign_psbt(psbt)
        .context("Failed to get external signature")?;
    
    if local_sig.ct_ne(&external_sig).into() {
        anyhow::bail!("Signature mismatch between implementations");
    }
    
    Ok(())
} 