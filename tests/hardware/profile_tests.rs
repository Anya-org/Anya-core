#[cfg(test)]
mod hardware_tests {

    // Mock hardware profile for testing
    struct MockHardwareProfile {
        cpu_cores: u32,
        memory_gb: u32,
        disk_space_gb: u32,
        network_mbps: f64,
    }

    // Mock bitcoin config for testing
    struct MockBitcoinConfig {
        taproot_enabled: bool,
        psbt_version: u8,
        rpc_threads: u32,
    }

    impl MockBitcoinConfig {
        fn from_hardware_profile(hw: &MockHardwareProfile) -> Self {
            Self {
                taproot_enabled: hw.cpu_cores >= 8,
                psbt_version: if hw.memory_gb >= 16 { 2 } else { 1 },
                rpc_threads: hw.cpu_cores.min(16),
            }
        }
    }

    #[test]
    fn test_minimal_hardware_profile() {
        let hw = MockHardwareProfile {
            cpu_cores: 2,
            memory_gb: 4,
            disk_space_gb: 50,
            network_mbps: 50.0,
        };

        let config = MockBitcoinConfig::from_hardware_profile(&hw);

        assert!(
            !config.taproot_enabled,
            "Taproot should be disabled on minimal"
        );
        assert_eq!(config.psbt_version, 1, "PSBT v1 expected for minimal");
        assert_eq!(config.rpc_threads, 2, "2 RPC threads for 2 cores");
    }

    #[test]
    fn test_enterprise_hardware_profile() {
        let hw = MockHardwareProfile {
            cpu_cores: 16,
            memory_gb: 128,
            disk_space_gb: 2000,
            network_mbps: 1000.0,
        };

        let config = MockBitcoinConfig::from_hardware_profile(&hw);

        assert!(config.taproot_enabled, "Taproot should be enabled");
        assert_eq!(config.psbt_version, 2, "PSBT v2 expected for enterprise");
        assert_eq!(config.rpc_threads, 16, "16 RPC threads for 16 cores");
    }

    #[test]
    fn test_standard_hardware_profile() {
        let hw = MockHardwareProfile {
            cpu_cores: 8,
            memory_gb: 32,
            disk_space_gb: 500,
            network_mbps: 200.0,
        };

        let config = MockBitcoinConfig::from_hardware_profile(&hw);

        assert!(
            config.taproot_enabled,
            "Taproot should be enabled on standard"
        );
        assert_eq!(config.psbt_version, 2, "PSBT v2 expected for standard");
        assert_eq!(config.rpc_threads, 8, "8 RPC threads for 8 cores");
    }

    #[test]
    fn test_edge_case_profiles() {
        let hw = MockHardwareProfile {
            cpu_cores: 3,      // Odd core count
            memory_gb: 7,      // Between tiers
            disk_space_gb: 99, // Just below threshold
            network_mbps: 99.9,
        };

        let config = MockBitcoinConfig::from_hardware_profile(&hw);

        // Basic validations for edge cases
        assert_eq!(config.psbt_version, 1, "PSBT v1 for <8GB RAM");
        assert_eq!(config.rpc_threads, 3, "3 RPC threads for 3 cores");
    }
}
