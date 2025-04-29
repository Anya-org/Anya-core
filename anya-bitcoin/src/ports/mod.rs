//! Port interfaces for Bitcoin implementation
//!
//! Following the Hexagonal Architecture pattern, this module contains
//! the port interfaces that define the boundaries of the core domain.

pub mod blockchain_port;
pub mod transaction_port;
pub mod layer2_port;


