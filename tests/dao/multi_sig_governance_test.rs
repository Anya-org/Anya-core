use anya_core::dao::compat::clarinet::{
    clarity::{
        types::{BuffData, Value},
        Address, Clarity, StacksAddress,
    },
    client::clarity_wasm::test::*,
    client::clarity_wasm::*,
    client::NetworkKeyCompression,
    client::*,
    client::Tx,
    client::Error as ClientError,
    types::StacksNetwork,
    utils::call_with_json_rpc,
};
use std::collections::HashMap;
use rand::rngs::OsRng;

/// Multi-signature governance integration test
/// 
/// This test verifies the functionality of the multi-sig governance contract
/// including proposing and executing transactions, adding/removing signers,
/// and changing thresholds.
#[test]
fn test_multi_sig_governance() {
    // Set up Clarinet environment
    let mut accounts = HashMap::new();
    let mut default_address = None;
    let contract_id = QualifiedContractIdentifier::local("multi-sig-governance").unwrap();
    
    // Setup test deployer and signers
    let deployer_address = StacksAddress::from_string("ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM").unwrap();
    let signer1_address = StacksAddress::from_string("ST2CY5V39NHDPWSXMW9QDT3HC3GD6Q6XX4CFRK9AG").unwrap();
    let signer2_address = StacksAddress::from_string("ST2JHG361ZXG51QTKY2NQCVBPPRRE2KZB1HR05NNC").unwrap();
    let non_signer_address = StacksAddress::from_string("ST2NEB84ASENDXKYGJPQW86YXQCEFEX2ZQPG87ND").unwrap();
    
    accounts.insert(deployer_address.clone(), 1_000_000_000);
    accounts.insert(signer1_address.clone(), 1_000_000_000);
    accounts.insert(signer2_address.clone(), 1_000_000_000);
    accounts.insert(non_signer_address.clone(), 1_000_000_000);
    default_address = Some(deployer_address.clone());
    
    let mut session = ClarityWasmSession::new(accounts, default_address);
    
    // Deploy contracts
    // First deploy the shared constants contract
    let dao_constants_identifier = QualifiedContractIdentifier::local("dao-constants").unwrap();
    let dao_constants_src = std::fs::read_to_string("contracts/dao/shared/dao-constants.clar").unwrap();
    session.deploy_contract(
        &dao_constants_identifier,
        &dao_constants_src,
        None,
    ).unwrap();
    
    // Next deploy the governance traits
    let governance_traits_identifier = QualifiedContractIdentifier::local("governance-traits").unwrap();
    let governance_traits_src = std::fs::read_to_string("contracts/dao/governance-traits.clar").unwrap();
    session.deploy_contract(
        &governance_traits_identifier,
        &governance_traits_src,
        None,
    ).unwrap();
    
    // Deploy the main multi-sig contract
    let src = std::fs::read_to_string("contracts/dao/multi-sig-governance.clar").unwrap();
    session.deploy_contract(
        &contract_id,
        &src,
        None,
    ).unwrap();
    
    // Test 1: Initial state validation
    println!("Test 1: Validating initial state");
    let result = session.call::<bool>(
        &contract_id,
        "is-valid-signer",
        &[Value::Principal(PrincipalData::from(deployer_address.clone()))],
    ).unwrap();
    assert_eq!(result, Value::Bool(true), "Deployer should be a valid signer");
    
    let result = session.call::<bool>(
        &contract_id,
        "is-valid-signer",
        &[Value::Principal(PrincipalData::from(signer1_address.clone()))],
    ).unwrap();
    assert_eq!(result, Value::Bool(true), "Signer1 should be a valid signer");
    
    let result = session.call::<bool>(
        &contract_id,
        "is-valid-signer",
        &[Value::Principal(PrincipalData::from(non_signer_address.clone()))],
    ).unwrap();
    assert_eq!(result, Value::Bool(false), "Non-signer should not be a valid signer");
    
    let result = session.call::<u128>(
        &contract_id,
        "get-threshold",
        &[],
    ).unwrap();
    assert_eq!(result, Value::UInt(2), "Initial threshold should be 2");
    
    let result = session.call::<u128>(
        &contract_id,
        "get-total-signers",
        &[],
    ).unwrap();
    assert_eq!(result, Value::UInt(3), "Initial signer count should be 3");
    
    // Test 2: Propose a transaction
    println!("Test 2: Proposing a transaction");
    let tx_sender = deployer_address.clone();
    let result = session.execute(
        &contract_id,
        "propose-transaction",
        &[Value::string_ascii_from_bytes("set-contract-owner".as_bytes()).unwrap()],
        tx_sender,
    ).unwrap();
    
    match result {
        Value::Response(response_data) => {
            match response_data.committed {
                true => {
                    assert_eq!(
                        response_data.data,
                        Value::UInt(1),
                        "Transaction ID should be 1"
                    );
                }
                false => panic!("Failed to propose transaction: {:?}", response_data.data),
            }
        }
        _ => panic!("Expected Response Value"),
    }
    
    // Test 3: Sign the transaction from a different signer
    println!("Test 3: Signing the transaction");
    let tx_sender = signer1_address.clone();
    let result = session.execute(
        &contract_id,
        "sign-transaction",
        &[Value::UInt(1)],
        tx_sender,
    ).unwrap();
    
    match result {
        Value::Response(response_data) => {
            match response_data.committed {
                true => {
                    assert_eq!(
                        response_data.data,
                        Value::UInt(1),
                        "Should return transaction ID 1"
                    );
                }
                false => panic!("Failed to sign transaction: {:?}", response_data.data),
            }
        }
        _ => panic!("Expected Response Value"),
    }
    
    // Test 4: Check transaction status
    println!("Test 4: Checking transaction status");
    let result = session.call::<HashMap<String, Value>>(
        &contract_id,
        "get-pending-transaction",
        &[Value::UInt(1)],
    ).unwrap();
    
    // Extract the signatures from the result and verify both signers have signed
    if let Value::Optional(opt) = result {
        if let Some(Value::Tuple(tuple_data)) = *opt {
            if let Some(Value::Sequence(signatures)) = tuple_data.get("signatures") {
                assert_eq!(signatures.len(), 2, "Should have 2 signatures");
                
                // Verify signatures include both deployer and signer1
                let has_deployer = signatures.iter().any(|sig| {
                    if let Value::Principal(principal) = sig {
                        principal.to_string().contains(&deployer_address.to_string())
                    } else {
                        false
                    }
                });
                
                let has_signer1 = signatures.iter().any(|sig| {
                    if let Value::Principal(principal) = sig {
                        principal.to_string().contains(&signer1_address.to_string())
                    } else {
                        false
                    }
                });
                
                assert!(has_deployer, "Deployer should have signed");
                assert!(has_signer1, "Signer1 should have signed");
            } else {
                panic!("Could not find signatures in transaction data");
            }
        } else {
            panic!("Expected tuple data in Optional");
        }
    } else {
        panic!("Expected Optional Value");
    }
    
    // Test 5: Add a new signer
    println!("Test 5: Adding a new signer");
    let tx_sender = deployer_address.clone();
    let result = session.execute(
        &contract_id,
        "propose-transaction",
        &[Value::string_ascii_from_bytes("add-new-signer").as_bytes().unwrap()],
        tx_sender,
    ).unwrap();
    
    // Get the transaction ID from the result
    let tx_id = match result {
        Value::Response(response_data) => {
            match response_data.committed {
                true => {
                    if let Value::UInt(id) = response_data.data {
                        id
                    } else {
                        panic!("Expected UInt Value for transaction ID");
                    }
                }
                false => panic!("Failed to propose transaction: {:?}", response_data.data),
            }
        }
        _ => panic!("Expected Response Value"),
    };
    
    // Sign the transaction with the second signer
    let tx_sender = signer2_address.clone();
    let result = session.execute(
        &contract_id,
        "sign-transaction",
        &[Value::UInt(tx_id)],
        tx_sender,
    ).unwrap();
    
    // Now that the threshold is met, attempt to add the new signer
    let tx_sender = deployer_address.clone();
    let result = session.execute(
        &contract_id,
        "add-signer",
        &[Value::Principal(PrincipalData::from(non_signer_address.clone()))],
        tx_sender,
    ).unwrap();
    
    match result {
        Value::Response(response_data) => {
            match response_data.committed {
                true => {
                    assert_eq!(
                        response_data.data,
                        Value::UInt(4),
                        "Total signers should now be 4"
                    );
                }
                false => panic!("Failed to add signer: {:?}", response_data.data),
            }
        }
        _ => panic!("Expected Response Value"),
    }
    
    // Test 6: Verify the new signer is valid
    println!("Test 6: Verifying the new signer");
    let result = session.call::<bool>(
        &contract_id,
        "is-valid-signer",
        &[Value::Principal(PrincipalData::from(non_signer_address.clone()))],
    ).unwrap();
    assert_eq!(result, Value::Bool(true), "New signer should now be a valid signer");
    
    // Test 7: Change the threshold
    println!("Test 7: Changing the threshold");
    let tx_sender = deployer_address.clone();
    let result = session.execute(
        &contract_id,
        "change-threshold",
        &[Value::UInt(3)],
        tx_sender,
    ).unwrap();
    
    match result {
        Value::Response(response_data) => {
            match response_data.committed {
                true => {
                    assert_eq!(
                        response_data.data,
                        Value::UInt(3),
                        "New threshold should be 3"
                    );
                }
                false => panic!("Failed to change threshold: {:?}", response_data.data),
            }
        }
        _ => panic!("Expected Response Value"),
    }
    
    let result = session.call::<u128>(
        &contract_id,
        "get-threshold",
        &[],
    ).unwrap();
    assert_eq!(result, Value::UInt(3), "Threshold should now be 3");
    
    println!("All multi-signature governance tests passed!");
}

