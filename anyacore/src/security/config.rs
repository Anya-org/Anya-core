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