// Unified FFI for React Native
#[react-native::bridge]
mod ffi {
    extern "Rust" {
        fn create_wallet(mnemonic: String) -> Result<()>;
        fn send_transaction(recipient: String, amount: u64) -> Result<String>;
        fn create_lightning_invoice(amount: u64) -> Result<String>;
        fn init_hsm(config: HsmConfig) -> Result<()>;
    }
} 