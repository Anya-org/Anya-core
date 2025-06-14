use anya_core::dao::{
    ComplianceLevel, CrossChainGovernance, DaoGovernance, DaoLabel, ProposalStatus,
    VerificationMethod, VoteDirection,
};
use chrono::{Duration, Utc};
use pretty_assertions::{assert_eq, assert_ne};
use tokio;

#[test]
fn test_dao3_compliance() {
    let dao = DaoGovernance::new();
    assert!(
        dao.verify_dao3_compliance(),
        "DAO should be DAO-3 compliant by default"
    );

    // Test all components individually
    assert!(
        dao.quadratic_voting_enabled,
        "Quadratic voting should be enabled"
    );
    assert!(
        dao.delegated_authority.active,
        "Delegated authority should be active"
    );
    assert!(
        dao.cross_chain_governance.is_some(),
        "Cross-chain governance should be configured"
    );
    assert!(
        dao.legal_wrappers.is_implemented(),
        "Legal wrappers should be implemented"
    );
}

#[test]
fn test_dao4_compliance() {
    let mut dao = DaoGovernance::new();

    // Default should not be DAO-4 compliant until we set advanced features
    assert!(
        !dao.verify_dao4_compliance(),
        "New DAO should not be DAO-4 compliant"
    );

    // Upgrade legal wrappers to advanced
    dao.legal_wrappers
        .set_compliance_level(ComplianceLevel::Advanced);

    // Add Merkle verification method to cross-chain governance
    if let Some(ccg) = &mut dao.cross_chain_governance {
        ccg.add_chain("ethereum", "0x1234567890abcdef", VerificationMethod::Merkle);
    }

    // Now should be DAO-4 compliant
    assert!(
        dao.verify_dao4_compliance(),
        "DAO should now be DAO-4 compliant"
    );
}

#[test]
fn test_create_and_vote_on_proposal() {
    let mut dao = DaoGovernance::new();

    // Create proposal
    let proposal_id = dao
        .create_proposal(
            "Test Bitcoin Integration",
            "Integrate with Bitcoin using Taproot",
            "0xuser1",
            100,  // voting period in blocks
            25.0, // required quorum percentage
            51.0, // required approval percentage
        )
        .expect("Failed to create proposal");

    // Verify proposal creation
    let proposal = dao
        .get_proposal(proposal_id)
        .expect("Failed to get proposal");
    assert_eq!(proposal.title, "Test Bitcoin Integration");
    assert_eq!(proposal.proposer, "0xuser1");
    assert_eq!(proposal.status, ProposalStatus::Active);

    // Cast votes
    dao.vote(proposal_id, "0xvoter1", VoteDirection::For, 20.0)
        .expect("Failed to cast vote");
    dao.vote(proposal_id, "0xvoter2", VoteDirection::Against, 10.0)
        .expect("Failed to cast vote");
    dao.vote(proposal_id, "0xvoter3", VoteDirection::For, 15.0)
        .expect("Failed to cast vote");

    // Check votes were recorded
    let proposal = dao
        .get_proposal(proposal_id)
        .expect("Failed to get proposal");
    assert_eq!(proposal.votes.len(), 3, "Should have 3 votes");

    // Force end of voting period
    dao.update_block_height(proposal.end_block + 1);

    // Tally votes
    let proposal = dao
        .get_proposal(proposal_id)
        .expect("Failed to get proposal");

    // With quadratic voting, the weights should be:
    // 0xvoter1: sqrt(20) ≈ 4.47
    // 0xvoter2: sqrt(10) ≈ 3.16
    // 0xvoter3: sqrt(15) ≈ 3.87
    // Total "For": ~8.34, Total "Against": ~3.16
    // Approval should be about 72.5% which exceeds required 51%
    assert_eq!(
        proposal.status,
        ProposalStatus::Passed,
        "Proposal should have passed"
    );
}

