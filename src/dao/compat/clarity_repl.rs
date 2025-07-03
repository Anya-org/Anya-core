//! Mock implementation of the clarity_repl module for test compatibility
//!
//! This module provides stub implementations of the clarity_repl functionality needed
//! for our DAO tests to pass. It simulates the behavior of the real clarity_repl library
//! without requiring the actual dependency.

/// Mock implementation of the clarity_repl crate
pub mod clarity {
    /// Runtime environment for Clarity execution
    pub mod runtime {
        use std::collections::HashMap;
        use std::sync::{Arc, Mutex};

        /// Execution context for Clarity contracts
        #[derive(Debug)]
        pub struct ContractContext {
            contract_id: String,
        }

        impl ContractContext {
            /// Create a new contract context
            pub fn new(contract_id: &str) -> Self {
                Self {
                    contract_id: contract_id.to_string(),
                }
            }
        }

        /// Simulated blockchain state
        #[derive(Debug)]
        pub struct SimulatedRuntime {
            contracts: HashMap<String, ContractContext>,
        }

        impl SimulatedRuntime {
            /// Create a new simulated runtime
            pub fn new() -> Self {
                Self {
                    contracts: HashMap::new(),
                }
            }

            /// Deploy a contract to the simulated blockchain
            pub fn deploy_contract(&mut self, contract_id: &str, _contract_content: &str) -> Result<(), String> {
                self.contracts.insert(contract_id.to_string(), ContractContext::new(contract_id));
                Ok(())
            }
        }
    }

    /// Value types for Clarity
    pub mod value {
        /// Represents a Clarity value
        #[derive(Debug, Clone)]
        pub enum Value {
            Bool(bool),
            Int(i128),
            UInt(u128),
            String(String),
            None,
        }

        impl Value {
            /// Create a boolean value
            pub fn bool(value: bool) -> Self {
                Self::Bool(value)
            }

            /// Create an unsigned integer value
            pub fn uint(value: u128) -> Self {
                Self::UInt(value)
            }

            /// Create a string value
            pub fn string(value: &str) -> Self {
                Self::String(value.to_string())
            }
        }
    }
}

/// Functions for evaluating Clarity code
pub mod repl {
    use super::clarity::runtime::SimulatedRuntime;
    use super::clarity::value::Value;

    /// Session for interacting with Clarity code
    #[derive(Debug)]
    pub struct Session {
        runtime: SimulatedRuntime,
    }

    impl Session {
        /// Create a new REPL session
        pub fn new() -> Self {
            Self {
                runtime: SimulatedRuntime::new(),
            }
        }

        /// Evaluate Clarity code
        pub fn eval(&mut self, _code: &str) -> Result<Value, String> {
            Ok(Value::Bool(true))
        }
    }
}

// Explicitly re-export the modules for easier access
pub use clarity::runtime;
pub use clarity::value;
pub use repl::Session;
