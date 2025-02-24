# Anya Threat Model Matrix v0.2

## Enhanced Component Coverage

| Component               | Threat Vector          | Mitigation Strategy                          | Detection Method                  | Severity | Status  |
|-------------------------|------------------------|-----------------------------------------------|------------------------------------|----------|---------|
| **DLC Engine**          | Oracle Collusion       | 3-of-5 Schnorr Multi-Signatures              | Threshold Signature Analysis      | Critical | Active  |
|                         | Dispute Timeout Exploit| Hourly Dispute Status Checks                 | Timeout Escrow Monitoring         | High     | Pending |
| **DAO Governance**       | Voting Sybil Attack    | Proof-of-Stake Weighted Voting               | Address Clustering Analysis       | High     | Live    |
|                         | Proposal Griefing      | Minimum Lock Period (72h)                    | Proposal Age Tracking             | Medium   | Active  |
| **sBTC Bridge**          | Peg Proof Spoofing     | SPV Proofs with Bitcoin Header Chain         | Proof Depth Validation             | Critical | Active  |
|                         | Mint/Burn Race Attack  | Sequential Nonce Locking                     | Transaction Nonce Monitoring      | High     | Beta    |
| **Lightning Gateway**    | Channel Jamming        | Dynamic Fee Adjustment + HTLC Limits         | Channel Balance Skew Detection    | High     | Planned |
|                         | Forwarding Attack      | Strict HTLC Expiry Policy                    | HTLC Expiry Variance Analysis      | Medium   | Active  |
| **Key Management**       | HSM Side-Channel       | FIDO2 Attestation + Tempest Shielding        | Power Analysis Monitoring         | Critical | Active  |
|                         | Key Rotation Risk      | Automated 90-Day Rotation Schedule           | Key Age Auditing                   | High     | Live    |

## Mitigation Implementation Plan

gantt
    title Threat Mitigation Roadmap
    dateFormat  YYYY-MM-DD
    section Critical
    DLC Multi-Oracle :done, a1, 2024-06-01, 2024-07-15
    sBTC SPV Proofs :active, a2, 2024-07-01, 45d
    HSM FIDO2 Integration :crit, a3, 2024-08-01, 30d
    
    section High
    DAO Sybil Protection :done, a4, 2024-05-01, 20d
    Key Rotation Scheduler :active, a5, 2024-07-15, 60d
    Lightning Jam Defense : a6, 2024-09-01, 30d

## Component Threat Mapping

| Component               | Threat Vector          | Mitigation Strategy                          | Detection Method                  | Severity |
|-------------------------|------------------------|-----------------------------------------------|------------------------------------|----------|
| **DLC Engine**          | Oracle Collusion       | Multi-sig (3-of-5) + Dispute Timeout         | Signature Anomaly Detection       | Critical |
|                         | Signature Forgery      | HSM-based Signing (FIDO2)                    | HSM Audit Logs                    | High     |
| **DAO Governance**       | 51% Attack             | Quadratic Voting + Veto Threshold            | Vote Distribution Analysis        | High     |
|                         | Proposal Spam          | Staked Proposal Deposits                     | Gas Price Monitoring              | Medium   |
| **sBTC Bridge**          | Reserve Drain          | 90% Collateral Floor + Circuit Breaker       | Realtime Reserve Monitoring       | Critical |
|                         | Invalid Peg Proof       | 6-Confirmation SPV Proofs                    | Block Header Validation            | High     |
| **Web5 PSBT**           | Schema Injection       | JSON Schema v7 Validation                    | DWN Record Sanitization            | Medium   |
|                         | DID Takeover           | 90-Day Key Rotation                          | DID Revocation Registry           | High     |
| **ML Federated**         | Model Poisoning        | Gradient Signing + TEE Verification          | Weight Distribution Analysis       | Critical |
|                         | Data Leakage           | Homomorphic Encryption                        | Model Output Auditing              | High     |
| **Bitcoin Protocol**     | Double Spend           | 6-Confirm Rule + Mempool Analysis            | Chain Reorg Monitoring            | Critical |
|                         | Fee Sniping            | RBF Signaling + Child Pays For Parent         | Transaction Ancestry Tracking      | Medium   |
| **Metrics Pipeline**     | Data Tampering         | Merkleized Metrics + ZK Proofs               | Hash Chain Validation              | High     |
|                         | Privacy Leakage        | k-Anonymity Aggregation                       | Differential Privacy Audits       | Medium   |

## Mitigation Implementation Status
