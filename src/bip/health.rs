// [AIR-3][AIS-3][BPC-3][AIT-3] BIP System Health Implementation
// Comprehensive BIP status monitoring and health checking

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tracing::{debug, info};

use super::bip353::{Bip353, Bip353Status};
use super::validation::{BipValidator, BitcoinConfig, ComplianceStatus};

/// Industry adoption levels for BIPs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdoptionStatus {
    /// Used in production by multiple wallets/services
    Widespread,
    /// Used in production by at least one wallet/service
    Limited,
    /// Implemented but not used in production
    Experimental,
    /// Not yet implemented in any production system
    None,
    /// The BIP is a draft proposal
    Draft,
}

impl fmt::Display for AdoptionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AdoptionStatus::Widespread => write!(f, "Widespread"),
            AdoptionStatus::Limited => write!(f, "Limited"),
            AdoptionStatus::Experimental => write!(f, "Experimental"),
            AdoptionStatus::None => write!(f, "None"),
            AdoptionStatus::Draft => write!(f, "Draft"),
        }
    }
}

/// BIP detail with implementation information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BipDetail {
    /// BIP number (e.g., "BIP-341")
    pub bip: String,
    /// Human-readable name
    pub name: String,
    /// Short description
    pub description: String,
    /// Compliance status
    pub status: ComplianceStatus,
    /// Implementation details
    pub implementation: String,
    /// Whether this is a beta feature
    pub is_beta: bool,
    /// Test coverage percentage (0-100)
    pub test_coverage: Option<u8>,
    /// Audit status
    pub audit_status: Option<String>,
    /// Required for core functionality
    pub required: bool,
    /// Industry adoption status
    pub industry_adoption: AdoptionStatus,
    /// Links to relevant discussions or implementations
    pub references: Option<Vec<String>>,
    /// Last updated timestamp
    pub last_updated: Option<String>,
}

/// BIP Health Report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BipHealthReport {
    /// Timestamp of the report
    pub timestamp: String,
    /// Overall health status (true if all required BIPs are compliant)
    pub healthy: bool,
    /// Map of BIP details by BIP number
    pub bips: HashMap<String, BipDetail>,
    /// Map of proposals that aren't formal BIPs yet
    pub proposals: HashMap<String, BipDetail>,
    /// Total number of supported BIPs
    pub total_supported: usize,
    /// Number of BIPs in beta status
    pub beta_count: usize,
    /// Number of compliant BIPs
    pub compliant_count: usize,
    /// Number of partially compliant BIPs
    pub partial_count: usize,
    /// Number of missing BIPs
    pub missing_count: usize,
    /// Number of proposals tracked
    pub proposal_count: usize,
}

impl Default for BipHealthReport {
    fn default() -> Self {
        Self::new()
    }
}

impl BipHealthReport {
    /// Create a new empty report
    pub fn new() -> Self {
        let now = chrono::Utc::now();

        Self {
            timestamp: now.to_rfc3339(),
            healthy: true,
            bips: HashMap::new(),
            proposals: HashMap::new(),
            total_supported: 0,
            beta_count: 0,
            compliant_count: 0,
            partial_count: 0,
            missing_count: 0,
            proposal_count: 0,
        }
    }

    /// Add a BIP to the report
    pub fn add_bip(&mut self, detail: BipDetail) {
        // Update counts
        self.total_supported += 1;

        if detail.is_beta {
            self.beta_count += 1;
        }

        match detail.status {
            ComplianceStatus::Compliant => self.compliant_count += 1,
            ComplianceStatus::Partial => self.partial_count += 1,
            ComplianceStatus::Missing => self.missing_count += 1,
        }

        // Update health status if a required BIP is not compliant
        if detail.required && detail.status != ComplianceStatus::Compliant {
            self.healthy = false;
        }

        // Add to map
        self.bips.insert(detail.bip.clone(), detail);
    }

    /// Add a proposal to the report
    pub fn add_proposal(&mut self, detail: BipDetail) {
        self.proposal_count += 1;
        self.proposals.insert(detail.bip.clone(), detail);
    }

    /// Convert report to markdown format
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();

        md.push_str("# BIP System Health Report\n\n");
        md.push_str(&format!("Generated: {}\n\n", self.timestamp));

