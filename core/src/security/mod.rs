// Add hardware capability validation
pub fn validate_hardware_capabilities() -> anyhow::Result<()> {
    #[cfg(target_arch = "x86_64")]
    {
        if !is_x86_feature_detected!("sha") && !is_x86_feature_detected!("aes") {
            anyhow::bail!("Missing required CPU features for HSM operations");
        }
    }

    // Validate TPM presence
    if !validate_tpm_2_0() {
        anyhow::bail!("TPM 2.0 required for secure operations");
    }

    Ok(())
}

fn validate_tpm_2_0() -> bool {
    #[cfg(target_os = "linux")]
    {
        Path::new("/dev/tpm0").exists() || Path::new("/dev/tpmrm0").exists()
    }
    #[cfg(not(target_os = "linux"))]
    {
        // Windows/macOS TPM validation
        false // Placeholder for platform-specific implementation
    }
}
