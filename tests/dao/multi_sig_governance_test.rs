// Multi-sig governance tests using REAL anya_core types and functionality
use anya_core::dao::compat::clarity_repl::repl::{
    ReadOnlyRequest, Session, TestEnvironment, TransactionRequest,
};
use anya_core::dao::compat::clarity_repl::vm::{PrincipalData, StacksTransaction, Value};
use anya_core::layer2::stacks::{StacksClient, StacksConfig};

#[test]
fn test_multi_sig_governance_with_real_stacks_client() {
    if std::env::var("ANYA_NETWORK_TEST").ok().as_deref() != Some("1") {
        eprintln!("[skip] Stacks client network test disabled");
        return;
    }
    // Test using actual StacksClient from anya_core with real configuration
    let config = StacksConfig {
        network: "testnet".to_string(),
        rpc_url: "https://stacks-node-api.testnet.stacks.co".to_string(),
        pox_enabled: true,
        timeout_ms: 30000,
    };

    let stacks_client = StacksClient::new(config);

    // Test deploying a REAL multi-sig governance contract
    let multi_sig_contract = r#"
        ;; Multi-sig governance contract
        (define-map signers principal bool)
        (define-map proposals uint {proposer: principal, action: (string-ascii 256), votes: uint, threshold: uint})
        (define-data-var proposal-id uint u0)
        (define-data-var required-threshold uint u2)
        
        (define-public (add-signer (signer principal))
            (begin
                (asserts! (is-eq tx-sender contract-caller) (err u401))
                (map-set signers signer true)
                (ok true)))
        
        (define-public (create-proposal (action (string-ascii 256)))
            (let ((new-id (+ (var-get proposal-id) u1)))
                (map-set proposals new-id {
                    proposer: tx-sender,
                    action: action,
                    votes: u1,
                    threshold: (var-get required-threshold)})
                (var-set proposal-id new-id)
                (ok new-id)))
    "#;

    let deploy_result =
        stacks_client.deploy_clarity_contract(multi_sig_contract, "multi-sig-governance");

    assert!(deploy_result.is_ok());
    let contract_id = deploy_result.unwrap();
    assert!(contract_id.contains("multi-sig-governance"));

    // Test protocol state and functionality
    let state_result = stacks_client.get_state();
    assert!(state_result.is_ok());
    let state = state_result.unwrap();
    assert!(!state.version.is_empty());
}

#[test]
fn test_clarity_vm_types() {
    // Test using actual Value types from compat module
    let deployer_principal =
        PrincipalData::from("ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM".to_string());
    let deployer_value = Value::Principal(deployer_principal);

    let threshold_value = Value::UInt(2);
    let valid_signer = Value::Bool(true);

    // Verify the types work correctly
    match deployer_value {
        Value::Principal(principal) => {
            assert_eq!(
                principal.address,
                "ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM"
            );
        }
        _ => panic!("Expected Principal value"),
    }

    match threshold_value {
        Value::UInt(val) => assert_eq!(val, 2),
        _ => panic!("Expected UInt value"),
    }

    match valid_signer {
        Value::Bool(val) => assert!(val),
        _ => panic!("Expected Bool value"),
    }
}

#[test]
fn test_stacks_transaction_creation() {
    // Test creating StacksTransaction with real types
    let tx = StacksTransaction {
        contract_call: "multi-sig-governance".to_string(),
        function_name: "add-signer".to_string(),
        args: vec![
            Value::Principal(PrincipalData::from(
                "ST2CY5V39NHDPWSXMW9QDT3HC3GD6Q6XX4CFRK9AG".to_string(),
            )),
            Value::UInt(1),
        ],
        sender: Some(PrincipalData::from(
            "ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM".to_string(),
        )),
    };

    assert_eq!(tx.contract_call, "multi-sig-governance");
    assert_eq!(tx.function_name, "add-signer");
    assert_eq!(tx.args.len(), 2);
}

#[test]
fn test_clarity_session() {
    // Test using actual Session from compat module
    let mut session = Session::new();

    // Deploy a contract using real session
    let deploy_result = session.deploy_contract("multi-sig", "(define-data-var threshold uint u2)");

    assert!(deploy_result.is_ok());

    // Call a contract function
    let call_result = session.call_contract("multi-sig", "get-threshold", &[]);

    assert!(call_result.is_ok());
}

#[test]
fn test_multi_sig_workflow() {
    // Complete workflow test using actual types
    let config = StacksConfig {
        network: "testnet".to_string(),
        rpc_url: "https://stacks-node-api.testnet.stacks.co".to_string(),
        pox_enabled: false,
        timeout_ms: 5000,
    };

    let client = StacksClient::new(config);

    // Deploy multi-sig contract
    let contract_deploy = client.deploy_clarity_contract(
        r#"
        (define-data-var threshold uint u2)
        (define-map signers principal bool)
        
        (define-public (add-signer (signer principal))
            (begin
                (map-set signers signer true)
                (ok true)))
        "#,
        "multi-sig-dao",
    );

    assert!(contract_deploy.is_ok());

    // Add signers
    let add_signer_result = client.call_contract_function(
        &contract_deploy.unwrap(),
        "add-signer",
        &[Value::String(
            "ST2CY5V39NHDPWSXMW9QDT3HC3GD6Q6XX4CFRK9AG".to_string(),
        )],
    );

    assert!(add_signer_result.is_ok());
}

