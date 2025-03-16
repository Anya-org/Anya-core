#[cfg(test)]
pub mod mock {
    use bitcoin::Transaction;
    use bitcoin::util::psbt::PartiallySignedTransaction;
    
    /// Mock Bitcoin transaction verification that doesn't require a local node
    pub fn verify_transaction(tx_hex: &str) -> bool {
        // Parse transaction from hex
        match hex::decode(tx_hex) {
            Ok(bytes) => {
                match bitcoin::consensus::deserialize::<Transaction>(&bytes) {
                    Ok(_) => true,
                    Err(_) => false
                }
            },
            Err(_) => false
        }
    }
    
    /// Mock PSBT verification that doesn't require a local node
    pub fn verify_psbt(psbt_base64: &str) -> bool {
        // Parse PSBT from base64
        match base64::decode(psbt_base64) {
            Ok(bytes) => {
                match bitcoin::consensus::deserialize::<PartiallySignedTransaction>(&bytes) {
                    Ok(_) => true,
                    Err(_) => false
                }
            },
            Err(_) => false
        }
    }
    
    /// Test data provider for Bitcoin tests
    pub fn get_test_transactions() -> Vec<(&'static str, bool)> {
        vec![
            // Valid Taproot transaction (BIP-341 compliant)
            ("0200000000010140d43a99926d43eb0e619bf0b3d83b4a31f60c176beecfb9d35bf45e54d0f7420100000000ffffffff0200e1f505000000002251209a9ea267884f5549c206b2aec2bd56d98730f90532ea7f7154d4d4f923b7e3bb0c4c1e0a0000000016001438df8b9f4eea7c7c5a92fb8bd097d0984a269abb02473044022077efae8a3985f89044e13e01f26f2e6542f48db42e0af26d1f7ade945108b10f022028d260a8e115224d6aa6f0d4832ab17b0e70abf46fc83fcc0f25da6602a9f49801210279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f8179800000000", true),
            // Invalid transaction (wrong format)
            ("not_a_transaction", false)
        ]
    }
} 