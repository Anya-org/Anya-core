// Layer 2 Integrations
// Last Updated: 2025-03-06

//! # Layer 2 Integrations
//!
//! This module provides integration with various Bitcoin Layer 2 solutions.
//! It includes support for BOB (Bitcoin Optimistic Blockchain), Lightning Network,
//! and other Layer 2 scaling solutions.

pub mod bob;
pub mod lightning;

// Re-export key types for easier access
pub use bob::{Layer2Client as BobClient, Layer2Config as BobConfig, Layer2Error as BobError};

/// Layer 2 type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layer2Type {
    /// BOB Hybrid L2
    Bob,
    /// Lightning Network
    Lightning,
    /// State Channels
    StateChannel,
    /// Sidechains
    Sidechain,
    /// RGB Protocol
    Rgb,
    /// RSK Sidechain
    Rsk,
    /// Stacks Blockchain
    Stacks,
    /// Discreet Log Contracts
    Dlc,
    /// Taproot Assets
    TaprootAssets,
}

impl std::fmt::Display for Layer2Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Layer2Type::Bob => write!(f, "BOB"),
            Layer2Type::Lightning => write!(f, "Lightning Network"),
            Layer2Type::StateChannel => write!(f, "State Channel"),
            Layer2Type::Sidechain => write!(f, "Sidechain"),
            Layer2Type::Rgb => write!(f, "RGB Protocol"),
            Layer2Type::Rsk => write!(f, "RSK"),
            Layer2Type::Stacks => write!(f, "Stacks"),
            Layer2Type::Dlc => write!(f, "DLC"),
            Layer2Type::TaprootAssets => write!(f, "Taproot Assets"),
        }
    }
}

/// Configuration for the Layer 2 manager
#[derive(Clone, Debug)]
pub struct Layer2ManagerConfig {
    /// BOB L2 configuration
    pub bob_config: Option<bob::BobConfig>,
    /// Lightning configuration
    pub lightning_config: Option<lightning::LightningConfig>,
    /// Enable/disable specific Layer 2 solutions
    pub enabled_solutions: Vec<Layer2Type>,
}

impl Default for Layer2ManagerConfig {
    fn default() -> Self {
        Self {
            bob_config: Some(bob::BobConfig::default()),
            lightning_config: None,
            enabled_solutions: vec![Layer2Type::Bob],
        }
    }
}

/// Error types for the Layer 2 manager
#[derive(Debug, thiserror::Error)]
pub enum Layer2ManagerError {
    /// Solution not supported
    #[error("Layer 2 solution not supported: {0}")]
    SolutionNotSupported(String),

    /// Solution not enabled
    #[error("Layer 2 solution not enabled: {0}")]
    SolutionNotEnabled(String),

    /// BOB error
    #[error("BOB error: {0}")]
    Bob(#[from] bob::BobError),

    /// Lightning error
    #[error("Lightning error: {0}")]
    Lightning(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),
}

/// Result type for Layer 2 manager operations
pub type Layer2Result<T> = Result<T, Layer2ManagerError>;

/// Layer 2 solution status information
#[derive(Debug, Clone)]
pub struct Layer2Status {
    /// Layer 2 type
    pub l2_type: Layer2Type,
    /// Whether the solution is enabled
    pub enabled: bool,
    /// Whether the solution is connected/healthy
    pub connected: bool,
    /// Version information if available
    pub version: Option<String>,
    /// Additional status details
    pub details: std::collections::HashMap<String, String>,
}

/// The Layer 2 manager provides a unified interface for all Layer 2 solutions
pub struct Layer2Manager {
    /// Configuration
    config: Layer2ManagerConfig,
    /// BOB client if enabled
    bob_client: Option<bob::BobClient>,
    /// Lightning client if enabled
    lightning_client: Option<lightning::LightningClient>,
}

impl Layer2Manager {
    /// Create a new Layer 2 manager with the provided configuration
    pub fn new(config: Layer2ManagerConfig) -> Self {
        let bob_client = if config.enabled_solutions.contains(&Layer2Type::Bob) {
            config.bob_config.clone().map(bob::BobClient::new)
        } else {
            None
        };

        let lightning_client = if config.enabled_solutions.contains(&Layer2Type::Lightning) {
            config.lightning_config.clone().map(|_| {
                // TODO: Replace with actual Lightning client initialization
                lightning::LightningClient::default()
            })
        } else {
            None
        };

        Self {
            config,
            bob_client,
            lightning_client,
        }
    }

