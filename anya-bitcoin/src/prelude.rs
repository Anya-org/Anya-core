// Prelude module for common imports
// This module exports common types, traits, and functions used throughout the codebase

// Re-export core error types
pub use crate::core::error::{AnyaResult, AnyaError};

// Re-export Bitcoin types
pub use bitcoin::{Block, BlockHash, Transaction, Txid};
pub use bitcoin::hash_types::*;
pub use bitcoin::blockdata::script::Script;
pub use bitcoin::blockdata::transaction::{OutPoint, Sequence, TxIn, TxOut};
pub use bitcoin::Address;
pub use bitcoin::Network;

// Re-export Layer 2 types
pub use crate::layer2::framework::{Layer2Protocol, TransactionStatus};
pub use crate::layer2::framework::ProtocolConfig;

// Re-export port types
pub use crate::ports::layer2_port::{
    ProtocolTransaction,
    ProtocolState,
    TransactionId,
    VerificationResult,
    SyncResult,
    AssetParams,
    AssetId,
    AssetTransfer,
    TransferResult,
    AssetState
};

// Re-export standard library types
pub use std::collections::HashMap;
pub use std::sync::{Arc, RwLock, Mutex};
pub use std::path::PathBuf;
pub use std::fmt::{self, Debug, Display};
pub use std::time::{Duration, Instant};
pub use std::error::Error as StdError;

// Re-export async utilities
pub use async_trait::async_trait;
pub use tokio::time;

// Re-export serde
pub use serde::{Serialize, Deserialize};

// Re-export logging
pub use tracing::{info, warn, error, debug, trace}; 