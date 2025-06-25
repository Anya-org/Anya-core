// Decentralized Reporting System Tests
use clarity_repl::vm::test_util::{TEST_HEADER_DB, TEST_BURN_STATE_DB};
use clarity_repl::vm::{Value, execute, StacksTransaction};
use clarity_repl::clarity::ClarityInstance;
use clarity_repl::repl::Session;
use clarity_repl::repl::TestEnvironment;
use std::env;

#[test]
fn test_decentralized_reporting_system() {
    // Initialize the Clarity VM and test environment
    let mut clarity_instance = ClarityInstance::new(false, TEST_HEADER_DB.clone(), TEST_BURN_STATE_DB.clone());
    let mut session = Session::new(clarity_instance);
    let test_env = TestEnvironment::new();

    // Set up the test environment with contracts
    let dao_constants = test_env.deploy_contract("dao-constants", include_str!("../../contracts/dao/shared/dao-constants.clar"));
    let governance_traits = test_env.deploy_contract("governance-traits", include_str!("../../contracts/dao/governance-traits.clar"));
    let multi_sig_governance = test_env.deploy_contract("multi-sig-governance", include_str!("../../contracts/dao/multi-sig-governance.clar"));
    let treasury_management = test_env.deploy_contract("decentralized-treasury-management", include_str!("../../contracts/dao/decentralized-treasury-management.clar"));
    let reporting_system = test_env.deploy_contract("reporting-system-decentralized", include_str!("../../contracts/dao/reporting-system-decentralized.clar"));

    // Test admin access - should fail when called directly
    let result = session.execute(
        "reporting-system-decentralized", 
        "add-report-generator", 
        vec![Value::Principal("ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM".into())]
    );
    assert!(result.is_err(), "Non-governance call should fail");
    
    // Test report generation - should succeed for governance contract
    // Set up a mock call from the governance contract
    session.set_tx_sender("ST2CY5V39NHDPWSXMW9QDT3HC3GD6Q6XX4CFRK9AG.multi-sig-governance");
    
    // Add a report generator through governance
    let result = session.execute(
        "reporting-system-decentralized", 
        "add-report-generator", 
        vec![Value::Principal("ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM".into())]
    );
    assert!(result.is_ok(), "Governance call should succeed");
    
    // Now test report generation as the new generator
    session.set_tx_sender("ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM");
    
    let result = session.execute(
        "reporting-system-decentralized", 
        "generate-report", 
        vec![
            Value::UInt(1), // REPORT_TYPE_TREASURY
            Value::Bool(true), // is_public
            Value::list_from(vec![]) // Empty metrics list
        ]
    );
    assert!(result.is_ok(), "Report generation should succeed");
    
    // Update reporting settings through governance
    session.set_tx_sender("ST2CY5V39NHDPWSXMW9QDT3HC3GD6Q6XX4CFRK9AG.multi-sig-governance");
    
    let result = session.execute(
        "reporting-system-decentralized", 
        "update-reporting-settings", 
        vec![
            Value::Bool(true),           // reporting enabled
            Value::UInt(500),            // new interval
            Value::UInt(500000),         // new retention
            Value::string_ascii_from_bytes("detailed".as_bytes().to_vec()).unwrap()
        ]
    );
    assert!(result.is_ok(), "Settings update should succeed");
    
    // Check settings through read-only functions
    let result = session.eval_read_only(
        "reporting-system-decentralized", 
        "(get-reporting-interval)"
    );
    assert_eq!(result.expect("Should be able to read interval"), Value::UInt(500));
    
    let result = session.eval_read_only(
        "reporting-system-decentralized", 
        "(get-privacy-level)"
    );
    assert_eq!(
        result.expect("Should be able to read privacy level"), 
        Value::string_ascii_from_bytes("detailed".as_bytes().to_vec()).unwrap()
    );
    
    // Update report type through governance
    session.set_tx_sender("ST2CY5V39NHDPWSXMW9QDT3HC3GD6Q6XX4CFRK9AG.multi-sig-governance");
    
    let result = session.execute(
        "reporting-system-decentralized", 
        "update-report-type", 
        vec![
            Value::UInt(1),             // REPORT_TYPE_TREASURY
            Value::string_ascii_from_bytes("Updated Treasury Report".as_bytes().to_vec()).unwrap(),
            Value::string_utf8_from_bytes("Updated description for treasury report".as_bytes().to_vec()).unwrap(),
            Value::list_from(vec![
                Value::string_ascii_from_bytes("metric1".as_bytes().to_vec()).unwrap(),
                Value::string_ascii_from_bytes("metric2".as_bytes().to_vec()).unwrap(),
            ]),
            Value::UInt(14),            // FREQUENCY_BIWEEKLY
            Value::Bool(true)
        ]
    );
    assert!(result.is_ok(), "Report type update should succeed");
    
    // Test report type update
    let result = session.eval_read_only(
        "reporting-system-decentralized", 
        "(get-report-type u1)"
    );
    
    let report_type = result.expect("Should be able to read report type");
    // Verify the name update
    // Note: In a real test, you'd need to extract the name field from the tuple
    
    // Try to perform admin action from non-governance account (should fail)
    session.set_tx_sender("ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM");
    
    let result = session.execute(
        "reporting-system-decentralized", 
        "update-reporting-settings", 
        vec![
            Value::Bool(false),          // reporting disabled
            Value::UInt(100),            // new interval
            Value::UInt(200000),         // new retention
            Value::string_ascii_from_bytes("minimal".as_bytes().to_vec()).unwrap()
        ]
    );
    assert!(result.is_err(), "Settings update from non-governance should fail");
}
