use super::*;
use crate::security::secrets::SecretsManager;

pub struct CredentialVerificationService {
    secrets: SecretsManager,
    verifiers: HashMap<String, Box<dyn CredentialVerifier>>
}

impl CredentialVerificationService {
    pub fn new(secrets: SecretsManager) -> Self {
        let mut verifiers = HashMap::new();
        verifiers.insert(
            "wallet".to_string(),
            Box::new(WalletVerifier::new()) as Box<dyn CredentialVerifier>
        );
        // Add other verifiers...
        
        Self {
            secrets,
            verifiers
        }
    }
    
    pub fn verify_credential(&self, cred: &Credential) -> Result<bool> {
        // Basic checks
        if !self.verify_timestamps(cred)? {
            return Ok(false);
        }
        
        if !self.verify_status(cred)? {
            return Ok(false);
        }
        
        // Get appropriate verifier
        let verifier = self.get_verifier(&cred.cred_type)?;
        
        // Verify using specific verifier
        verifier.verify(cred)
    }
    
    fn verify_timestamps(&self, cred: &Credential) -> Result<bool> {
        let now = OffsetDateTime::now_utc();
        
        // Check if expired
        if let Some(expires) = cred.expires_at {
            if expires < now {
                return Ok(false);
            }
        }
        
        // Check if issued in future
        if cred.issued_at > now {
            return Ok(false);
        }
        
        Ok(true)
    }
    
    fn verify_status(&self, cred: &Credential) -> Result<bool> {
        match cred.status {
            CredentialStatus::Active => Ok(true),
            _ => Ok(false)
        }
    }
    
    fn get_verifier(&self, cred_type: &CredentialType) -> Result<&Box<dyn CredentialVerifier>> {
        let key = match cred_type {
            CredentialType::Wallet(_) => "wallet",
            CredentialType::Web5(_) => "web5", 
            CredentialType::Lightning(_) => "lightning",
            CredentialType::Development(_) => "development"
        };
        
        self.verifiers.get(key)
            .ok_or_else(|| anyhow::anyhow!("No verifier found for credential type"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn test_verification_flow() {
        let secrets = SecretsManager::new().unwrap();
        let service = CredentialVerificationService::new(secrets);
        
        let cred = create_test_credential();
        assert!(service.verify_credential(&cred).unwrap());
        
        let expired = create_expired_credential();
        assert!(!service.verify_credential(&expired).unwrap());
    }

    fn create_test_credential() -> Credential {
        // Test helper to create valid credential
        todo!()
    }

    fn create_expired_credential() -> Credential {
        // Test helper to create expired credential
        todo!() 
    }
}
