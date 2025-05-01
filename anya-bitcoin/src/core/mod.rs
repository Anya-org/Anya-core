//! Core Bitcoin implementation
//!
//! This module contains the core functionality of the Bitcoin implementation,
//! following the hexagonal architecture principle of keeping the domain
//! logic separate from external concerns.

pub mod consensus;
pub mod mempool;
pub mod network;
pub mod script;
pub mod error;
pub mod wallet;
