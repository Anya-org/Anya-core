//! Hardware optimization and testing module [AIR-3][AIS-3][BPC-3][PFM-3]
//! 
//! This module contains integration tests for hardware optimization components,
//! with specific focus on validating the i3-7020U (Kaby Lake) minimum hardware
//! specification support and ensuring Bitcoin consensus compliance.
//!
//! The test suite validates full alignment with all four Bitcoin Core principles:
//! - Decentralization: Supporting widespread hardware incl. i3-7020U baseline
//! - Security: Ensuring hardware optimizations maintain consensus rules
//! - Immutability: Guaranteeing identical verification results across hardware
//! - Privacy: Optimizing batch operations for privacy-enhancing technologies

mod hardware_optimization_tests;
mod profile_tests;
mod bitcoin_principles_tests;

// Re-export hardware tests for main test runner
pub use hardware_optimization_tests::*;
pub use profile_tests::*;
pub use bitcoin_principles_tests::*;
