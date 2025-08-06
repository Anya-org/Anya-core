// Mobile module for anya-core
// Contains FFI bindings and SDK implementations for mobile platforms

#[cfg(feature = "ffi")]
pub mod ffi;
pub mod sdk;

pub use self::sdk::MobileSDK;
