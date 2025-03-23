#[cfg(test)]
mod hil_tests {
    use super::*;
    use anya_hw::HardwareInterface;
    use crate::hardware::MockHardwareInterface;

    #[test]
    fn test_hsm_integration() {
        let hsm = HardwareInterface::connect("hsm://prod-cluster");
        let sig = hsm.sign_message(b"test", "secp256k1")
            .expect("HSM signing failed");
        
        let valid = verify_signature(&sig, b"test")
            .expect("Verification failed");
        assert!(valid, "HSM signature validation failed");
    }

    #[test]
    fn test_sgx_enclave_validation() {
        let enclave = HardwareInterface::sgx_enclave("v2.5-secure");
        let result = enclave.execute_secure(
            "verify_taproot_commitment", 
            &hex::decode(BIP341_SILENT_LEAF).unwrap()
        );
        
        assert!(result.unwrap(), "SGX enclave validation failed");
    }

    #[test]
    fn test_fpga_acceleration() {
        let hasher = HardwareInterface::fpga("sha256-accelerator");
        let hash = hasher.digest(b"test")
            .expect("FPGA acceleration failed");
        
        let expected = bitcoin::hashes::sha256::Hash::hash(b"test");
        assert_eq!(hash, expected.as_byte_array(), "FPGA hash mismatch");
    }

    #[test]
    fn test_hsm_failover() {
        let mut hsm = MockHardwareInterface::new();
        hsm.simulate_failure();
        
        let backup_hsm = HardwareInterface::connect("hsm://backup-cluster");
        let sig = backup_hsm.sign_message(b"test", "secp256k1")
            .expect("Backup HSM failed");
        
        assert!(verify_signature(&sig, b"test").unwrap());
    }

    #[test]
    fn test_sgx_attestation() {
        let enclave = HardwareInterface::sgx_enclave("v2.5-secure");
        let attestation = enclave.get_attestation()
            .expect("SGX attestation failed");
        
        assert!(validate_attestation(&attestation, "BTC_MAINNET"));
    }
} 