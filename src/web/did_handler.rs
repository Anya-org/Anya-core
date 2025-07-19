use std::error::Error;
// ... existing code ...
pub async fn handle_did_creation(request: Web5Request) -> Result<DidDocument> {
    // v2.0.0-beta9: Use new web5-rust API for DID creation
    use web5::did::{DIDMethod, DIDBuilder};
    let did_method = DIDMethod::Bip340;
    let public_key = generate_schnorr_key().await?;
    let did = DIDBuilder::new()
        .method(did_method)
        .public_key(public_key)
        .build()
        .await
        .map_err(|e| Error::Web5Error(e.to_string()))?;
    Ok(did)
}