        // Overall summary
        md.push_str("## Summary\n\n");
        md.push_str(&format!(
            "- Overall Health: **{}**\n",
            if self.healthy {
                "Healthy ✅"
            } else {
                "Needs Attention ⚠️"
            }
        ));
        md.push_str(&format!(
            "- Total BIPs Supported: **{}**\n",
            self.total_supported
        ));
        md.push_str(&format!(
            "- Fully Compliant: **{}**\n",
            self.compliant_count
        ));
        md.push_str(&format!(
            "- Partially Compliant: **{}**\n",
            self.partial_count
        ));
        md.push_str(&format!("- Beta Features: **{}**\n", self.beta_count));
        md.push_str(&format!(
            "- Missing/Not Implemented: **{}**\n",
            self.missing_count
        ));
        md.push_str(&format!(
            "- Proposals Tracked: **{}**\n\n",
            self.proposal_count
        ));

        // BIP Table
        md.push_str("## BIP Details\n\n");
        md.push_str("| BIP | Name | Status | Implementation | Adoption | Beta | Required |\n");
        md.push_str("|-----|------|--------|----------------|----------|------|----------|\n");

        // Sort BIPs by number for consistent output
        let mut bips: Vec<&String> = self.bips.keys().collect();
        bips.sort();

        for bip_key in &bips {
            if let Some(bip) = self.bips.get(*bip_key) {
                let status_icon = match bip.status {
                    ComplianceStatus::Compliant => "✅",
                    ComplianceStatus::Partial => "⚠️",
                    ComplianceStatus::Missing => "❌",
                };

                md.push_str(&format!(
                    "| {} | {} | {} {} | {} | {} | {} | {} |\n",
                    bip.bip,
                    bip.name,
                    bip.status,
                    status_icon,
                    bip.implementation,
                    bip.industry_adoption,
                    if bip.is_beta { "Yes" } else { "No" },
                    if bip.required { "Yes" } else { "No" }
                ));
            }
        }

        // Proposals section (if any)
        if !self.proposals.is_empty() {
            md.push_str("\n## Proposals & Draft BIPs\n\n");
            md.push_str("| Name | Status | Implementation | Adoption | Beta |\n");
            md.push_str("|------|--------|----------------|----------|------|\n");

            // Sort proposals
            let mut proposals: Vec<&String> = self.proposals.keys().collect();
            proposals.sort();

            for prop_key in proposals {
                if let Some(prop) = self.proposals.get(prop_key) {
                    let status_icon = match prop.status {
                        ComplianceStatus::Compliant => "✅",
                        ComplianceStatus::Partial => "⚠️",
                        ComplianceStatus::Missing => "❌",
                    };

                    md.push_str(&format!(
                        "| {} | {} {} | {} | {} | {} |\n",
                        prop.name,
                        prop.status,
                        status_icon,
                        prop.implementation,
                        prop.industry_adoption,
                        if prop.is_beta { "Yes" } else { "No" }
                    ));
                }
            }
        }

        md.push_str("\n## Implementation Details\n\n");

        // BIP details
        for bip_key in &bips {
            if let Some(bip) = self.bips.get(*bip_key) {
                md.push_str(&format!("### {}: {}\n\n", bip.bip, bip.name));
                md.push_str(&format!("{}\n\n", bip.description));
                md.push_str(&format!("- **Status**: {}\n", bip.status));
                md.push_str(&format!("- **Implementation**: {}\n", bip.implementation));
                md.push_str(&format!(
                    "- **Industry Adoption**: {}\n",
                    bip.industry_adoption
                ));

                if let Some(coverage) = bip.test_coverage {
                    md.push_str(&format!("- **Test Coverage**: {}%\n", coverage));
                }

                if let Some(audit) = &bip.audit_status {
                    md.push_str(&format!("- **Audit Status**: {}\n", audit));
                }

                if let Some(refs) = &bip.references {
                    md.push_str("- **References**:\n");
                    for reference in refs {
                        md.push_str(&format!("  - {}\n", reference));
                    }
                }

                if let Some(updated) = &bip.last_updated {
                    md.push_str(&format!("- **Last Updated**: {}\n", updated));
                }

                md.push_str(&format!(
                    "- **Beta Feature**: {}\n",
                    if bip.is_beta { "Yes" } else { "No" }
                ));
                md.push_str(&format!(
                    "- **Required**: {}\n\n",
                    if bip.required { "Yes" } else { "No" }
                ));
            }
        }

