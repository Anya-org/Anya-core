//! Compatibility module for HSM types
//!
//! This module provides compatibility functionality for types used in the HSM module.
//! [AIR-3][AIS-3][BPC-3][RES-3]

use bitcoin::Network;
use crate::security::hsm::config::BitcoinNetworkType;
use crate::security::hsm::error::HsmAuditEvent as ErrorHsmAuditEvent;
use crate::security::hsm::types::HsmAuditEvent as TypesHsmAuditEvent;

/// Convert BitcoinNetworkType to bitcoin::Network
impl From<BitcoinNetworkType> for Network {
    fn from(network_type: BitcoinNetworkType) -> Self {
        match network_type {
            BitcoinNetworkType::Mainnet => Network::Bitcoin,
            BitcoinNetworkType::Testnet => Network::Testnet,
            BitcoinNetworkType::Regtest => Network::Regtest,
            BitcoinNetworkType::Signet => Network::Signet,
        }
    }
}

/// Convert bitcoin::Network to BitcoinNetworkType
impl From<Network> for BitcoinNetworkType {
    fn from(network: Network) -> Self {
        match network {
            Network::Bitcoin => BitcoinNetworkType::Mainnet,
            Network::Testnet => BitcoinNetworkType::Testnet,
            Network::Regtest => BitcoinNetworkType::Regtest,
            Network::Signet => BitcoinNetworkType::Signet,
            _ => BitcoinNetworkType::Testnet, // Default to testnet for any other networks
        }
    }
}

/// Convert ErrorHsmAuditEvent to TypesHsmAuditEvent
impl From<&ErrorHsmAuditEvent> for TypesHsmAuditEvent {
    fn from(event: &ErrorHsmAuditEvent) -> Self {
        TypesHsmAuditEvent {
            event_type: event.event_type.to_string(),
            provider: format!("{:?}", event.result),  // Best effort conversion
            status: format!("{:?}", event.result),    // Use result as status
            details: event.error.clone(),             // Use error as details
            operation_id: event.key_id.clone(),       // Use key_id as operation_id (best effort)
        }
    }
}

/// Convert TypesHsmAuditEvent to ErrorHsmAuditEvent
impl From<&TypesHsmAuditEvent> for Result<ErrorHsmAuditEvent, crate::security::hsm::error::HsmError> {
    fn from(event: &TypesHsmAuditEvent) -> Self {
        use crate::security::hsm::error::{AuditEventType, AuditEventResult, AuditEventSeverity};
        
        // Convert string event_type to AuditEventType enum
        let event_type = match event.event_type.as_str() {
            "hsm.initialize" => AuditEventType::HsmInitialize,
            "hsm.operation" => AuditEventType::HsmOperation,
            "health_check" => AuditEventType::HealthCheck,
            "key.generate" => AuditEventType::KeyGeneration,
            "key.delete" => AuditEventType::KeyDeletion,
            _ => AuditEventType::Custom(event.event_type.clone()),
        };
        
        // Convert status string to result enum
        let result = match event.status.as_str() {
            "success" | "completed" => AuditEventResult::Success,
            "failed" | "error" => AuditEventResult::Failure,
            _ => AuditEventResult::Unknown,
        };
        
        // Choose severity based on result
        let severity = if result == AuditEventResult::Success {
            AuditEventSeverity::Info
        } else {
            AuditEventSeverity::Error
        };
        
        // Create new audit event
        let mut audit_event = ErrorHsmAuditEvent::new(
            event_type,
            result,
            severity,
        );
        
        // Add details if available
        if let Some(details) = &event.details {
            if let Ok(event_with_metadata) = audit_event.with_metadata(&serde_json::json!({ "details": details })) {
                audit_event = event_with_metadata;
            }
        }
        
        // Add operation_id if available
        if let Some(op_id) = &event.operation_id {
            if let Ok(event_with_params) = audit_event.with_parameters(&serde_json::json!({ "operation_id": op_id })) {
                audit_event = event_with_params;
            }
        }
        
        Ok(audit_event);
        let mut audit_event = ErrorHsmAuditEvent::new(
            event_type,
            result,
            severity,
        );
        
        // Add details if available using with_metadata
        if let Some(details) = &event.details {
            if let Ok(event_with_metadata) = audit_event.with_metadata(&serde_json::json!({ "details": details })) {
                audit_event = event_with_metadata;
            }
        }
        
        // Add operation_id if available using with_parameters
        if let Some(op_id) = &event.operation_id {
            if let Ok(event_with_params) = audit_event.with_parameters(&serde_json::json!({ "operation_id": op_id })) {
                audit_event = event_with_params;
            }
        }
        
        Ok(audit_event)
    }
}
