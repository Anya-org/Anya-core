use std::path::PathBuf;
use anya_core::install::{AnyaInstaller, InstallationSource, BitcoinConfig, protocol};

#[tokio::test]
async fn test_full_installation() {
    let source = InstallationSource::PreBuiltBinary("test.pkg".to_string());

    let bitcoin_config = BitcoinConfig {
        network: "Testnet".to_string(),
        data_dir: PathBuf::from("/tmp/bitcoin_data_full_install_test"),
    };

    let installer = AnyaInstaller::new(source, bitcoin_config).unwrap();

    let result = installer.install(PathBuf::from("/tmp/full_install_target")).await;
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
    let bitcoin_config = BitcoinConfig {
        network: "Mainnet".to_string(),
        data_dir: PathBuf::from("/tmp/bitcoin_data_mainnet_taproot_test"),
    };

    let source = InstallationSource::PreBuiltBinary("taproot.pkg".to_string());

    let installer = AnyaInstaller::new(source, bitcoin_config).unwrap();

    let result = installer.install(PathBuf::from("/tmp/mainnet_taproot_target")).await;
    assert!(result.is_ok(), "Taproot installation failed");
}

/*
// This test is commented out as NetworkValidationConfig and its functionality
// (like required_ports) seem to have been removed from AnyaInstaller.
// The use of BitcoinConfig::default() was also an error, and the struct has changed.
// The test needs to be re-evaluated based on current installer capabilities.
#[tokio::test]
async fn test_network_compliance_failure() {
    let config = NetworkValidationConfig {
        required_ports: vec![8333],
        ..Default::default()
    };

    let installer = AnyaInstaller::new(
        config,
        InstallationSource::Local {
            path: PathBuf::from(\"test.pkg\"),
            checksum: \"test\".into(),
        },
        BitcoinConfig::default(),
    )
    .unwrap();

    let result = installer.install(PathBuf::from(\"/tmp\")).await;
    assert!(result.is_err(), \"Should fail on port validation\");
}
*/
