#![feature(edition2021)]
#[cfg(test)]
mod integration_tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_version_comparison() {
        assert_eq!(version_compare("v0.10.0", "v0.9.5"), Ordering::Greater);
        assert_eq!(version_compare("2.5.1", "2.5.0"), Ordering::Greater);
        assert_eq!(version_compare("v3.0-rc1", "v3.0"), Ordering::Less);
    }

    #[tokio::test]
    async fn test_taproot_commitment_verification() {
        let result = AnyaInstaller::verify_taproot_commitment()
            .expect("Taproot verification failed");
        assert!(result, "SILENT_LEAF commitment verification failed");
    }

    #[test]
    fn test_memory_isolation_check() {
        #[cfg(target_os = "linux")]
        {
            let result = check_memory_isolation()
                .expect("Memory isolation check failed");
            assert!(result, "Memory isolation not properly configured");
        }
    }

    #[test]
    fn test_full_installation_flow() {
        let temp_dir = TempDir::new().unwrap();
        let installer = AnyaInstaller::new(temp_dir.path().to_str().unwrap())
            .expect("Installer creation failed");
        
        let result = installer.install_with_cleanup();
        assert!(result.is_ok(), "Installation failed: {:?}", result);
        
        let audit = installer.generate_audit_log();
        assert!(audit.is_ok(), "Audit generation failed");
    }
} 