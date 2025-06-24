// Decentralized Reporting System Test
// Tests the integration between multi-signature governance and reporting system

use clarity::vm::database::MemoryBackingStore;
use clarity::vm::types::{PrincipalData, QualifiedContractIdentifier, Value};
use clarity::vm::{execute as vm_execute, ClarityVersion};
use std::convert::TryFrom;

#[test]
fn test_decentralized_reporting_system() {
    // Setup the database
    let mut marf = MemoryBackingStore::new();
    
    // Deploy constants contract
    let dao_constants_contract_id = QualifiedContractIdentifier::new(
        PrincipalData::parse("ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM").unwrap(),
        "dao-constants".into(),
    );

    // Deploy governance traits contract
    let governance_traits_contract_id = QualifiedContractIdentifier::new(
        PrincipalData::parse("ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM").unwrap(),
        "governance-traits".into(),
    );

    // Deploy multi-sig governance contract
    let multi_sig_contract_id = QualifiedContractIdentifier::new(
        PrincipalData::parse("ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM").unwrap(),
        "multi-sig-governance".into(),
    );

    // Deploy reporting system contract
    let reporting_system_contract_id = QualifiedContractIdentifier::new(
        PrincipalData::parse("ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM").unwrap(),
        "reporting-system".into(),
    );

    // Test signers
    let signer1 = PrincipalData::parse("ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM").unwrap();
    let signer2 = PrincipalData::parse("ST2CY5V39NHDPWSXMW9QDT3HC3GD6Q6XX4CFRK9AG").unwrap();
    let signer3 = PrincipalData::parse("ST2JHG361ZXG51QTKY2NQCVBPPRRE2KZB1HR05NNC").unwrap();
    
    // Test non-signer
    let non_signer = PrincipalData::parse("ST2NEB84ASENDXKYGJPQW86YXQCEFEX2ZQPG87ND").unwrap();

    // Test cases:
    
    // 1. Non-governance contract cannot add a report generator
    let result = vm_execute(
        &reporting_system_contract_id,
        "add-report-generator",
        &[Value::Principal(non_signer.clone())],
        &non_signer,
        &mut marf,
        ClarityVersion::Clarity2,
    )
    .unwrap();
    
    // Should return error code 401 (unauthorized)
    assert!(result.to_string().contains("err u401"));

    // 2. Multi-sig proposal to add a report generator
    
    // First, propose the transaction from signer1
    let tx_id_result = vm_execute(
        &multi_sig_contract_id,
        "propose-transaction",
        &[Value::string_ascii_from_bytes(
            ".reporting-system.add-report-generator ST2NEB84ASENDXKYGJPQW86YXQCEFEX2ZQPG87ND".as_bytes().to_vec()
        ).unwrap()],
        &signer1,
        &mut marf,
        ClarityVersion::Clarity2,
    )
    .unwrap();
    
    let tx_id = match tx_id_result {
        Value::Response(response) => {
            match *response.data {
                Value::UInt(id) => id,
                _ => panic!("Expected uint, got {:?}", response.data),
            }
        },
        _ => panic!("Expected response, got {:?}", tx_id_result),
    };
    
    // Second signer signs the transaction
    vm_execute(
        &multi_sig_contract_id,
        "sign-transaction",
        &[Value::UInt(tx_id)],
        &signer2,
        &mut marf,
        ClarityVersion::Clarity2,
    )
    .unwrap();
    
    // Execute the transaction (needs manual execution since we're mocking the blockchain)
    vm_execute(
        &multi_sig_contract_id,
        "execute-transaction",
        &[Value::UInt(tx_id)],
        &signer1,
        &mut marf,
        ClarityVersion::Clarity2,
    )
    .unwrap();

    // 3. Verify that the report generator was added
    // Now the previously non-authorized principal should be able to generate reports
    let generate_result = vm_execute(
        &reporting_system_contract_id,
        "generate-report",
        &[Value::UInt(1), Value::Bool(true), Value::list_from([])],
        &non_signer,
        &mut marf,
        ClarityVersion::Clarity2,
    )
    .unwrap();
    
    // Should return success with a report ID
    assert!(generate_result.to_string().starts_with("(ok u"));

    // 4. Multi-sig proposal to update reporting settings
    let tx_id_result = vm_execute(
        &multi_sig_contract_id,
        "propose-transaction",
        &[Value::string_ascii_from_bytes(
            ".reporting-system.update-reporting-settings false u2000 u2000000 \"confidential\"".as_bytes().to_vec()
        ).unwrap()],
        &signer1,
        &mut marf,
        ClarityVersion::Clarity2,
    )
    .unwrap();
    
    let tx_id = match tx_id_result {
        Value::Response(response) => {
            match *response.data {
                Value::UInt(id) => id,
                _ => panic!("Expected uint, got {:?}", response.data),
            }
        },
        _ => panic!("Expected response, got {:?}", tx_id_result),
    };
    
    // Second and third signers sign the transaction
    vm_execute(
        &multi_sig_contract_id,
        "sign-transaction",
        &[Value::UInt(tx_id)],
        &signer2,
        &mut marf,
        ClarityVersion::Clarity2,
    )
    .unwrap();
    
    vm_execute(
        &multi_sig_contract_id,
        "sign-transaction",
        &[Value::UInt(tx_id)],
        &signer3,
        &mut marf,
        ClarityVersion::Clarity2,
    )
    .unwrap();
    
    // Execute the transaction
    vm_execute(
        &multi_sig_contract_id,
        "execute-transaction",
        &[Value::UInt(tx_id)],
        &signer1,
        &mut marf,
        ClarityVersion::Clarity2,
    )
    .unwrap();
    
    // 5. Verify that reporting is now disabled
    let report_attempt = vm_execute(
        &reporting_system_contract_id,
        "generate-scheduled-reports",
        &[],
        &non_signer,
        &mut marf,
        ClarityVersion::Clarity2,
    )
    .unwrap();
    
    // Should return error because reporting is disabled
    assert!(report_attempt.to_string().contains("err u403"));
}
