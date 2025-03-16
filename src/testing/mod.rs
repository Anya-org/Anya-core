#[derive(BitcoinProtocol)]
pub struct UnifiedTester {
    #[taproot_commitment]  // BPC-2 Compliance
    bitcoin_validator: Arc<dyn BitcoinValidator>,
    #[psbt_required]  // DAO-3 Requirement
    dao_verifier: DaoComplianceCheck,
    #[oracle_service]  // AIS-3 Implementation
    ai_monitor: AIMetricCollector,
}

impl UnifiedTester {
    /// Cross-component validation
    pub fn full_system_test(&self) -> Result<TestReport> {
        let bitcoin_health = self.bitcoin_validator.run_checks()?; // BIP-341/342 validation
        let dao_compliance = self.dao_verifier.verify_dao3_rules()?; // Quadratic voting checks
        let ai_perf = self.ai_monitor.collect_metrics()?; // AIR-3/AIS-3 validation
        
        Ok(TestReport {
            bitcoin: bitcoin_health,
            dao: dao_compliance,
            ai: ai_perf,
            system: self.check_interconnections()?,
        })
    }
}

// Core Testing Components:
// - Bitcoin Protocol Validator (BIP-341/342)
// - DAO-3 Governance Checker
// - AI Security Monitor (AIS-3)
// - Cross-Component Integration Tests 

//! Testing utilities for Anya-Core

pub mod performance;
pub mod sectional_test_utils;
// Other testing modules...

// Re-export performance test runner for convenience
pub use performance::runner::{run_comprehensive_test_suite, run_targeted_test}; 