// Mobile module for anya-core
// Contains FFI bindings and SDK implementations for mobile platforms

pub mod sdk;
#[cfg(feature = "ffi")]
pub mod ffi;

pub use self::sdk::MobileSDK;
