use std::error::Error;
// Web5 example with Bitcoin anchoring

#[cfg(feature = "web5")]
pub mod web5_examples {
    use crate::bitcoin::transaction::Transaction;
    use crate::bitcoin::taproot::TaprootTree;
    use crate::web5::credential::CredentialManager;
    use crate::web5::did::DidManager;
    use crate::web5::dwn::DwnManager;
    use bitcoin::Network;
    use std::collections::HashMap;
    use serde_json::Value;
    use std::sync::Arc;

    // Example: Create and verify a credential with Bitcoin anchoring
    #[allow(dead_code)]
    pub async fn create_anchored_credential() -> anyhow::Result<()>  -> Result<(), Box<dyn Error>> {
        // Initialize managers
        let did_manager = Arc::new(DidManager::new_in_memory()?);
        let dwn_manager = DwnManager::new_in_memory();
        let credential_manager = CredentialManager::new_with_anchoring(
            did_manager.clone(),
            dwn_manager.clone(),
            Network::Testnet,
        );

        // Create DIDs for issuer and subject
        let issuer_did = did_manager.create_key_did().await?;
        let subject_did = did_manager.create_key_did().await?;

        // Create credential claims
        let mut claims = HashMap::new();
        claims.insert("name".to_string(), Value::String("Alice".to_string()));
        claims.insert("age".to_string(), Value::Number(25.into()));
        claims.insert("isVerified".to_string(), Value::Bool(true));

        // Issue credential with Bitcoin anchoring
        let credential = credential_manager.issue_anchored_credential(
            &issuer_did,
            &subject_did,
            "IdentityCredential",
            claims,
            Some(365), // Valid for 1 year
        ).await?;

        // Verify the credential has Bitcoin anchoring information
        assert!(credential.bitcoin_anchoring.is_some());
        
        // Verify the credential
        let is_valid = credential_manager.verify_credential(&credential).await?;
        assert!(is_valid);
        
        println!("Credential created and verified with Bitcoin anchoring!");
        println!("Anchoring transaction ID: {}", credential.bitcoin_anchoring?.txid);
        
        Ok(())
    }

    // Example: Store data in DWN with Bitcoin anchoring
    #[allow(dead_code)]
    pub async fn store_data_with_anchoring() -> anyhow::Result<()>  -> Result<(), Box<dyn Error>> {
        // Initialize managers
        let did_manager = Arc::new(DidManager::new_in_memory()?);
        let dwn_manager = DwnManager::new_in_memory();
        
        // Create a DID
        let user_did = did_manager.create_key_did().await?;
        
        // Store data in DWN with Bitcoin anchoring
        let data = serde_json::json!({
            "title": "Important Document",
            "content": "This is a secure document anchored to Bitcoin",
            "timestamp": chrono::Utc::now().to_rfc3339()
        });
        
        let record_id = dwn_manager.store_anchored_data(
            &user_did, 
            "secureDocument",
            data.to_string().as_bytes().to_vec(),
            Network::Testnet
        ).await?;
        
        // Retrieve the data with verification
        let (retrieved_data, anchoring_info) = dwn_manager.get_anchored_data_with_proof(
            &user_did,
            &record_id
        ).await?;
        
        // Verify the anchoring information
        assert!(anchoring_info.is_some());
        
        println!("Data stored and retrieved with Bitcoin anchoring!");
        println!("Anchoring transaction ID: {}", anchoring_info?.txid);
        
        let content = String::from_utf8(retrieved_data)?;
        println!("Retrieved content: {}", content);
        
        Ok(())
    }
}