#[test]
fn test_stacks_asset_integration() {
    // Fixed test using current StacksConfig structure and API
    let config = StacksConfig {
        network: "testnet".to_string(),
        rpc_url: "http://localhost:20443".to_string(),
        pox_enabled: false,
        timeout_ms: 5000,
    };

    let stacks_client = StacksClient::new(config);

    // Test contract deployment (simulates asset issuance contract)
    let asset_contract = r#"
        ;; Simplified asset contract for testing
        (define-fungible-token multi-sig-token)
        (define-data-var token-name (string-ascii 32) "MultiSig Governance Token")
        (define-data-var token-symbol (string-ascii 8) "MSG")
        
        (define-public (mint (amount uint) (recipient principal))
            (ft-mint? multi-sig-token amount recipient))
        
        (define-public (transfer (amount uint) (sender principal) (recipient principal))
            (ft-transfer? multi-sig-token amount sender recipient))
    "#;

    let deploy_result = stacks_client.deploy_clarity_contract(asset_contract, "multi-sig-asset");
    assert!(deploy_result.is_ok());

    let contract_id = deploy_result.unwrap();
    assert!(contract_id.contains("multi-sig-asset"));

    // Test asset minting via contract call
    let mint_result = stacks_client.call_contract_function(
        &contract_id,
        "mint",
        &[
            Value::UInt(1000000), // amount
            Value::Principal(PrincipalData::from(
                "ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM".to_string(),
            )), // recipient
        ],
    );
    assert!(mint_result.is_ok());

    // Test asset transfer via contract call
    let transfer_result = stacks_client.call_contract_function(
        &contract_id,
        "transfer",
        &[
            Value::UInt(100), // amount
            Value::Principal(PrincipalData::from(
                "ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM".to_string(),
            )), // sender
            Value::Principal(PrincipalData::from(
                "ST2CY5V39NHDPWSXMW9QDT3HC3GD6Q6XX4CFRK9AG".to_string(),
            )), // recipient
        ],
    );
    assert!(transfer_result.is_ok());
}

#[test]
fn test_multi_sig_workflow_complete() {
    // Complete workflow test using actual types and functionality
    let config = StacksConfig {
        network: "testnet".to_string(),
        rpc_url: "https://stacks-node-api.testnet.stacks.co".to_string(),
        pox_enabled: true,
        timeout_ms: 30000,
    };

    let client = StacksClient::new(config);

    // Step 1: Deploy multi-sig contract
    let multi_sig_contract = r#"
        ;; Advanced Multi-sig Governance Contract
        (define-map signers principal {active: bool, weight: uint})
        (define-map proposals uint {
            proposer: principal,
            action: (string-ascii 256),
            votes: uint,
            threshold: uint,
            executed: bool,
            created-at: uint
        })
        (define-data-var proposal-id uint u0)
        (define-data-var required-threshold uint u3)
        (define-data-var total-signers uint u0)
        
        (define-public (add-signer (signer principal) (weight uint))
            (begin
                (asserts! (is-eq tx-sender contract-caller) (err u401))
                (map-set signers signer {active: true, weight: weight})
                (var-set total-signers (+ (var-get total-signers) u1))
                (ok true)))
        
        (define-public (create-proposal (action (string-ascii 256)))
            (let ((new-id (+ (var-get proposal-id) u1)))
                (map-set proposals new-id {
                    proposer: tx-sender,
                    action: action,
                    votes: u1,
                    threshold: (var-get required-threshold),
                    executed: false,
                    created-at: block-height
                })
                (var-set proposal-id new-id)
                (ok new-id)))
        
        (define-public (vote-on-proposal (proposal-id uint))
            (let ((proposal (unwrap! (map-get? proposals proposal-id) (err u404)))
                  (signer-info (unwrap! (map-get? signers tx-sender) (err u403))))
                (asserts! (get active signer-info) (err u403))
                (map-set proposals proposal-id 
                    (merge proposal {votes: (+ (get votes proposal) (get weight signer-info))}))
                (ok true)))
    "#;

    let deploy_result = client.deploy_clarity_contract(multi_sig_contract, "advanced-multi-sig");
    assert!(deploy_result.is_ok());

    // Step 2: Test contract function calls
    let add_signer_call = client.call_contract_function(
        "advanced-multi-sig",
        "add-signer",
        &[
            Value::String("ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM".to_string()),
            Value::String("2".to_string()),
        ],
    );
    assert!(add_signer_call.is_ok());

    // Step 3: Test proposal creation
    let create_proposal_call = client.call_contract_function(
        "advanced-multi-sig",
        "create-proposal",
        &[Value::String("Transfer 1000 STX to treasury".to_string())],
    );
    assert!(create_proposal_call.is_ok());

    // Step 4: Test voting
    let vote_call = client.call_contract_function(
        "advanced-multi-sig",
        "vote-on-proposal",
        &[Value::String("1".to_string())],
    );
    assert!(vote_call.is_ok());
}

#[test]
fn test_test_environment_integration() {
    // Test using the actual TestEnvironment
    let mut test_env = TestEnvironment::new();

    // Execute a transaction request
    let tx_request = TransactionRequest {
        contract_call: "multi-sig-governance".to_string(),
        function_name: "add-signer".to_string(),
        function_args: vec![
            Value::Principal(PrincipalData::from(
                "ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM".to_string(),
            )),
            Value::UInt(1),
        ],
        sender: PrincipalData::from("ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM".to_string()),
    };

    let tx_result = test_env.execute_transaction(tx_request);
    assert!(tx_result.is_ok());

    // Execute a read-only request
    let readonly_request = ReadOnlyRequest {
        contract_call: "multi-sig-governance".to_string(),
        function_name: "get-signer-count".to_string(),
        function_args: vec![],
    };

    let readonly_result = test_env.execute_read_only(readonly_request);
    assert!(readonly_result.is_ok());
}
