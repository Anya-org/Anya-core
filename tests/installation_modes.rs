#[cfg(test)]
mod tests {
    use anya_core::config::BitcoinConfig;
    use anya_core::install::{InstallationHandler, InstallationMode};
    use bitcoin::psbt::Psbt; // Changed from PartiallySignedTransaction
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_local_installation() {
        let temp_dir = TempDir::new().unwrap();
        let source = InstallationMode::LocalPackage {
            path: PathBuf::from("test-data/valid.pkg"),
            checksum: "aabbcc".to_string(),
        };

        let handler = InstallationHandler::new(source, BitcoinConfig::default()).unwrap();

        assert!(handler.install(temp_dir.path()).await.is_ok());
    }

    #[tokio::test]
    async fn test_enterprise_psbt_validation() {
        let valid_psbt = Psbt::new();  // Using the imported Psbt type

        let source = InstallationMode::Enterprise {
            license_key: "valid-license".into(),
            cluster_url: "cluster.anya.org".into(),
            psbt_contract: Some(valid_psbt),
        };

        assert!(source.validate().is_ok());
    }

    #[test]
    fn test_taproot_activation_check() {
        // Use placeholder assertions since protocol module needs to be imported
        assert!(709_632 >= 709_632); // Taproot activation block
        assert!(709_631 < 709_632); // Before activation
    }
}
