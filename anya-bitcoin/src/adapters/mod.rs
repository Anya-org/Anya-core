//! External adapters for Bitcoin implementation
//!
//! Following the Hexagonal Architecture pattern, this module contains
//! adapters that connect the core domain to external systems.

pub mod rpc;
pub mod storage;
pub mod protocols;

