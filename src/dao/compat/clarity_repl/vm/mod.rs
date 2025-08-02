//! Stacks VM types compatibility layer
//!
//! This module contains types compatible with Stacks VM/Clarity

/// Principal data types
#[derive(Debug, Clone, PartialEq)]
pub struct PrincipalData {
    pub address: String,
    pub contract_name: Option<String>,
}

impl PrincipalData {
    /// Create a new PrincipalData from an address string
    pub fn from(address: String) -> Self {
        if address.contains(".") {
            let parts: Vec<&str> = address.split(".").collect();
            Self {
                address: parts[0].to_string(),
                contract_name: Some(parts[1].to_string()),
            }
        } else {
            Self {
                address,
                contract_name: None,
            }
        }
    }
}

/// Clarity value types
#[derive(Debug, Clone)]
pub enum Value {
    Int(i128),
    UInt(u128),
    Bool(bool),
    Principal(PrincipalData),
    None,
    Some(Box<Value>),
    String(String),
    Sequence(Vec<Value>),
}

/// Stacks transaction for contract calls
#[derive(Debug)]
pub struct StacksTransaction {
    pub contract_call: String,
    pub function_name: String,
    pub args: Vec<Value>,
    pub sender: Option<PrincipalData>,
}