        // Proposal details (if any)
        if !self.proposals.is_empty() {
            md.push_str("## Proposal Details\n\n");

            for prop_key in self.proposals.keys() {
                if let Some(prop) = self.proposals.get(prop_key) {
                    md.push_str(&format!("### {}\n\n", prop.name));
                    md.push_str(&format!("{}\n\n", prop.description));
                    md.push_str(&format!("- **Status**: {}\n", prop.status));
                    md.push_str(&format!("- **Implementation**: {}\n", prop.implementation));
                    md.push_str(&format!(
                        "- **Industry Adoption**: {}\n",
                        prop.industry_adoption
                    ));

                    if let Some(coverage) = prop.test_coverage {
                        md.push_str(&format!("- **Test Coverage**: {}%\n", coverage));
                    }

                    if let Some(audit) = &prop.audit_status {
                        md.push_str(&format!("- **Audit Status**: {}\n", audit));
                    }

                    if let Some(refs) = &prop.references {
                        md.push_str("- **References**:\n");
                        for reference in refs {
                            md.push_str(&format!("  - {}\n", reference));
                        }
                    }

                    if let Some(updated) = &prop.last_updated {
                        md.push_str(&format!("- **Last Updated**: {}\n", updated));
                    }

                    md.push_str(&format!(
                        "- **Beta Feature**: {}\n\n",
                        if prop.is_beta { "Yes" } else { "No" }
                    ));
                }
            }
        }

        md
    }

    /// Convert report to JSON format
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

/// BIP Health Checker System
pub struct BipHealthChecker {
    /// Bitcoin configuration
    bitcoin_config: BitcoinConfig,
    /// BIP353 implementation
    bip353: Option<Arc<Mutex<Bip353>>>,
    /// Known BIPs
    known_bips: HashMap<String, BipDetail>,
    /// Known proposals
    known_proposals: HashMap<String, BipDetail>,
    /// Last check time
    last_check: Instant,
    /// Check interval
    check_interval: Duration,
    /// Last report
    last_report: Option<BipHealthReport>,
}

impl BipHealthChecker {
    /// Create a new BIP health checker
    pub fn new(bitcoin_config: BitcoinConfig, bip353: Option<Arc<Mutex<Bip353>>>) -> Self {
        let mut checker = Self {
            bitcoin_config,
            bip353,
            known_bips: HashMap::new(),
            known_proposals: HashMap::new(),
            last_check: Instant::now() - Duration::from_secs(3600), // Force immediate check
            check_interval: Duration::from_secs(3600),              // Default: check every hour
            last_report: None,
        };

        // Initialize known BIPs
        checker.init_known_bips();

        // Initialize known proposals
        checker.init_known_proposals();

        checker
    }

