#[tokio::test]
async fn test_full_installation() {
    let config = NetworkValidationConfig {
        check_ssl: false,
        ..Default::default()
    };

    let source = InstallationSource::Local {
        path: PathBuf::from("test.pkg"),
        checksum: "aabbcc".into(),
    };

    let bitcoin_config = BitcoinConfig {
        network: BitcoinNetwork::Testnet,
        required_bips: vec![174, 341],
        taproot_enabled: true,
    };

    let installer = AnyaInstaller::new(config, source, bitcoin_config).unwrap();

    let result = installer.install(PathBuf::from("/tmp")).await;
    assert!(result.is_ok(), "Installation failed: {:?}", result);
}

#[test]
fn test_bip_verification() {
    let required = vec![174, 341, 370];
    let installed = vec![174, 341, 370, 371];
    assert!(protocol::verify_bip_support(&required, &installed).is_ok());

    let installed_missing = vec![174, 341];
    assert!(protocol::verify_bip_support(&required, &installed_missing).is_err());
}

#[tokio::test]
async fn test_mainnet_taproot_installation() {
    let config = NetworkValidationConfig {
        endpoints: vec!["https://taproot.node".into()],
        ..Default::default()
    };

    let bitcoin_config = BitcoinConfig {
        network: BitcoinNetwork::Mainnet,
        required_bips: vec![341],
        taproot_enabled: true,
    };

    let installer = AnyaInstaller::new(
        config,
        InstallationSource::Local {
            path: PathBuf::from("taproot.pkg"),
            checksum: "taproot123".into(),
        },
        bitcoin_config,
    )
    .unwrap();

    let result = installer.install(PathBuf::from("/tmp")).await;
    assert!(result.is_ok(), "Taproot installation failed");
}

#[tokio::test]
async fn test_network_compliance_failure() {
    let config = NetworkValidationConfig {
        required_ports: vec![8333],
        ..Default::default()
    };

    let installer = AnyaInstaller::new(
        config,
        InstallationSource::Local {
            path: PathBuf::from("test.pkg"),
            checksum: "test".into(),
        },
        BitcoinConfig::default(),
    )
    .unwrap();

    let result = installer.install(PathBuf::from("/tmp")).await;
    assert!(result.is_err(), "Should fail on port validation");
}
