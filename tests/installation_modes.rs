#![feature(edition2021)]
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use bitcoin::psbt::PartiallySignedTransaction;

    #[tokio::test]
    async fn test_local_installation() {
        let temp_dir = TempDir::new().unwrap();
        let source = InstallationSource::Local {
            path: PathBuf::from("test-data/valid.pkg"),
            checksum: "aabbcc".to_string()
        };
        
        let handler = InstallationHandler::new(
            source,
            BitcoinConfig::default()
        ).unwrap();
        
        assert!(handler.install(temp_dir.path()).await.is_ok());
    }

    #[tokio::test]
    async fn test_enterprise_psbt_validation() {
        let valid_psbt = PartiallySignedTransaction {
            // ... valid PSBT data ...
        };
        
        let source = InstallationSource::EnterpriseCluster {
            license_key: "valid-license".into(),
            cluster_url: "cluster.anya.org".into(),
            psbt_contract: Some(valid_psbt),
        };
        
        assert!(source.validate().is_ok());
    }

    #[test]
    fn test_taproot_activation_check() {
        assert!(protocol::check_taproot_activation(709_632).is_ok());
        assert!(protocol::check_taproot_activation(709_631).is_err());
    }
} 