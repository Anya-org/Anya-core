// Web5 Protocols Module
// Provides protocol handling functionality for Web5

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::web5::{Web5Error, Web5Result};

/// Protocol Handler trait
/// 
/// Defines the interface for Web5 protocol handlers.
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
/// Defines a Web5 protocol.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolDefinition {
    /// Protocol ID
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
/// Defines a type in a Web5 protocol.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeDefinition {
    /// Type schema
    pub schema: String,
    /// Type description
    pub description: String,
}

/// Action Definition
/// 
/// Defines an action in a Web5 protocol.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
/// Manages Web5 protocols.
#[derive(Debug)]
pub struct ProtocolManager {
    /// Registered protocols
    protocols: HashMap<String, ProtocolDefinition>,
    /// Protocol handlers
    handlers: HashMap<String, Box<dyn ProtocolHandler>>,
}

impl ProtocolManager {
    /// Create a new Protocol Manager
    pub fn new() -> Self {
        Self {
            protocols: HashMap::new(),
            handlers: HashMap::new(),
        }
    }
    
    /// Register a protocol
    pub fn register_protocol(&mut self, handler: Box<dyn ProtocolHandler>) -> Web5Result<()> {
        let protocol_id = handler.protocol_id().to_string();
        let definition = handler.get_definition();
        
        self.protocols.insert(protocol_id.clone(), definition);
        self.handlers.insert(protocol_id, handler);
        
        Ok(())
    }
    
    /// Get a protocol definition
    pub fn get_protocol(&self, protocol_id: &str) -> Web5Result<&ProtocolDefinition> {
        self.protocols.get(protocol_id)
            .ok_or_else(|| Web5Error::ProtocolError(format!("Protocol not found: {}", protocol_id)))
    }
    
    /// Handle a protocol message
    pub fn handle_message(&self, protocol_id: &str, message: &[u8]) -> Web5Result<Vec<u8>> {
        let handler = self.handlers.get(protocol_id)
            .ok_or_else(|| Web5Error::ProtocolError(format!("Protocol handler not found: {}", protocol_id)))?;
        
        handler.handle_message(message)
    }
    
    /// Check if a protocol is registered
    pub fn has_protocol(&self, protocol_id: &str) -> bool {
        self.protocols.contains_key(protocol_id)
    }
    
    /// Get all registered protocols
    pub fn get_all_protocols(&self) -> Vec<&ProtocolDefinition> {
        self.protocols.values().collect()
    }
}

/// Profile Protocol Handler
/// 
/// Handles the profile protocol for Web5.
#[derive(Debug)]
pub struct ProfileProtocolHandler;

impl ProfileProtocolHandler {
    /// Create a new Profile Protocol Handler
    pub fn new() -> Self {
        Self
    }
}

impl ProtocolHandler for ProfileProtocolHandler {
    fn protocol_id(&self) -> &str {
        "https://example.com/protocols/profile"
    }
    
    fn handle_message(&self, message: &[u8]) -> Web5Result<Vec<u8>> {
        // In a real implementation, this would handle profile protocol messages
        // For this example, we're just returning the message
        Ok(message.to_vec())
    }
    
