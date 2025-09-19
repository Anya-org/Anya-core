// [AIR-3][AIS-3][BPC-3][RES-3] HSM provider module declarations
// This follows official Bitcoin Improvement Proposals (BIPs) standards for HSM providers
// Each provider implements the HsmProvider trait with specific security features

pub mod bitcoin;
pub mod hardware;
pub mod ledger;
pub mod pkcs11;
// Simulator is for development only; gate it behind the `dev-sim` feature
#[cfg(feature = "dev-sim")]
pub mod simulator;
pub mod software;
pub mod tpm;

// Re-export provider structs for use by other modules
pub use self::bitcoin::BitcoinHsmProvider;
pub use self::hardware::HardwareHsmProvider;
#[cfg(feature = "dev-sim")]
pub use self::simulator::SimulatorHsmProvider;
pub use self::software::SoftwareHsmProvider;