    /// Initialize the list of known BIPs
    fn init_known_bips(&mut self) {
        // Schnorr Signatures
        self.known_bips.insert(
            "BIP-340".to_string(),
            BipDetail {
                bip: "BIP-340".to_string(),
                name: "Schnorr Signatures".to_string(),
                description: "Schnorr Signatures for secp256k1".to_string(),
                status: ComplianceStatus::Missing, // Will be checked later
                implementation: "Uses rust-secp256k1 library".to_string(),
                is_beta: false,
                test_coverage: Some(90),
                audit_status: Some("Verified".to_string()),
                required: true,
                industry_adoption: AdoptionStatus::Widespread,
                references: Some(vec![
                    "https://github.com/bitcoin/bips/blob/master/bip-0340.mediawiki".to_string(),
                    "https://bitcoinops.org/en/topics/schnorr-signatures/".to_string(),
                ]),
                last_updated: Some("2023-05-01".to_string()),
            },
        );

        // Taproot
        self.known_bips.insert(
            "BIP-341".to_string(),
            BipDetail {
                bip: "BIP-341".to_string(),
                name: "Taproot".to_string(),
                description: "Taproot: SegWit version 1 spending rules".to_string(),
                status: ComplianceStatus::Missing, // Will be checked later
                implementation: "Native implementation in core/script".to_string(),
                is_beta: false,
                test_coverage: Some(95),
                audit_status: Some("Verified".to_string()),
                required: true,
                industry_adoption: AdoptionStatus::Widespread,
                references: Some(vec![
                    "https://github.com/bitcoin/bips/blob/master/bip-0341.mediawiki".to_string(),
                    "https://bitcoinops.org/en/topics/taproot/".to_string(),
                ]),
                last_updated: Some("2023-05-01".to_string()),
            },
        );

        // Tapscript
        self.known_bips.insert(
            "BIP-342".to_string(),
            BipDetail {
                bip: "BIP-342".to_string(),
                name: "Tapscript".to_string(),
                description: "Validation of Taproot Scripts".to_string(),
                status: ComplianceStatus::Missing, // Will be checked later
                implementation: "Native implementation in core/script".to_string(),
                is_beta: false,
                test_coverage: Some(90),
                audit_status: Some("Verified".to_string()),
                required: true,
                industry_adoption: AdoptionStatus::Widespread,
                references: Some(vec![
                    "https://github.com/bitcoin/bips/blob/master/bip-0342.mediawiki".to_string(),
                    "https://bitcoinops.org/en/topics/tapscript/".to_string(),
                ]),
                last_updated: Some("2023-05-01".to_string()),
            },
        );

        // PSBT
        self.known_bips.insert(
            "BIP-174".to_string(),
            BipDetail {
                bip: "BIP-174".to_string(),
                name: "PSBT".to_string(),
                description: "Partially Signed Bitcoin Transactions".to_string(),
                status: ComplianceStatus::Missing, // Will be checked later
                implementation: "Full implementation in core/transaction".to_string(),
                is_beta: false,
                test_coverage: Some(98),
                audit_status: Some("Verified".to_string()),
                required: true,
                industry_adoption: AdoptionStatus::Widespread,
                references: Some(vec![
                    "https://github.com/bitcoin/bips/blob/master/bip-0174.mediawiki".to_string(),
                    "https://bitcoinops.org/en/topics/psbt/".to_string(),
                ]),
                last_updated: Some("2023-05-01".to_string()),
            },
        );

        // PSBT Version 2
        self.known_bips.insert(
            "BIP-370".to_string(),
            BipDetail {
                bip: "BIP-370".to_string(),
                name: "PSBT Version 2".to_string(),
                description: "PSBT Version 2 with Tap enhancements".to_string(),
                status: ComplianceStatus::Missing, // Will be checked later
                implementation: "Full implementation in core/transaction".to_string(),
                is_beta: false,
                test_coverage: Some(85),
                audit_status: Some("Verified".to_string()),
                required: false,
                industry_adoption: AdoptionStatus::Limited,
                references: Some(vec![
                    "https://github.com/bitcoin/bips/blob/master/bip-0370.mediawiki".to_string(),
                    "https://bitcoinops.org/en/topics/psbt/".to_string(),
                ]),
                last_updated: Some("2023-05-01".to_string()),
            },
        );

        // BIP353 DNS Payment Instructions (new!)
        self.known_bips.insert(
            "BIP-353".to_string(),
            BipDetail {
                bip: "BIP-353".to_string(),
                name: "DNS Payment Instructions".to_string(),
                description:
                    "DNS-based Bitcoin Payment Instructions using bitcoin@domain.tld identifiers"
                        .to_string(),
                status: ComplianceStatus::Missing, // Will be checked later
                implementation: "Full implementation in bip/bip353.rs".to_string(),
                is_beta: true,
                test_coverage: Some(80),
                audit_status: Some("In Progress".to_string()),
                required: false,
                industry_adoption: AdoptionStatus::Experimental,
                references: Some(vec![
                    "https://github.com/bitcoin/bips/blob/master/bip-0353.mediawiki".to_string(),
                ]),
                last_updated: Some("2025-04-15".to_string()),
            },
        );

        // BIP-329: Wallet Labels
        self.known_bips.insert(
            "BIP-329".to_string(),
            BipDetail {
                bip: "BIP-329".to_string(),
                name: "Wallet Labels".to_string(),
                description: "Wallet label export/import format".to_string(),
                status: ComplianceStatus::Missing, // Will be checked later
                implementation: "Basic implementation in wallet/labels.rs".to_string(),
                is_beta: true,
                test_coverage: Some(65),
                audit_status: Some("Pending".to_string()),
                required: false,
                industry_adoption: AdoptionStatus::Limited,
                references: Some(vec![
                    "https://github.com/bitcoin/bips/blob/master/bip-0329.mediawiki".to_string(),
                    "https://bitcoinops.org/en/topics/wallet-labels/".to_string(),
                    "https://bitcoinops.org/en/topic-dates/#april-2025"
                        .to_string()
                        .to_string(),
                ]),
                last_updated: Some("2025-02-24".to_string()),
            },
        );

        // BIP-322: Generic Signed Message Format
        self.known_bips.insert(
            "BIP-322".to_string(),
            BipDetail {
                bip: "BIP-322".to_string(),
                name: "Generic Signed Message Format".to_string(),
                description: "Generic signed message format for Bitcoin".to_string(),
                status: ComplianceStatus::Missing, // Will be checked later
                implementation: "Partial implementation in messaging/".to_string(),
                is_beta: true,
                test_coverage: Some(50),
                audit_status: Some("Not Started".to_string()),
                required: false,
                industry_adoption: AdoptionStatus::Limited,
                references: Some(vec![
                    "https://github.com/bitcoin/bips/blob/master/bip-0322.mediawiki".to_string(),
                    "https://bitcoinops.org/en/topics/generic-signmessage/".to_string(),
                    "https://bitcoinops.org/en/topic-dates/#april-2025".to_string(),
                ]),
                last_updated: Some("2025-04-01".to_string()),
            },
        );

        // BIP-119: CHECKTEMPLATEVERIFY
        self.known_bips.insert(
            "BIP-119".to_string(),
            BipDetail {
                bip: "BIP-119".to_string(),
                name: "CHECKTEMPLATEVERIFY".to_string(),
                description: "New opcode for transaction output commitments".to_string(),
                status: ComplianceStatus::Missing, // Will be checked later
                implementation: "Experimental implementation in core/script/opcodes".to_string(),
                is_beta: true,
                test_coverage: Some(30),
                audit_status: Some("Not Started".to_string()),
                required: false,
                industry_adoption: AdoptionStatus::Experimental,
                references: Some(vec![
                    "https://github.com/bitcoin/bips/blob/master/bip-0119.mediawiki".to_string(),
                    "https://bitcoinops.org/en/topics/op_checktemplateverify/".to_string(),
                    "https://bitcoinops.org/en/topic-dates/#april-2025".to_string(),
                ]),
                last_updated: Some("2025-04-10".to_string()),
            },
        );

        // BIP-118: SIGHASH_ANYPREVOUT
        self.known_bips.insert(
            "BIP-118".to_string(),
            BipDetail {
                bip: "BIP-118".to_string(),
                name: "SIGHASH_ANYPREVOUT".to_string(),
                description: "New sighash flag for non-input-specific transaction signatures"
                    .to_string(),
                status: ComplianceStatus::Missing, // Will be checked later
                implementation: "Experimental implementation in core/script/sighash.rs".to_string(),
                is_beta: true,
                test_coverage: Some(40),
                audit_status: Some("Not Started".to_string()),
                required: false,
                industry_adoption: AdoptionStatus::Experimental,
                references: Some(vec![
                    "https://github.com/bitcoin/bips/blob/master/bip-0118.mediawiki".to_string(),
                    "https://bitcoinops.org/en/topics/sighash_anyprevout/".to_string(),
                ]),
                last_updated: Some("2024-10-15".to_string()),
            },
        );

        // BIP-374: DLEQ Proofs
        self.known_bips.insert(
            "BIP-374".to_string(),
            BipDetail {
                bip: "BIP-374".to_string(),
                name: "Discrete Log Equivalence Proofs".to_string(),
                description: "Protocol for proving equivalence of discrete logarithms".to_string(),
                status: ComplianceStatus::Missing, // Will be checked later
                implementation: "Experimental implementation in security/crypto/dleq.rs"
                    .to_string(),
                is_beta: true,
                test_coverage: Some(20),
                audit_status: Some("Not Started".to_string()),
                required: false,
                industry_adoption: AdoptionStatus::Experimental,
                references: Some(vec![
                    "https://github.com/bitcoin/bips/blob/master/bip-0374.mediawiki".to_string(),
                    "https://bitcoinops.org/en/topic-dates/#april-2025".to_string(),
                ]),
                last_updated: Some("2025-03-14".to_string()),
            },
        );
    }

