// use std::error::Error; // Commented out as it's not being used
// Web5 Protocols Implementation
// Provides protocol handlers for Web5 interactions
// [AIR-012] Operational Reliability and [AIP-002] Modular Architecture

use crate::web5::identity::{Web5Error, Web5Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Protocol Handler trait
///
/// Defines the interface for protocol handlers in the Web5 system,
/// following the Hexagonal Architecture principles.
pub trait ProtocolHandler: Send + Sync {
    /// Get the protocol ID
    fn protocol_id(&self) -> &str;

    /// Handle a protocol message
    fn handle_message(&self, message: &[u8]) -> Web5Result<Vec<u8>>;

    /// Get protocol definition
    fn get_definition(&self) -> ProtocolDefinition;
}

/// Protocol Definition
///
/// Describes a protocol's capabilities and structure.
#[derive(Clone, Serialize, Deserialize)]
pub struct ProtocolDefinition {
    /// Protocol ID (URI)
    pub protocol: String,
    /// Protocol version
    pub version: String,
    /// Protocol types
    pub types: HashMap<String, TypeDefinition>,
    /// Protocol actions
    pub actions: Vec<ActionDefinition>,
}

/// Type Definition
///
/// Describes a data type within a protocol.
#[derive(Clone, Serialize, Deserialize)]
pub struct TypeDefinition {
    /// Type schema
    pub schema: String,
    /// Type description
    pub description: String,
}

/// Action Definition
///
/// Describes an action within a protocol.
#[derive(Clone, Serialize, Deserialize)]
pub struct ActionDefinition {
    /// Action name
    pub name: String,
    /// Action description
    pub description: String,
    /// Action input type
    pub input: Option<String>,
    /// Action output type
    pub output: Option<String>,
}

/// Protocol Manager
///
/// Manages protocol handlers and facilitates protocol-based interactions.
pub struct ProtocolManager {
    /// Registered protocols
    protocols: HashMap<String, ProtocolDefinition>,
    /// Protocol handlers
    handlers: HashMap<String, Box<dyn ProtocolHandler>>,
}

impl Default for ProtocolManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ProtocolManager {
    /// Create a new protocol manager
    pub fn new() -> Self {
        Self {
            protocols: HashMap::new(),
            handlers: HashMap::new(),
        }
    }

    /// Register a protocol handler
    pub fn register_protocol(&mut self, handler: Box<dyn ProtocolHandler>) -> Web5Result<()> {
        let protocol_id = handler.protocol_id().to_string();
        let definition = handler.get_definition();

        self.protocols.insert(protocol_id.clone(), definition);
        self.handlers.insert(protocol_id, handler);

        Ok(())
    }

    /// Get a protocol definition by ID
    pub fn get_protocol(&self, protocol_id: &str) -> Web5Result<&ProtocolDefinition> {
        self.protocols
            .get(protocol_id)
            .ok_or_else(|| Web5Error::Protocol(format!("Protocol not found: {protocol_id}")))
    }

    /// Handle a message for a specific protocol
    pub fn handle_message(&self, protocol_id: &str, message: &[u8]) -> Web5Result<Vec<u8>> {
        let handler = self.handlers.get(protocol_id).ok_or_else(|| {
            Web5Error::Protocol(format!("No handler found for protocol: {protocol_id}"))
        })?;

        handler.handle_message(message)
    }

    /// Check if a protocol is registered
    pub fn has_protocol(&self, protocol_id: &str) -> bool {
        self.protocols.contains_key(protocol_id)
    }

    /// Get all registered protocol definitions
    pub fn get_all_protocols(&self) -> Vec<&ProtocolDefinition> {
        self.protocols.values().collect()
    }
}

/// Profile Protocol Handler
///
/// Handles the standard profile protocol for Web5.
pub struct ProfileProtocolHandler;

impl Default for ProfileProtocolHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl ProfileProtocolHandler {
    /// Create a new profile protocol handler
    pub fn new() -> Self {
        Self {}
    }
}

impl ProtocolHandler for ProfileProtocolHandler {
    fn protocol_id(&self) -> &str {
        "https://identity.foundation/schemas/profile"
    }

    fn handle_message(&self, message: &[u8]) -> Web5Result<Vec<u8>> {
        // Simple echo implementation for demonstration
        Ok(message.to_vec())
    }

    fn get_definition(&self) -> ProtocolDefinition {
        let mut types = HashMap::new();
        types.insert(
            "profile".to_string(),
            TypeDefinition {
                schema: r#"{
                    "type": "object",
                    "properties": {
                        "name": { "type": "string" },
                        "image": { "type": "string", "format": "uri" },
                        "description": { "type": "string" }
                    }
                }"#
                .to_string(),
                description: "A user profile".to_string(),
            },
        );

        let actions = vec![
            ActionDefinition {
                name: "get".to_string(),
                description: "Get a profile".to_string(),
                input: None,
                output: Some("profile".to_string()),
            },
            ActionDefinition {
                name: "update".to_string(),
                description: "Update a profile".to_string(),
                input: Some("profile".to_string()),
                output: Some("profile".to_string()),
            },
        ];

        ProtocolDefinition {
            protocol: self.protocol_id().to_string(),
            version: "1.0".to_string(),
            types,
            actions,
        }
    }
}

