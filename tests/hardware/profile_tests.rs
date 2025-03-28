#![feature(edition2021)]
#[cfg(test)]
mod hardware_tests {
    use super::*;
    use sysinfo::System;

    #[test]
    fn test_minimal_hardware_profile() {
        let hw = HardwareProfile {
            cpu_cores: 2,
            memory_gb: 4,
            disk_space_gb: 50,
            network_mbps: 50.0,
        };

        let config = BitcoinConfig::from_hardware_profile(&hw);
        
        assert!(!config.taproot_enabled, "Taproot should be disabled on minimal");
        assert_eq!(config.psbt_version, 1, "PSBT v1 expected for minimal");
        assert_eq!(config.rpc_threads, 2, "2 RPC threads for 2 cores");
    }

    #[test]
    fn test_enterprise_hardware_profile() {
        let hw = HardwareProfile {
            cpu_cores: 16,
            memory_gb: 128,
            disk_space_gb: 2000,
            network_mbps: 1000.0,
        };

        let config = BitcoinConfig::from_hardware_profile(&hw);
        
        assert!(config.taproot_enabled, "Taproot should be enabled");
        assert_eq!(config.psbt_version, 2, "PSBT v2 required");
        assert!(config.dlc_support, "DLC support required");
        assert_eq!(config.rpc_threads, 8, "50% core utilization");
    }

    #[test]
    fn test_edge_case_profiles() {
        let hw = HardwareProfile {
            cpu_cores: 3, // Odd core count
            memory_gb: 7, // Between tiers
            disk_space_gb: 99, // Just below threshold
            network_mbps: 99.9,
        };

        let config = BitcoinConfig::from_hardware_profile(&hw);
        
        assert!(!config.rgb_support, "RGB should be disabled under 100GB");
        assert_eq!(config.psbt_version, 1, "PSBT v1 for <8GB RAM");
    }
} 