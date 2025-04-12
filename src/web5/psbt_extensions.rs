#[derive(Debug, Serialize, Deserialize)]
pub struct Web5PsbtExtension {
    #[serde(rename = "web5:context")]
    pub context: String,
    #[serde(rename = "web5:credential")]
    pub verifiable_credential: Option<Jwt>,
    #[serde(rename = "web5:proof")]
    pub linked_data_proof: LinkedDataProof,
}

impl Web5PsbtExtension {
    pub fn validate(&self) -> Result<()> {
        // BIP-174/Web5 validation
        verify_web5_signature_scheme(&self.linked_data_proof)?;
        Ok(())
    }

    pub fn validate_vc(&self) -> Result<()> {
        use ssi::vc::Credential;
        
        if let Some(vc) = &self.verifiable_credential {
            let credential = Credential::from_jwt(vc)
                .map_err(|e| anyhow!("Invalid VC format: {}", e))?;
            
            if !credential.validate_structure() {
                anyhow::bail!("VC doesn't conform to W3C VC-DATA-MODEL");
            }
        }
        Ok(())
    }
} 