/// Credentials Protocol Handler
///
/// Handles the standard credentials protocol for Web5.
pub struct CredentialProtocolHandler;

impl Default for CredentialProtocolHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl CredentialProtocolHandler {
    /// Create a new credentials protocol handler
    pub fn new() -> Self {
        Self {}
    }
}

impl ProtocolHandler for CredentialProtocolHandler {
    fn protocol_id(&self) -> &str {
        "https://identity.foundation/schemas/credentials"
    }

    fn handle_message(&self, message: &[u8]) -> Web5Result<Vec<u8>> {
        // Simple echo implementation for demonstration
        Ok(message.to_vec())
    }

    fn get_definition(&self) -> ProtocolDefinition {
        let mut types = HashMap::new();
        types.insert(
            "credential".to_string(),
            TypeDefinition {
                schema: r#"{
                    "type": "object",
                    "properties": {
                        "id": { "type": "string" },
                        "type": { "type": "array", "items": { "type": "string" } },
                        "issuer": { "type": "string" },
                        "issuanceDate": { "type": "string", "format": "date-time" },
                        "credentialSubject": { "type": "object" }
                    }
                }"#
                .to_string(),
                description: "A verifiable credential".to_string(),
            },
        );

        let actions = vec![
            ActionDefinition {
                name: "issue".to_string(),
                description: "Issue a credential".to_string(),
                input: Some("credential".to_string()),
                output: Some("credential".to_string()),
            },
            ActionDefinition {
                name: "verify".to_string(),
                description: "Verify a credential".to_string(),
                input: Some("credential".to_string()),
                output: None,
            },
        ];

        ProtocolDefinition {
            protocol: self.protocol_id().to_string(),
            version: "1.0".to_string(),
            types,
            actions,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[tokio::test]
    async fn test_protocol_manager() -> Result<(), Box<dyn Error>> {
        let mut manager = ProtocolManager::new();

        // Test registering a protocol
        let profile_handler = Box::new(ProfileProtocolHandler::new());
        manager.register_protocol(profile_handler)?;

        // Test protocol lookup
        assert!(manager.has_protocol("https://identity.foundation/schemas/profile"));

        // Test getting all protocols
        let protocols = manager.get_all_protocols();
        assert_eq!(protocols.len(), 1);

        // Test getting protocol definition
        let _def = manager.get_protocol("https://identity.foundation/schemas/profile")?;

        Ok(())
    }

    #[tokio::test]
    async fn test_profile_protocol_handler() -> Result<(), Box<dyn Error>> {
        let handler = ProfileProtocolHandler::new();

        // Test protocol ID
        assert_eq!(
            handler.protocol_id(),
            "https://identity.foundation/schemas/profile"
        );

        // Test message handling
        let response = handler.handle_message(b"test")?;
        assert_eq!(response, b"test");

        Ok(())
    }

    #[tokio::test]
    async fn test_credential_protocol_handler() -> Result<(), Box<dyn Error>> {
        let handler = CredentialProtocolHandler::new();

        // Test protocol ID
        assert_eq!(
            handler.protocol_id(),
            "https://identity.foundation/schemas/credentials"
        );

        // Test message handling
        let response = handler.handle_message(b"test")?;
        assert_eq!(response, b"test");

        Ok(())
    }
}
