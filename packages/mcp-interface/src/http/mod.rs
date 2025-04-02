// MCP HTTP Interface Module [AIR-3][BPC-3][AIS-3]
//
// This module provides HTTP transport for the MCP interfaces
// It implements both the client and server components
// Compliant with the Bitcoin Development Framework v2.5

mod error;
mod transport;
mod handlers;

// Re-export main components
pub use error::TransportError;
pub use transport::HttpTransport;
pub use transport::start_server;

// Export internal state for testing
#[cfg(test)]
pub use transport::HttpServerState;