    /// Initialize the list of known proposals (not yet formal BIPs)
    fn init_known_proposals(&mut self) {
        // MuSig2 (not a formal BIP yet but widely used)
        self.known_proposals.insert(
            "musig2".to_string(),
            BipDetail {
                bip: "musig2".to_string(),
                name: "MuSig2".to_string(),
                description: "Multisignature scheme using Schnorr signatures".to_string(),
                status: ComplianceStatus::Partial,
                implementation: "Experimental implementation in security/crypto/musig.rs"
                    .to_string(),
                is_beta: true,
                test_coverage: Some(70),
                audit_status: Some("In Progress".to_string()),
                required: false,
                industry_adoption: AdoptionStatus::Limited,
                references: Some(vec![
                    "https://bitcoinops.org/en/topics/musig/".to_string(),
                    "https://bitcoinops.org/en/topic-dates/#april-2025".to_string(),
                ]),
                last_updated: Some("2025-02-15".to_string()),
            },
        );

        // Erlay (Protocol enhancement, not a formal BIP)
        self.known_proposals.insert(
            "erlay".to_string(),
            BipDetail {
                bip: "erlay".to_string(),
                name: "Erlay".to_string(),
                description: "Bandwidth-efficient transaction relay protocol".to_string(),
                status: ComplianceStatus::Missing,
                implementation: "Not implemented".to_string(),
                is_beta: true,
                test_coverage: None,
                audit_status: None,
                required: false,
                industry_adoption: AdoptionStatus::Experimental,
                references: Some(vec![
                    "https://bitcoinops.org/en/topics/erlay/".to_string(),
                    "https://bitcoinops.org/en/topic-dates/#april-2025".to_string(),
                ]),
                last_updated: Some("2025-03-15".to_string()),
            },
        );

        // Ephemeral Anchors
        self.known_proposals.insert(
            "ephemeral_anchors".to_string(),
            BipDetail {
                bip: "ephemeral_anchors".to_string(),
                name: "Ephemeral Anchors".to_string(),
                description: "Zero-value outputs that can be spent by anyone".to_string(),
                status: ComplianceStatus::Missing,
                implementation: "Experimental in core/transaction/anchors.rs".to_string(),
                is_beta: true,
                test_coverage: Some(30),
                audit_status: Some("Not Started".to_string()),
                required: false,
                industry_adoption: AdoptionStatus::Experimental,
                references: Some(vec![
                    "https://bitcoinops.org/en/topics/ephemeral-anchors/".to_string(),
                    "https://bitcoinops.org/en/topic-dates/#april-2025".to_string(),
                ]),
                last_updated: Some("2025-03-15".to_string()),
            },
        );

        // FROST (Threshold signature scheme)
        self.known_proposals.insert(
            "frost".to_string(),
            BipDetail {
                bip: "frost".to_string(),
                name: "FROST".to_string(),
                description: "Flexible Round-Optimized Schnorr Threshold signatures".to_string(),
                status: ComplianceStatus::Missing,
                implementation: "Not implemented".to_string(),
                is_beta: true,
                test_coverage: None,
                audit_status: None,
                required: false,
                industry_adoption: AdoptionStatus::Experimental,
                references: Some(vec![
                    "https://bitcoinops.org/en/topic-dates/#april-2025".to_string()
                ]),
                last_updated: Some("2025-03-15".to_string()),
            },
        );

        // Channel Jamming Mitigation
        self.known_proposals.insert(
            "channel_jamming".to_string(),
            BipDetail {
                bip: "channel_jamming".to_string(),
                name: "Channel Jamming Mitigation".to_string(),
                description: "Protocol for upfront and hold fees to address channel jamming"
                    .to_string(),
                status: ComplianceStatus::Missing,
                implementation: "Not implemented".to_string(),
                is_beta: true,
                test_coverage: None,
                audit_status: None,
                required: false,
                industry_adoption: AdoptionStatus::Experimental,
                references: Some(vec![
                    "https://bitcoinops.org/en/topic-dates/#april-2025".to_string()
                ]),
                last_updated: Some("2025-03-15".to_string()),
            },
        );

        // MATT (Merkleized Alternative to Tapscript Trees)
        self.known_proposals.insert(
            "matt".to_string(),
            BipDetail {
                bip: "matt".to_string(),
                name: "MATT".to_string(),
                description: "Merkleized Alternative to Tapscript Trees".to_string(),
                status: ComplianceStatus::Missing,
                implementation: "Not implemented".to_string(),
                is_beta: true,
                test_coverage: None,
                audit_status: None,
                required: false,
                industry_adoption: AdoptionStatus::Draft,
                references: Some(vec![
                    "https://bitcoinops.org/en/topics/matt/".to_string(),
                    "https://bitcoinops.org/en/topic-dates/#april-2025".to_string(),
                ]),
                last_updated: Some("2025-04-15".to_string()),
            },
        );
    }

