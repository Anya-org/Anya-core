use std::collections::HashSet;
use serde::{Serialize, Deserialize};

// Define the KeyType enum that was missing
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum KeyType {
    Secp256k1,
    Ed25519,
    Schnorr,
}

// Define the default functions needed for serde defaults
fn default_key_lifetime() -> u16 { 90 }
fn default_hsm_threshold() -> u8 { 2 }

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SecurityConfig {
    #[serde(default = "default_key_lifetime")]
    pub key_rotation_days: u16,
    #[serde(default = "default_hsm_threshold")]
    pub hsm_approvals: u8,
    #[serde(default)]
    pub allowed_key_types: HashSet<KeyType>,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            key_rotation_days: 90,
            hsm_approvals: 2,
            allowed_key_types: HashSet::from([KeyType::Secp256k1]),
        }
    }
} 