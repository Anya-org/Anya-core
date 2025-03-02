pub fn enterprise_features_enabled() -> bool {
    true
}

pub struct ComplianceConfig {
    pub enabled: bool,
}

impl Default for ComplianceConfig {
    fn default() -> Self {
        Self { enabled: true }
    }
} 