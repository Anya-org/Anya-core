// Add PSBT version validation
pub fn validate_version(required: u8) -> bool {
    let current = bitcoin::psbt::PSBT_VERSION;
    if current < required {
        error!(
            "PSBT version mismatch: Required {} > Actual {}",
            required, current
        );
        false
    } else {
        true
    }
} 