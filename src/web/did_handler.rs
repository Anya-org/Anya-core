// ... existing code ...
pub async fn handle_did_creation(request: Web5Request) -> Result<DidDocument> {
    // Add BIP-340 based DID method
    let did_method = "bip340".to_string();
    let public_key = generate_schnorr_key().await?;
    let did = web5_did::bip340::create_did(public_key)
        .map_err(|e| Error::Web5Error(e.to_string()))?;
    Ok(did)
}