use std::sync::Arc;
use anyhow::Result;
use rand::Rng;
use sha3::Digest;

pub struct QuantumResistantCrypto {
    private_key: Vec<u8>,
    public_key: Vec<u8>,
    key_size: usize,
}

impl QuantumResistantCrypto {
    pub fn new() -> Self {
        let key_size = 32; // 256-bit keys
        let mut rng = rand::thread_rng();
        let private_key: Vec<u8> = (0..key_size).map(|_| rng.gen()).collect();
        let public_key = Self::generate_public_key(&private_key);
        
        Self {
            private_key,
            public_key,
            key_size,
        }
    }

    fn generate_public_key(private_key: &[u8]) -> Vec<u8> {
        let mut hasher = sha3::Sha3_256::new();
        hasher.update(private_key);
        hasher.finalize().to_vec()
    }

    pub fn sign(&self, message: &[u8]) -> Result<Vec<u8>> {
        let mut hasher = sha3::Sha3_256::new();
        hasher.update(message);
        let digest = hasher.finalize();
        
        // Simple deterministic signature (in practice, use a proper signature scheme)
        let mut signature = Vec::with_capacity(self.key_size * 2);
        signature.extend_from_slice(&digest[..self.key_size]);
        signature.extend_from_slice(&self.private_key);
        
        Ok(signature)
    }

    pub fn verify(&self, message: &[u8], signature: &[u8]) -> Result<bool> {
        if signature.len() != self.key_size * 2 {
            return Ok(false);
        }

        let mut hasher = sha3::Sha3_256::new();
        hasher.update(message);
        let digest = hasher.finalize();
        
        // Verify the first half of the signature matches the message digest
        let expected_digest = &signature[..self.key_size];
        if expected_digest != digest.as_slice() {
            return Ok(false);
        }

        // Verify the second half matches the public key
        let expected_public_key = &signature[self.key_size..];
        Ok(expected_public_key == self.public_key)
    }

    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>> {
        let nonce: [u8; 12] = rand::random();
        let mut cipher = aes_gcm::Aes256Gcm::new_from_slice(&self.private_key)?;
        
        let ciphertext = cipher.encrypt(&nonce, plaintext)?;
        let mut encrypted = Vec::with_capacity(nonce.len() + ciphertext.len());
        encrypted.extend_from_slice(&nonce);
        encrypted.extend_from_slice(&ciphertext);
        
        Ok(encrypted)
    }

    pub fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>> {
        if ciphertext.len() < 12 {
            return Err(anyhow::anyhow!("Ciphertext too short"));
        }

        let nonce = &ciphertext[..12];
        let ciphertext = &ciphertext[12..];
        let mut cipher = aes_gcm::Aes256Gcm::new_from_slice(&self.private_key)?;
        
        Ok(cipher.decrypt(nonce, ciphertext)?)
    }
}
