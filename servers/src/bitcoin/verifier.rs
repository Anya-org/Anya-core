fn verify_taproot(&self, tx: &Transaction) -> Result<(), TaprootError> {
    // Missing BIP-341 §4.3 silent leaf validation
    verify_schnorr_signature(tx)?; // BIP-340
    // ... existing code ...
} 