/// Test decentralized contribution oracle functionality
#[test]
fn test_decentralized_contribution_oracle() {
    // Set up Clarinet environment
    let mut accounts = HashMap::new();
    let mut default_address = None;
    
    // Setup test addresses
    let deployer_address = StacksAddress::from_string("ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM").unwrap();
    let signer1_address = StacksAddress::from_string("ST2CY5V39NHDPWSXMW9QDT3HC3GD6Q6XX4CFRK9AG").unwrap();
    let signer2_address = StacksAddress::from_string("ST2JHG361ZXG51QTKY2NQCVBPPRRE2KZB1HR05NNC").unwrap();
    let oracle1_address = StacksAddress::from_string("ST2NEB84ASENDXKYGJPQW86YXQCEFEX2ZQPG87ND").unwrap();
    let oracle2_address = StacksAddress::from_string("ST2REHHS5J3CERCRBEPMGH7921Q6PYKAADT7JP2VB").unwrap();
    let oracle3_address = StacksAddress::from_string("ST3AM1A56AK2C1XAFJ4115ZSV26EB49BVQ10MGCS0").unwrap();
    
    accounts.insert(deployer_address.clone(), 1_000_000_000);
    accounts.insert(signer1_address.clone(), 1_000_000_000);
    accounts.insert(signer2_address.clone(), 1_000_000_000);
    accounts.insert(oracle1_address.clone(), 1_000_000_000);
    accounts.insert(oracle2_address.clone(), 1_000_000_000);
    accounts.insert(oracle3_address.clone(), 1_000_000_000);
    default_address = Some(deployer_address.clone());
    
    // Initialize session
    let mut session = ClarityWasmSession::new(accounts, default_address);
    
    // Define contract identifiers
    let dao_constants_id = QualifiedContractIdentifier::local("dao-constants").unwrap();
    let governance_traits_id = QualifiedContractIdentifier::local("governance-traits").unwrap();
    let multi_sig_governance_id = QualifiedContractIdentifier::local("multi-sig-governance").unwrap();
    let token_id = QualifiedContractIdentifier::local("token").unwrap();
    let decentralized_oracle_id = QualifiedContractIdentifier::local("decentralized-contribution-oracle").unwrap();
    
    // Deploy necessary contracts
    // 1. Deploy shared constants
    let dao_constants_src = std::fs::read_to_string("contracts/dao/shared/dao-constants.clar").unwrap();
    session.deploy_contract(
        &dao_constants_id,
        &dao_constants_src,
        None,
    ).unwrap();
    
    // 2. Deploy governance traits
    let governance_traits_src = std::fs::read_to_string("contracts/dao/governance-traits.clar").unwrap();
    session.deploy_contract(
        &governance_traits_id,
        &governance_traits_src,
        None,
    ).unwrap();
    
    // 3. Deploy multi-sig governance
    let multi_sig_governance_src = std::fs::read_to_string("contracts/dao/multi-sig-governance.clar").unwrap();
    session.deploy_contract(
        &multi_sig_governance_id,
        &multi_sig_governance_src,
        None,
    ).unwrap();
    
    // 4. Deploy token contract (simplified mock for testing)
    let token_src = r#"
    (define-trait ft-token-trait
      (
        (transfer (uint principal principal (optional (buff 34))) (response bool uint))
        (get-balance (principal) (response uint uint))
      )
    )
    
    (define-fungible-token anya-token u21000000000000000)
    
    (define-public (transfer (amount uint) (sender principal) (recipient principal) (memo (optional (buff 34))))
      (ft-transfer? anya-token amount sender recipient)
    )
    
    (define-public (get-balance (owner principal))
      (ok (ft-get-balance anya-token owner))
    )
    
    (define-public (mint (amount uint) (recipient principal))
      (ft-mint? anya-token amount recipient)
    )
    "#;
    
    session.deploy_contract(
        &token_id,
        token_src,
        None,
    ).unwrap();
    
    // Mint tokens for the oracles to stake
    let tx_sender = deployer_address.clone();
    for oracle_address in [oracle1_address.clone(), oracle2_address.clone(), oracle3_address.clone()].iter() {
        let result = session.execute(
            &token_id,
            "mint",
            &[Value::UInt(1000000000), Value::Principal(PrincipalData::from(oracle_address.clone()))],
            tx_sender.clone(),
        ).unwrap();
        
        match result {
            Value::Response(response_data) => {
                assert!(response_data.committed, "Failed to mint tokens for oracle");
            }
            _ => panic!("Expected Response Value"),
        }
    }
    
    // 5. Deploy decentralized oracle contract
    let decentralized_oracle_src = std::fs::read_to_string("contracts/dao/decentralized-contribution-oracle.clar").unwrap();
    session.deploy_contract(
        &decentralized_oracle_id,
        &decentralized_oracle_src,
        None,
    ).unwrap();
    
    // Test 1: Apply as oracle
    println!("Test 1: Apply as oracle");
    let tx_sender = oracle1_address.clone();
    let result = session.execute(
        &decentralized_oracle_id,
        "apply-as-oracle",
        &[Value::Principal(PrincipalData::from(token_id.clone()))],
        tx_sender.clone(),
    ).unwrap();
    
    match result {
        Value::Response(response_data) => {
            match response_data.committed {
                true => {
                    // Should return staked amount
                    assert!(response_data.data.to_string().contains("u100000000"), 
                        "Should have staked 100,000,000 tokens");
                }
                false => panic!("Failed to apply as oracle: {:?}", response_data.data),
            }
        }
        _ => panic!("Expected Response Value"),
    }
    
    // Test 2: Approve oracle application
    println!("Test 2: Approve oracle application");
    let tx_sender = deployer_address.clone();
    let result = session.execute(
        &decentralized_oracle_id,
        "approve-oracle-application",
        &[Value::Principal(PrincipalData::from(oracle1_address.clone()))],
        tx_sender,
    ).unwrap();
    
    match result {
        Value::Response(response_data) => {
            match response_data.committed {
                true => {
                    assert_eq!(
                        response_data.data,
                        Value::UInt(1),
                        "Should have 1 oracle now"
                    );
                }
                false => panic!("Failed to approve oracle: {:?}", response_data.data),
            }
        }
        _ => panic!("Expected Response Value"),
    }
    
    // Test 3: Check oracle status
    println!("Test 3: Check if address is an active oracle");
    let result = session.call::<bool>(
        &decentralized_oracle_id,
        "is-authorized-oracle",
        &[Value::Principal(PrincipalData::from(oracle1_address.clone()))],
    ).unwrap();
    assert_eq!(result, Value::Bool(true), "Oracle1 should be an active oracle");
    
    // Test 4: Add a few more oracles
    println!("Test 4: Add more oracles");
    for oracle_address in [oracle2_address.clone(), oracle3_address.clone()].iter() {
        // Apply as oracle
        let tx_sender = oracle_address.clone();
        let result = session.execute(
            &decentralized_oracle_id,
            "apply-as-oracle",
            &[Value::Principal(PrincipalData::from(token_id.clone()))],
            tx_sender,
        ).unwrap();
        
        match result {
            Value::Response(response_data) => {
                assert!(response_data.committed, "Failed to apply as oracle");
            }
            _ => panic!("Expected Response Value"),
        }
        
        // Approve oracle application
        let tx_sender = deployer_address.clone();
        let result = session.execute(
            &decentralized_oracle_id,
            "approve-oracle-application",
            &[Value::Principal(PrincipalData::from(oracle_address.clone()))],
            tx_sender,
        ).unwrap();
        
        match result {
            Value::Response(response_data) => {
                assert!(response_data.committed, "Failed to approve oracle");
            }
            _ => panic!("Expected Response Value"),
        }
    }
    
    // Get total oracles
    let result = session.call::<u128>(
        &decentralized_oracle_id,
        "get-total-oracles",
        &[],
    ).unwrap();
    assert_eq!(result, Value::UInt(3), "Should have 3 oracles now");
    
    println!("All decentralized contribution oracle tests passed!");
}
