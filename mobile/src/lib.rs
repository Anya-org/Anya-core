#[cfg(feature = "mobile")]
pub fn mobile_interface_version() -> &'static str {
    "0.1.0"
}

#[cfg(feature = "mobile")]
pub struct MobileConfig {
    pub offline_mode: bool,
}

#[cfg(feature = "mobile")]
impl Default for MobileConfig {
    fn default() -> Self {
        Self {
            offline_mode: false,
        }
    }
} 