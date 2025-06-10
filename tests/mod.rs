//! Main test module for Anya Core
//! 
//! This module organizes all test categories and provides utility functions
//! for testing the entire Anya Core system.

// Common test utilities - centralized to eliminate duplicates
pub mod common;

// Test modules
pub mod bitcoin;
pub mod core;
pub mod dao;
pub mod hardware;
pub mod integration;
pub mod layer2;
pub mod protocols;
pub mod security;

/// Test utilities (legacy compatibility)
pub mod test_utils;

/// Re-export important test functionality
pub use test_utils::*;
pub use common::*;