    /// Set the check interval
    pub fn set_check_interval(&mut self, interval_secs: u64) {
        self.check_interval = Duration::from_secs(interval_secs);
    }

    /// Check if we need to regenerate the health report
    pub fn needs_check(&self) -> bool {
        self.last_check.elapsed() >= self.check_interval || self.last_report.is_none()
    }

    /// Check BIP health and generate a report
    pub fn check_health(&mut self) -> Result<BipHealthReport, Box<dyn Error>> {
        // Check if we need to regenerate
        if !self.needs_check() {
            if let Some(report) = &self.last_report {
                return Ok(report.clone());
            }
        }

        debug!("Generating new BIP health report");
        let mut report = BipHealthReport::new();

        // Validate each known BIP
        for (bip_key, mut bip_detail) in self.known_bips.clone() {
            let status = self.bitcoin_config.validate_bip(&bip_key)?;
            bip_detail.status = status;

            // For BIP-353, check if beta
            if bip_key == "BIP-353" {
                if let Some(bip353) = &self.bip353 {
                    let bip353 = bip353.lock().unwrap();
                    bip_detail.is_beta = bip353.status() == Bip353Status::Beta;
                }
            }

            report.add_bip(bip_detail);
        }

        // Add proposals
        for (_, proposal) in self.known_proposals.clone() {
            report.add_proposal(proposal);
        }

        // Update last check time and report
        self.last_check = Instant::now();
        self.last_report = Some(report.clone());

        Ok(report)
    }