#[test]
fn test_delegation() {
    let mut dao = DaoGovernance::new();

    // Create proposal
    let proposal_id = dao
        .create_proposal(
            "Test Taproot Asset Implementation",
            "Implement Taproot Assets according to BDF v2.5",
            "0xproposer",
            100,
            25.0,
            51.0,
        )
        .expect("Failed to create proposal");

    // Set up delegation
    let expiry = Utc::now() + Duration::days(7);
    dao.delegate_voting_power("0xdelegator", "0xdelegate", expiry, 0.8)
        .expect("Failed to delegate voting power");

    // Vote as delegator
    dao.vote(proposal_id, "0xdelegator", VoteDirection::For, 100.0)
        .expect("Failed to cast vote");

    // Check that vote was recorded under delegate
    let proposal = dao
        .get_proposal(proposal_id)
        .expect("Failed to get proposal");
    assert!(
        proposal.votes.contains_key("0xdelegate"),
        "Vote should be recorded under delegate"
    );

    let vote = proposal.votes.get("0xdelegate").expect("Vote not found");
    assert!(vote.is_delegated, "Vote should be marked as delegated");
    assert_eq!(
        vote.delegated_from
            .as_ref()
            .expect("Missing delegation source"),
        "0xdelegator"
    );

    // With quadratic voting and 0.8 power factor:
    // sqrt(100) * 0.8 ≈ 8.0
    assert!(
        (vote.power - 8.0).abs() < 0.1,
        "Power calculation is incorrect"
    );
}

#[test]
fn test_cross_chain_proposal() {
    let mut dao = DaoGovernance::new();

    // Create cross-chain proposal
    let proposal_id = dao
        .create_cross_chain_proposal(
            "RSK Integration",
            "Integrate with RSK sidechain",
            "0xproposer",
            100,
            25.0,
            51.0,
            &["rsk".to_string()],
        )
        .expect("Failed to create cross-chain proposal");

    // Verify proposal has target chains
    let proposal = dao
        .get_proposal(proposal_id)
        .expect("Failed to get proposal");
    assert_eq!(proposal.target_chains.len(), 1);
    assert_eq!(proposal.target_chains[0], "rsk");

    // Vote on proposal to pass it
    dao.vote(proposal_id, "0xvoter1", VoteDirection::For, 100.0)
        .expect("Failed to cast vote");

    // Force end of voting period
    dao.update_block_height(proposal.end_block + 1);

    // Execute the cross-chain proposal
    let result = dao.execute_cross_chain_proposal(proposal_id);
    assert!(result.is_ok(), "Failed to execute cross-chain proposal");

    // Check proposal status after execution
    let proposal = dao
        .get_proposal(proposal_id)
        .expect("Failed to get proposal");
    assert_eq!(proposal.status, ProposalStatus::Executed);
}

#[tokio::test]
async fn test_async_dao_interface() {
    // This test would verify the async DAO interface
    // In a real implementation, you would test against the actual async methods

    // For this example, we'll skip it since we don't have the full implementation
    // But in a real test, you would:
    // 1. Create a DAO instance that implements DaoGovernancePort
    // 2. Call async methods and verify results
    // 3. Test concurrent operations
}

#[test]
fn test_legal_wrappers() {
    let mut dao = DaoGovernance::new();

    // Verify default legal wrappers
    assert!(dao.legal_wrappers.is_implemented());
    assert_eq!(dao.legal_wrappers.legal_structure, "DAO LLC");
    assert_eq!(dao.legal_wrappers.jurisdiction, "Wyoming, USA");

    // Add legal document
    dao.legal_wrappers
        .add_document("terms", "https://example.com/terms");
    assert!(dao.legal_wrappers.legal_documents.contains_key("terms"));

    // Upgrade compliance level
    dao.legal_wrappers
        .set_compliance_level(ComplianceLevel::Full);
    assert_eq!(dao.legal_wrappers.compliance_level, ComplianceLevel::Full);
}
