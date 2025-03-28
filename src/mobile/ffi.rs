#![feature(edition2021)]
// Unified mobile FFI interface
#[react-native::bridge]
mod ffi {
    extern "Rust" {
        // Core wallet operations
        fn create_taproot_wallet(mnemonic: String) -> Result<String>;
        fn sign_psbt(psbt: String) -> Result<String>;
        fn verify_payment(proof: String) -> Result<bool>;
        
        // HSM operations
        fn init_hsm(config: String) -> Result<()>;
        fn hsm_sign(message: String) -> Result<String>;
        
        // Compliance features
        fn generate_compliance_badge() -> Result<String>;
    }
} 