    /// Get a list of supported Layer 2 solution types
    pub fn get_supported_types(&self) -> Vec<Layer2Type> {
        vec![
            Layer2Type::Bob,
            Layer2Type::Lightning,
            Layer2Type::StateChannel,
            Layer2Type::Sidechain,
        ]
    }

    /// Get a list of enabled Layer 2 solution types
    pub fn get_enabled_types(&self) -> Vec<Layer2Type> {
        self.config.enabled_solutions.clone()
    }

    /// Check if a Layer 2 solution type is enabled
    pub fn is_enabled(&self, l2_type: Layer2Type) -> bool {
        self.config.enabled_solutions.contains(&l2_type)
    }

    /// Get the status of a specific Layer 2 solution
    pub async fn get_status(&self, l2_type: Layer2Type) -> Layer2Result<Layer2Status> {
        if !self.is_enabled(l2_type) {
            return Err(Layer2ManagerError::SolutionNotEnabled(l2_type.to_string()));
        }

        match l2_type {
            Layer2Type::Bob => {
                if let Some(client) = &self.bob_client {
                    let connected = client.check_health().await.unwrap_or(false);
                    let mut details = std::collections::HashMap::new();
                    
                    if connected {
                        if let Ok(relay_status) = client.get_relay_status().await {
                            details.insert("last_block_height".to_string(), relay_status.last_block_height.to_string());
                            details.insert("is_synced".to_string(), relay_status.is_synced.to_string());
                        }
                    }

                    Ok(Layer2Status {
                        l2_type,
                        enabled: true,
                        connected,
                        version: Some("1.0.0".to_string()),
                        details,
                    })
                } else {
                    Err(Layer2ManagerError::SolutionNotEnabled(l2_type.to_string()))
                }
            },
            Layer2Type::Lightning => {
                if let Some(_client) = &self.lightning_client {
                    // TODO: Implement Lightning status check
                    Ok(Layer2Status {
                        l2_type,
                        enabled: true,
                        connected: false,
                        version: Some("0.1.0".to_string()),
                        details: std::collections::HashMap::new(),
                    })
                } else {
                    Err(Layer2ManagerError::SolutionNotEnabled(l2_type.to_string()))
                }
            },
            _ => Err(Layer2ManagerError::SolutionNotSupported(l2_type.to_string())),
        }
    }

    /// Get the status of all enabled Layer 2 solutions
    pub async fn get_all_status(&self) -> Vec<Layer2Result<Layer2Status>> {
        let mut results = Vec::new();
        
        for l2_type in &self.config.enabled_solutions {
            results.push(self.get_status(*l2_type).await);
        }
        
        results
    }

    /// Get the BOB client if enabled
    pub fn bob_client(&self) -> Option<&bob::BobClient> {
        self.bob_client.as_ref()
    }

    /// Get the Lightning client if enabled
    pub fn lightning_client(&self) -> Option<&lightning::LightningClient> {
        self.lightning_client.as_ref()
    }
}

// Placeholder modules for future implementations
pub mod rgb {
    //! RGB Protocol integration module
    //! To be implemented in Q3 2025
}

pub mod rsk {
    //! RSK Sidechain integration module
    //! To be implemented in Q3 2025
}

pub mod stacks {
    //! Stacks Blockchain integration module
    //! To be implemented in Q3 2025
}

pub mod dlc {
    //! Discreet Log Contracts integration module
    //! To be implemented in Q3 2025
}

pub mod taproot {
    //! Taproot Assets integration module
    //! To be implemented in Q2 2025
}

// Future modules to be implemented
mod state_channels;
mod sidechains; 