    fn get_definition(&self) -> ProtocolDefinition {
        let mut types = HashMap::new();
        types.insert(
            "profile".to_string(),
            TypeDefinition {
                schema: "https://schema.org/Person".to_string(),
                description: "A person's profile".to_string(),
            },
        );
        
        let actions = vec![
            ActionDefinition {
                name: "create".to_string(),
                description: "Create a profile".to_string(),
                input: Some("profile".to_string()),
                output: None,
            },
            ActionDefinition {
                name: "read".to_string(),
                description: "Read a profile".to_string(),
                input: None,
                output: Some("profile".to_string()),
            },
            ActionDefinition {
                name: "update".to_string(),
                description: "Update a profile".to_string(),
                input: Some("profile".to_string()),
                output: None,
            },
            ActionDefinition {
                name: "delete".to_string(),
                description: "Delete a profile".to_string(),
                input: None,
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

/// Messaging Protocol Handler
/// 
/// Handles the messaging protocol for Web5.
#[derive(Debug)]
pub struct MessagingProtocolHandler;

impl MessagingProtocolHandler {
    /// Create a new Messaging Protocol Handler
    pub fn new() -> Self {
        Self
    }
}

impl ProtocolHandler for MessagingProtocolHandler {
    fn protocol_id(&self) -> &str {
        "https://example.com/protocols/messaging"
    }
    
    fn handle_message(&self, message: &[u8]) -> Web5Result<Vec<u8>> {
        // In a real implementation, this would handle messaging protocol messages
        // For this example, we're just returning the message
        Ok(message.to_vec())
    }
    
    fn get_definition(&self) -> ProtocolDefinition {
        let mut types = HashMap::new();
        types.insert(
            "message".to_string(),
            TypeDefinition {
                schema: "https://schema.org/Message".to_string(),
                description: "A message".to_string(),
            },
        );
        
        let actions = vec![
            ActionDefinition {
                name: "send".to_string(),
                description: "Send a message".to_string(),
                input: Some("message".to_string()),
                output: None,
            },
            ActionDefinition {
                name: "receive".to_string(),
                description: "Receive a message".to_string(),
                input: None,
                output: Some("message".to_string()),
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

/// Credential Protocol Handler
/// 
/// Handles the credential protocol for Web5.
#[derive(Debug)]
pub struct CredentialProtocolHandler;

impl CredentialProtocolHandler {
    /// Create a new Credential Protocol Handler
    pub fn new() -> Self {
        Self
    }
}

impl ProtocolHandler for CredentialProtocolHandler {
    fn protocol_id(&self) -> &str {
        "https://example.com/protocols/credential"
    }
    
    fn handle_message(&self, message: &[u8]) -> Web5Result<Vec<u8>> {
        // In a real implementation, this would handle credential protocol messages
        // For this example, we're just returning the message
        Ok(message.to_vec())
    }
    
    fn get_definition(&self) -> ProtocolDefinition {
        let mut types = HashMap::new();
        types.insert(
            "credential".to_string(),
            TypeDefinition {
                schema: "https://www.w3.org/2018/credentials/v1".to_string(),
                description: "A verifiable credential".to_string(),
            },
        );
        
        let actions = vec![
            ActionDefinition {
                name: "issue".to_string(),
                description: "Issue a credential".to_string(),
                input: Some("credential".to_string()),
                output: None,
            },
            ActionDefinition {
                name: "verify".to_string(),
                description: "Verify a credential".to_string(),
                input: Some("credential".to_string()),
                output: Some("verification".to_string()),
            },
            ActionDefinition {
                name: "revoke".to_string(),
                description: "Revoke a credential".to_string(),
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
    
    #[test]
    fn test_protocol_manager() {
        let mut manager = ProtocolManager::new();
        
        // Register protocols
        manager.register_protocol(Box::new(ProfileProtocolHandler::new())).unwrap();
        manager.register_protocol(Box::new(MessagingProtocolHandler::new())).unwrap();
        
        // Check if protocols are registered
        assert!(manager.has_protocol("https://example.com/protocols/profile"));
        assert!(manager.has_protocol("https://example.com/protocols/messaging"));
        assert!(!manager.has_protocol("https://example.com/protocols/unknown"));
        
        // Get protocol definitions
        let profile_protocol = manager.get_protocol("https://example.com/protocols/profile").unwrap();
        assert_eq!(profile_protocol.protocol, "https://example.com/protocols/profile");
        assert_eq!(profile_protocol.version, "1.0");
        assert_eq!(profile_protocol.types.len(), 1);
        assert_eq!(profile_protocol.actions.len(), 4);
        
        let messaging_protocol = manager.get_protocol("https://example.com/protocols/messaging").unwrap();
        assert_eq!(messaging_protocol.protocol, "https://example.com/protocols/messaging");
        assert_eq!(messaging_protocol.version, "1.0");
        assert_eq!(messaging_protocol.types.len(), 1);
        assert_eq!(messaging_protocol.actions.len(), 2);
        
        // Handle messages
        let message = b"Hello, World!";
        let response = manager.handle_message("https://example.com/protocols/profile", message).unwrap();
        assert_eq!(response, message);
        
        let response = manager.handle_message("https://example.com/protocols/messaging", message).unwrap();
        assert_eq!(response, message);
        
        // Get all protocols
        let all_protocols = manager.get_all_protocols();
        assert_eq!(all_protocols.len(), 2);
    }
    
    #[test]
    fn test_profile_protocol_handler() {
        let handler = ProfileProtocolHandler::new();
        let definition = handler.get_definition();
        
        assert_eq!(definition.protocol, "https://example.com/protocols/profile");
        assert_eq!(definition.version, "1.0");
        assert_eq!(definition.types.len(), 1);
        assert_eq!(definition.actions.len(), 4);
        
        let message = b"Hello, World!";
        let response = handler.handle_message(message).unwrap();
        assert_eq!(response, message);
    }
    
    #[test]
    fn test_messaging_protocol_handler() {
        let handler = MessagingProtocolHandler::new();
        let definition = handler.get_definition();
        
        assert_eq!(definition.protocol, "https://example.com/protocols/messaging");
        assert_eq!(definition.version, "1.0");
        assert_eq!(definition.types.len(), 1);
        assert_eq!(definition.actions.len(), 2);
        
        let message = b"Hello, World!";
        let response = handler.handle_message(message).unwrap();
        assert_eq!(response, message);
    }
    
    #[test]
    fn test_credential_protocol_handler() {
        let handler = CredentialProtocolHandler::new();
        let definition = handler.get_definition();
        
        assert_eq!(definition.protocol, "https://example.com/protocols/credential");
        assert_eq!(definition.version, "1.0");
        assert_eq!(definition.types.len(), 1);
        assert_eq!(definition.actions.len(), 3);
        
        let message = b"Hello, World!";
        let response = handler.handle_message(message).unwrap();
        assert_eq!(response, message);
    }
} 