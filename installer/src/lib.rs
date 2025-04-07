pub mod hardware;
pub mod network;
pub mod adapters;
pub mod ports;

// Re-export platform module from core crate
pub use core::platform;

// ... existing code ... 