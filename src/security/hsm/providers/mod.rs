// [AIR-3][AIS-3][BPC-3][RES-3] HSM provider module declarations
// This follows the Bitcoin Development Framework v2.5 standards for HSM providers
// Each provider implements the HsmProvider trait with specific security features

pub mod bitcoin;
pub mod hardware;
pub mod ledger;
pub mod pkcs11;
pub mod simulator;
pub mod software;
pub mod tpm;

// Re-export only the BitcoinHsmProvider to avoid duplicate exports
// This is the primary provider used in the application
pub use self::bitcoin::BitcoinHsmProvider; // [AIR-3][AIS-3][BPC-3][RES-3] AI labeling for compliance
