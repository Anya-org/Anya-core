#[cfg(test)]
mod tests {
    use anya_core::network::validation::{NetworkValidator, NetworkValidationConfig};
    use std::net::SocketAddr;
    use tokio::net::TcpListener;

    #[tokio::test]
    async fn test_full_network_validation() {
        let mut config = NetworkValidationConfig::default();
        config.check_ssl = false; // Disable SSL for local tests

        let validator = NetworkValidator::new(config);
        let result = validator.validate_network().await;

        assert!(result.connectivity.internet_available);
        assert_eq!(result.ports.open_ports, vec![80, 443]);
        assert!(result.bandwidth.download_mbps > 0.0);
    }

    #[tokio::test]
    async fn test_bip341_port_validation() {
        let config = NetworkValidationConfig {
            required_ports: vec![8433], // Taproot monitoring port
            ..Default::default()
        };

        let validator = NetworkValidator::new(config);
        let result = validator.validate_ports().await;

        assert!(
            result.open_ports.contains(&8433),
            "BIP-341 Taproot port must be open"
        );
    }

    #[tokio::test]
    async fn test_psbt_port_validation() {
        let config = NetworkValidationConfig {
            required_ports: vec![174], // BIP-174 PSBT port
            ..Default::default()
        };

        let validator = NetworkValidator::new(config);
        let result = validator.validate_ports().await;

        assert!(
            !result.closed_ports.contains(&174),
            "BIP-174 PSBT port must be accessible"
        );
    }
}
