// [AIR-3][AIS-3][BPC-3][AIT-3] BIP353 Beta Access Control
// Lightning Network based authorization for BIP353 beta features

use std::error::Error;
use std::fmt;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use serde::{Serialize, Deserialize};
use base64::{Engine as _, engine::general_purpose};
use sha2::{Sha256, Digest};
use uuid::Uuid;
use secp256k1::{Secp256k1, Message};
use secp256k1::ecdsa::{RecoverableSignature, RecoveryId};

use super::bip353::{Bip353Status};

/// LNURL authentication for BIP353 Beta access
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BetaAccessConfig {
    /// Whether beta access authentication is enabled
    pub enabled: bool,
    /// List of authorized public keys
    pub authorized_pubkeys: Vec<String>,
    /// Token expiration time in seconds
    pub token_expiration: u64,
    /// URL to redirect for authentication
    pub auth_url: Option<String>,
}

impl Default for BetaAccessConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            authorized_pubkeys: Vec::new(),
            token_expiration: 86400, // 24 hours
            auth_url: None,
        }
    }
}

/// Authentication session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthSession {
    /// Session ID
    pub id: String,
    /// Challenge key (k1)
    pub k1: String,
    /// Creation time
    pub created_at: u64,
    /// Expiration time
    pub expires_at: u64,
    /// Whether the session is authenticated
    pub authenticated: bool,
    /// Authenticated public key
    pub pubkey: Option<String>,
}

impl AuthSession {
    /// Create a new auth session
    pub fn new(expiration_secs: u64) -> Self {
        let id = Uuid::new_v4().to_string();
        let k1 = Uuid::new_v4().to_string();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Self {
            id,
            k1,
            created_at: now,
            expires_at: now + expiration_secs,
            authenticated: false,
            pubkey: None,
        }
    }
    
    /// Check if the session is expired
    pub fn is_expired(&self) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        self.expires_at < now
    }
}

/// Beta access errors
#[derive(Debug)]
pub enum BetaAccessError {
    /// Authentication error
    AuthError(String),
    /// Invalid session
    InvalidSession(String),
    /// Expired session
    ExpiredSession,
    /// Not authorized
    NotAuthorized,
    /// Configuration error
    ConfigError(String),
}

impl fmt::Display for BetaAccessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BetaAccessError::AuthError(msg) => write!(f, "Authentication error: {}", msg),
            BetaAccessError::InvalidSession(msg) => write!(f, "Invalid session: {}", msg),
            BetaAccessError::ExpiredSession => write!(f, "Expired session"),
            BetaAccessError::NotAuthorized => write!(f, "Not authorized"),
            BetaAccessError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
        }
    }
}

impl Error for BetaAccessError {}

/// Beta access manager
pub struct BetaAccessManager {
    config: BetaAccessConfig,
    sessions: Arc<Mutex<HashMap<String, AuthSession>>>,
    secp: Secp256k1<secp256k1::All>,
}

impl BetaAccessManager {
    /// Create a new beta access manager
    pub fn new(config: BetaAccessConfig) -> Self {
        Self {
            config,
            sessions: Arc::new(Mutex::new(HashMap::new())),
            secp: Secp256k1::new(),
        }
    }
    
    /// Create a new auth session
    pub fn create_session(&self) -> AuthSession {
        let session = AuthSession::new(self.config.token_expiration);
        
        // Store the session
        let mut sessions = self.sessions.lock().unwrap();
        sessions.insert(session.id.clone(), session.clone());
        
        // Clean up expired sessions
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        sessions.retain(|_, s| s.expires_at > now);
        
        session
    }
    
    /// Get auth URL for LNURL auth
    pub fn get_auth_url(&self, session: &AuthSession) -> Option<String> {
        if !self.config.enabled {
            return None;
        }
        
        self.config.auth_url.as_ref().map(|url| {
            format!("{}?tag=login&k1={}", url, session.k1)
        })
    }
    
