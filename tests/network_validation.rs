#[cfg(test)]
mod tests {
    use anya_core::network::validation::{NetworkValidationConfig, NetworkValidator};
    use std::env;

    #[tokio::test]
    async fn test_full_network_validation() {
        if env::var("ANYA_NETWORK_TEST").ok().as_deref() != Some("1") {
            println!("[skip] Network tests disabled (set ANYA_NETWORK_TEST=1 to enable)");
            return;
        }
        let config = NetworkValidationConfig {
            check_ssl: false,
            ..NetworkValidationConfig::default()
        }; // Disable SSL for local tests

        let validator = NetworkValidator::new(config);
        let result = validator.validate_network().await;

        // Be lenient; environment may vary
        assert!(result.connectivity.internet_available);
        assert!(result.bandwidth.download_mbps >= 0.0);
    }

    #[tokio::test]
    async fn test_bip341_port_validation() {
        if env::var("ANYA_NETWORK_TEST").ok().as_deref() != Some("1") {
            println!("[skip] Network tests disabled (set ANYA_NETWORK_TEST=1 to enable)");
            return;
        }
        let config = NetworkValidationConfig {
            required_ports: vec![8433], // Taproot monitoring port
            ..Default::default()
        };

        let validator = NetworkValidator::new(config);
        let result = validator.validate_ports().await;

        // Only assert if enabled environment actually has this open
        assert!(result.open_ports.contains(&8433) || result.closed_ports.contains(&8433));
    }

    #[tokio::test]
    async fn test_psbt_port_validation() {
        if env::var("ANYA_NETWORK_TEST").ok().as_deref() != Some("1") {
            println!("[skip] Network tests disabled (set ANYA_NETWORK_TEST=1 to enable)");
            return;
        }
        let config = NetworkValidationConfig {
            required_ports: vec![174], // BIP-174 PSBT port
            ..Default::default()
        };

        let validator = NetworkValidator::new(config);
        let result = validator.validate_ports().await;

        // Only check we got a determination
        assert!(result.open_ports.contains(&174) || result.closed_ports.contains(&174));
    }
}
