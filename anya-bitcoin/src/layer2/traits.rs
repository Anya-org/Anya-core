//! Shared Layer 2 traits for federation, contract execution, and ML-driven logic
//! Used by all Layer 2 modules (RSK, DLC, Taproot Assets, RGB, Lightning, etc.)

use std::collections::HashMap;

/// Represents a generic proposal for federated action
pub trait Proposal {
    fn id(&self) -> &str;
    fn action(&self) -> &str;
    fn data(&self) -> &HashMap<String, String>;
}

/// Trait for multi-layer contract execution (e.g., BitVM, RSK, DLC, etc.)
pub trait ContractExecutor<P: Proposal> {
    /// Execute a contract action, returning a simulated or real txid
    fn execute_contract(&self, proposal: &P) -> Result<String, String>;
}

/// Trait for ML-driven federation logic (e.g., anomaly detection, adaptive threshold)
pub trait FederationMLHook<P: Proposal> {
    /// Called before approval/signing to allow ML-based checks or adjustments
    fn on_approve(&self, proposal: &P, member_id: &str) -> Result<(), String>;
    /// Called before execution to allow ML-based validation or veto
    fn on_execute(&self, proposal: &P) -> Result<(), String>;
}
