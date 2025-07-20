use crate::web::web5_adapter::Web5Adapter;
// ...existing code...
pub async fn handle_did_creation(request: Web5Request) -> Result<DidDocument> {
    // Example: Use the async HTTP-based adapter for DID creation
    let adapter = Web5Adapter::new("http://localhost:8080"); // Service URL should be configurable
    let did_doc = adapter.create_did("bip340").await
        .map_err(|e| Error::Web5Error(e.to_string()))?;
    // Convert DidDocumentResponse to DidDocument as needed
    Ok(did_doc.document)
}
