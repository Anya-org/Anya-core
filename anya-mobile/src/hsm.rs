fn verify_fido2_signature(&self, sig: &[u8]) -> Result<()> {
    let expected = self.derive_signature()?;
    if sig.ct_ne(expected.as_ref()).into() {
        anyhow::bail!("FIDO2 signature validation failed");
    }
    Ok(())
}