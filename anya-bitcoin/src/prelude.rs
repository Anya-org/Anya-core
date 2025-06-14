// Prelude module for common imports
// This module exports common types, traits, and functions used throughout the codebase

// Re-export core error types
pub use crate::core::error::{AnyaError, AnyaResult};

// Re-export Bitcoin types
pub use bitcoin::blockdata::script::Script;
pub use bitcoin::blockdata::transaction::{OutPoint, Sequence, TxIn, TxOut};
pub use bitcoin::hash_types::*;
pub use bitcoin::Address;
pub use bitcoin::Network;
pub use bitcoin::{Block, BlockHash, Transaction, Txid};

// Re-export Layer 2 types
pub use crate::layer2::framework::ProtocolConfig;
pub use crate::layer2::framework::{Layer2Protocol, TransactionStatus};

// Re-export port types
pub use crate::ports::layer2_port::{
    AssetId, AssetParams, AssetState, AssetTransfer, ProtocolState, ProtocolTransaction,
    SyncResult, TransactionId, TransferResult, VerificationResult,
};

// Re-export standard library types
pub use std::collections::HashMap;
pub use std::error::Error as StdError;
pub use std::fmt::{self, Debug, Display};
pub use std::path::PathBuf;
pub use std::sync::{Arc, Mutex, RwLock};
pub use std::time::{Duration, Instant};

// Re-export async utilities
pub use async_trait::async_trait;
pub use std::time;

// Re-export serde
pub use serde::{Deserialize, Serialize};

// Re-export logging
pub use tracing::{debug, error, info, trace, warn};