    /// Generate a health report and save to file
    pub fn generate_report_file(&mut self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let report = self.check_health()?;

        // Generate report in markdown format
        let md_content = report.to_markdown();
        std::fs::write(file_path, md_content)?;

        // Also save JSON version
        let json_path = format!("{}.json", file_path.replace(".md", ""));
        let json_content = report.to_json()?;
        std::fs::write(json_path, json_content)?;

        info!("BIP health report saved to {}", file_path);
        Ok(())
    }

    /// Get BIP health as JSON
    pub fn health_json(&mut self) -> Result<String, Box<dyn Error>> {
        let report = self.check_health()?;
        Ok(report.to_json()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report_generation() {
        let mut config = BitcoinConfig::default();
        config.taproot_enabled = true;
        config.tapscript_enabled = true;
        config.psbt_version = 2;
        config.bip353_enabled = true;
        config.bip353_status = Bip353Status::Beta;

        let mut checker = BipHealthChecker::new(config, None);
        let report = checker.check_health().unwrap();

        assert!(report.bips.contains_key("BIP-341"));
        assert!(report.bips.contains_key("BIP-353"));

        // BIP-341 should be compliant
        if let Some(bip341) = report.bips.get("BIP-341") {
            assert_eq!(bip341.status, ComplianceStatus::Compliant);
        }

        // BIP-353 should be partially compliant (beta)
        if let Some(bip353) = report.bips.get("BIP-353") {
            assert_eq!(bip353.status, ComplianceStatus::Partial);
            assert!(bip353.is_beta);
        }

        // Should include at least one proposal
        assert!(!report.proposals.is_empty());
    }
}
