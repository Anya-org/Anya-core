//! Stacks REPL compatibility layer
//!
//! This module contains types compatible with Stacks REPL environment

use crate::dao::compat::clarity_repl::vm::{PrincipalData, Value};

/// REPL session for Clarity contract testing
#[derive(Debug)]
pub struct Session {
    pub contracts: Vec<String>,
    pub current_sender: PrincipalData,
}

impl Session {
    pub fn new() -> Self {
        Self {
            contracts: Vec::new(),
            current_sender: PrincipalData::from(
                "ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM".to_string(),
            ),
        }
    }

    pub fn deploy_contract(&mut self, name: &str, code: &str) -> Result<String, String> {
        self.contracts.push(format!("{}: {}", name, code));
        Ok(format!("Contract {} deployed", name))
    }

    pub fn call_contract(
        &self,
        _contract: &str,
        _function: &str,
        _args: &[Value],
    ) -> Result<Value, String> {
        // Mock implementation for testing
        Ok(Value::UInt(42))
    }
}

/// Test environment for Clarity contracts
#[derive(Debug)]
pub struct TestEnvironment {
    pub session: Session,
    pub epoch: u32,
}

impl TestEnvironment {
    pub fn new() -> Self {
        Self {
            session: Session::new(),
            epoch: 2, // Epoch 2.0
        }
    }

    pub fn execute_transaction(&mut self, request: TransactionRequest) -> Result<String, String> {
        // Mock implementation for testing
        Ok(format!(
            "Transaction executed: {}.{}",
            request.contract_call, request.function_name
        ))
    }

    pub fn execute_read_only(&self, _request: ReadOnlyRequest) -> Result<Value, String> {
        // Mock implementation for testing
        Ok(Value::UInt(3))
    }
}

/// Transaction request for REPL environment
#[derive(Debug)]
pub struct TransactionRequest {
    pub contract_call: String,
    pub function_name: String,
    pub function_args: Vec<Value>,
    pub sender: PrincipalData,
}

/// Read-only request for REPL environment
#[derive(Debug)]
pub struct ReadOnlyRequest {
    pub contract_call: String,
    pub function_name: String,
    pub function_args: Vec<Value>,
}