    /// Verify a signed message (from LNURL-auth)
    pub fn verify_auth(&self, session_id: &str, signature: &str, pubkey: &str) -> Result<AuthSession, BetaAccessError> {
        if !self.config.enabled {
            return Err(BetaAccessError::ConfigError("Beta access auth not enabled".to_string()));
        }
        
        // Get the session
        let mut sessions = self.sessions.lock().unwrap();
        let session = sessions.get_mut(session_id).ok_or_else(|| {
            BetaAccessError::InvalidSession("Session not found".to_string())
        })?;
        
        // Check if session is expired
        if session.is_expired() {
            return Err(BetaAccessError::ExpiredSession);
        }
        
        // Decode signature and public key
        let signature_bytes = match general_purpose::STANDARD.decode(signature) {
            Ok(bytes) => bytes,
            Err(e) => return Err(BetaAccessError::AuthError(format!("Invalid signature: {}", e))),
        };
        
        let pubkey_bytes = match hex::decode(pubkey) {
            Ok(bytes) => bytes,
            Err(e) => return Err(BetaAccessError::AuthError(format!("Invalid pubkey: {}", e))),
        };
        
        // Create message from k1
        let mut hasher = Sha256::new();
        hasher.update(session.k1.as_bytes());
        let message_hash = hasher.finalize();
        
        let message = match Message::from_slice(&message_hash) {
            Ok(msg) => msg,
            Err(e) => return Err(BetaAccessError::AuthError(format!("Invalid message: {}", e))),
        };
        
        // Parse public key and signature
        let pubkey = match secp256k1::PublicKey::from_slice(&pubkey_bytes) {
            Ok(pk) => pk,
            Err(e) => return Err(BetaAccessError::AuthError(format!("Invalid public key: {}", e))),
        };
        
        // The signature format is zbase32 with recovery ID prefix
        let recovery_id = RecoveryId::from_i32(signature_bytes[0] as i32 - 31).map_err(|e| {
            BetaAccessError::AuthError(format!("Invalid recovery ID: {}", e))
        })?;
        
        let signature = match RecoverableSignature::from_compact(&signature_bytes[1..], recovery_id) {
            Ok(sig) => sig,
            Err(e) => return Err(BetaAccessError::AuthError(format!("Invalid signature: {}", e))),
        };
        
        // Verify the signature
        match self.secp.verify_ecdsa_recoverable(&message, &signature, &pubkey) {
            Ok(_) => {
                // Check if pubkey is authorized
                if !self.is_authorized(pubkey.to_string()) {
                    return Err(BetaAccessError::NotAuthorized);
                }
                
                // Mark session as authenticated
                session.authenticated = true;
                session.pubkey = Some(pubkey.to_string());
                
                Ok(session.clone())
            },
            Err(e) => Err(BetaAccessError::AuthError(format!("Signature verification failed: {}", e))),
        }
    }
    
    /// Check if a public key is authorized
    fn is_authorized(&self, pubkey: String) -> bool {
        self.config.authorized_pubkeys.contains(&pubkey)
    }
    
    /// Check if a session is valid and authenticated
    pub fn is_authenticated(&self, session_id: &str) -> bool {
        let sessions = self.sessions.lock().unwrap();
        if let Some(session) = sessions.get(session_id) {
            !session.is_expired() && session.authenticated
        } else {
            false
        }
    }
    
    /// Add an authorized public key
    pub fn add_authorized_pubkey(&mut self, pubkey: String) {
        if !self.config.authorized_pubkeys.contains(&pubkey) {
            self.config.authorized_pubkeys.push(pubkey);
        }
    }
    
    /// Remove an authorized public key
    pub fn remove_authorized_pubkey(&mut self, pubkey: &str) {
        self.config.authorized_pubkeys.retain(|p| p != pubkey);
    }
    
    /// Get current configuration
    pub fn get_config(&self) -> &BetaAccessConfig {
        &self.config
    }
    
    /// Update configuration
    pub fn update_config(&mut self, config: BetaAccessConfig) {
        self.config = config;
    }
}

/// Auth token for API requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BetaAuthToken {
    /// Session ID
    pub session_id: String,
    /// Expiration time
    pub expires_at: u64,
    /// Token creation time
    pub created_at: u64,
}

impl BetaAuthToken {
    /// Create a new auth token
    pub fn new(session_id: String, expires_at: u64) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Self {
            session_id,
            expires_at,
            created_at: now,
        }
    }
    
    /// Check if the token is expired
    pub fn is_expired(&self) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        self.expires_at < now
    }
    
    /// Encode the token as JWT-like string
    pub fn encode(&self) -> String {
        let json = serde_json::to_string(self).unwrap_or_default();
        general_purpose::STANDARD.encode(json.as_bytes())
    }
    
    /// Decode a token from string
    pub fn decode(token: &str) -> Result<Self, BetaAccessError> {
        let bytes = match general_purpose::STANDARD.decode(token) {
            Ok(bytes) => bytes,
            Err(e) => return Err(BetaAccessError::AuthError(format!("Invalid token: {}", e))),
        };
        
        let json = match String::from_utf8(bytes) {
            Ok(json) => json,
            Err(e) => return Err(BetaAccessError::AuthError(format!("Invalid token data: {}", e))),
        };
        
        let token = match serde_json::from_str::<Self>(&json) {
            Ok(token) => token,
            Err(e) => return Err(BetaAccessError::AuthError(format!("Invalid token format: {}", e))),
        };
        
        if token.is_expired() {
            return Err(BetaAccessError::ExpiredSession);
        }
        
        Ok(token)
    }
} 