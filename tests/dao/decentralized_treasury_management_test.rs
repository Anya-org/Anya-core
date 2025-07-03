#[cfg(test)]
mod decentralized_treasury_management_tests {
    use anya_core::dao::compat::clarinet::test_runner::*;
    use anya_core::dao::compat::clarinet::types::*;
    use anya_core::dao::compat::clarinet::contract_helpers::*;
    use anya_core::dao::compat::clarinet::macros::*;
    use anya_core::dao::compat::clarinet::client::contracts::Contract;
    
    #[test]
    fn test_decentralized_treasury_management() {
        // Set up test environment with multiple signers
        let mut env = TestEnvironment::new();
        let signer_1 = env.add_account("signer_1", 1000);
        let signer_2 = env.add_account("signer_2", 1000);
        let signer_3 = env.add_account("signer_3", 1000);
        let non_signer = env.add_account("non_signer", 1000);
        
        // Deploy governance contracts
        let dao_constants = env.deploy_contract("dao-constants", &signer_1, "contracts/dao/shared/dao-constants.clar");
        let governance_traits = env.deploy_contract("governance-traits", &signer_1, "contracts/dao/governance-traits.clar");
        let multi_sig_governance = env.deploy_contract("multi-sig-governance", &signer_1, "contracts/dao/multi-sig-governance.clar");
        
        // Deploy treasury contract
        let treasury = env.deploy_contract("decentralized-treasury-management", &signer_1, "contracts/dao/decentralized-treasury-management.clar");
        
        // Test updating treasury parameters through governance
        // 1. First, propose a transaction from signer_1
        let result = env.execute_transaction(TransactionRequest {
            caller: signer_1.clone(),
            contract: multi_sig_governance.clone(),
            function_name: "propose-transaction".to_string(),
            args: vec![
                "\"ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.decentralized-treasury-management.update-treasury-parameters\"".into(),
                "u20".into(), // New reserve ratio min
                "u25".into(), // New POL ratio target
                "u35".into(), // New emergency threshold
            ],
        });
        
        // 2. Get the transaction ID
        let tx_id = match result.expect("Failed to propose transaction") {
            Value::UInt(id) => id,
            _ => panic!("Expected uint for transaction ID"),
        };
        
        // 3. Have signer_2 sign the transaction
        let result = env.execute_transaction(TransactionRequest {
            caller: signer_2.clone(),
            contract: multi_sig_governance.clone(),
            function_name: "sign-transaction".to_string(),
            args: vec![tx_id.into()],
        });
        result.expect("Failed to sign transaction");
        
        // 4. Execute the transaction (should reach threshold with 2 signers)
        let result = env.execute_transaction(TransactionRequest {
            caller: signer_1.clone(),
            contract: multi_sig_governance.clone(),
            function_name: "execute-transaction".to_string(),
            args: vec![tx_id.into()],
        });
        result.expect("Failed to execute transaction");
        
        // 5. Verify parameters were updated
        let result = env.execute_read_only(ReadOnlyRequest {
            caller: signer_1.clone(),
            contract: treasury.clone(),
            function_name: "get-treasury-ratios".to_string(),
            args: vec![],
        });
        
        if let Value::Object(ratios) = result.expect("Failed to get treasury ratios") {
            assert_eq!(
                ratios.get("reserve-ratio-min").unwrap(),
                &Value::UInt(20),
                "Reserve ratio min should be updated to 20"
            );
            assert_eq!(
                ratios.get("pol-ratio-target").unwrap(),
                &Value::UInt(25),
                "POL ratio target should be updated to 25"
            );
        } else {
            panic!("Expected object for treasury ratios");
        }
        
        // Test emergency activation through governance
        // 1. Propose emergency activation
        let result = env.execute_transaction(TransactionRequest {
            caller: signer_1.clone(),
            contract: multi_sig_governance.clone(),
            function_name: "propose-transaction".to_string(),
            args: vec![
                "\"ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.decentralized-treasury-management.activate-emergency\"".into(),
            ],
        });
        
        // 2. Get the transaction ID
        let tx_id = match result.expect("Failed to propose emergency activation") {
            Value::UInt(id) => id,
            _ => panic!("Expected uint for transaction ID"),
        };
        
        // 3. Have signer_2 and signer_3 sign the transaction
        let result = env.execute_transaction(TransactionRequest {
            caller: signer_2.clone(),
            contract: multi_sig_governance.clone(),
            function_name: "sign-transaction".to_string(),
            args: vec![tx_id.into()],
        });
        result.expect("Failed to sign emergency activation");
        
        let result = env.execute_transaction(TransactionRequest {
            caller: signer_3.clone(),
            contract: multi_sig_governance.clone(),
            function_name: "sign-transaction".to_string(),
            args: vec![tx_id.into()],
        });
        result.expect("Failed to sign emergency activation");
        
        // 4. Execute the transaction
        let result = env.execute_transaction(TransactionRequest {
            caller: signer_1.clone(),
            contract: multi_sig_governance.clone(),
            function_name: "execute-transaction".to_string(),
            args: vec![tx_id.into()],
        });
        result.expect("Failed to execute emergency activation");
        
        // 5. Verify emergency was activated
        let result = env.execute_read_only(ReadOnlyRequest {
            caller: signer_1.clone(),
            contract: treasury.clone(),
            function_name: "get-emergency-status".to_string(),
            args: vec![],
        });
        
        if let Value::Object(status) = result.expect("Failed to get emergency status") {
            assert_eq!(
                status.get("emergency-active").unwrap(),
                &Value::Bool(true),
                "Emergency should be activated"
            );
        } else {
            panic!("Expected object for emergency status");
        }
        
        // Test that non-governance contract cannot call protected functions
        let result = env.execute_transaction(TransactionRequest {
            caller: non_signer.clone(),
            contract: treasury.clone(),
            function_name: "update-treasury-parameters".to_string(),
            args: vec![
                "u10".into(), // New reserve ratio min
                "u15".into(), // New POL ratio target
                "u20".into(), // New emergency threshold
            ],
        });
        
        // Should fail with unauthorized error
        assert!(result.is_err(), "Non-governance caller should not be able to update parameters");
        
        // Test treasury operation proposal and execution
        // 1. Propose a buyback operation
        let result = env.execute_transaction(TransactionRequest {
            caller: signer_1.clone(),
            contract: multi_sig_governance.clone(),
            function_name: "propose-transaction".to_string(),
            args: vec![
                "\"ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.decentralized-treasury-management.propose-treasury-operation\"".into(),
                "u1".into(), // OP_BUYBACK
                "u1000000000".into(), // 10 tokens
                "none".into(), // No target for buyback
            ],
        });
        
        // 2. Get the transaction ID
        let tx_id = match result.expect("Failed to propose treasury operation") {
            Value::UInt(id) => id,
            _ => panic!("Expected uint for transaction ID"),
        };
        
        // 3. Have signer_2 sign the transaction
        let result = env.execute_transaction(TransactionRequest {
            caller: signer_2.clone(),
            contract: multi_sig_governance.clone(),
            function_name: "sign-transaction".to_string(),
            args: vec![tx_id.into()],
        });
        result.expect("Failed to sign treasury operation");
        
        // 4. Execute the transaction
        let result = env.execute_transaction(TransactionRequest {
            caller: signer_1.clone(),
            contract: multi_sig_governance.clone(),
            function_name: "execute-transaction".to_string(),
            args: vec![tx_id.into()],
        });
        
        // 5. Get the operation ID
        let op_id = match result.expect("Failed to execute treasury operation proposal") {
            Value::UInt(id) => id,
            _ => panic!("Expected uint for operation ID"),
        };
        
        // 6. Propose execution of the treasury operation
        let result = env.execute_transaction(TransactionRequest {
            caller: signer_1.clone(),
            contract: multi_sig_governance.clone(),
            function_name: "propose-transaction".to_string(),
            args: vec![
                "\"ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.decentralized-treasury-management.execute-treasury-operation\"".into(),
                op_id.into(), // Operation ID
            ],
        });
        
        // 7. Get the transaction ID
        let tx_id = match result.expect("Failed to propose execution") {
            Value::UInt(id) => id,
            _ => panic!("Expected uint for transaction ID"),
        };
        
        // 8. Have signer_2 sign the transaction
        let result = env.execute_transaction(TransactionRequest {
            caller: signer_2.clone(),
            contract: multi_sig_governance.clone(),
            function_name: "sign-transaction".to_string(),
            args: vec![tx_id.into()],
        });
        result.expect("Failed to sign execution");
        
        // 9. Execute the transaction
        let result = env.execute_transaction(TransactionRequest {
            caller: signer_1.clone(),
            contract: multi_sig_governance.clone(),
            function_name: "execute-transaction".to_string(),
            args: vec![tx_id.into()],
        });
        result.expect("Failed to execute treasury operation");
        
        // 10. Verify operation was executed
        let result = env.execute_read_only(ReadOnlyRequest {
            caller: signer_1.clone(),
            contract: treasury.clone(),
            function_name: "get-operation".to_string(),
            args: vec![op_id.into()],
        });
        
        if let Value::Optional(Some(box_value)) = result.expect("Failed to get operation") {
            if let Value::Object(operation) = *box_value {
                assert_eq!(
                    operation.get("executed").unwrap(),
                    &Value::Bool(true),
                    "Operation should be marked as executed"
                );
            } else {
                panic!("Expected object for operation");
            }
        } else {
            panic!("Expected optional with object for operation");
        }
    }
}
