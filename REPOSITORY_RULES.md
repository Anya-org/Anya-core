# Anya Core Repository Rules 

## Overview

This document outlines the comprehensive rules and standards for the Anya Core repository, ensuring alignment with the Bitcoin Development Framework v2.5 and proper integration with the AI labeling system. These rules govern all code contributions, documentation updates, and repository management practices.

## Core Principles

### 1. Bitcoin Development Framework Compliance

All code and documentation must maintain strict adherence to the Bitcoin Development Framework v2.5 core principles:

- **Decentralization**: Code must preserve Bitcoin's core tenets of decentralization, immutability, and censorship resistance
- **Privacy-Preservation**: DLC implementations must use non-interactive oracle patterns to maintain transaction indistinguishability
- **Asset Management**: Taproot-enabled protocols must be used for asset issuance and management

### 2. Hexagonal Architecture Enforcement

All components must follow hexagonal architecture principles:

- **Core Logic**: Domain-specific business logic must be implementation-agnostic
- **Adapters**: Interface implementations must be separated from core logic
- **Ports**: Well-defined interfaces must be used for all external communication

### 3. AI Labeling Requirements

All code must include appropriate AI labels as defined in [AI_LABELLING.md](AI_LABELLING.md):

```rust
/// [AIR-3][AIS-3][AIT-2][BPC-3] Taproot implementation
/// 
/// Implements BIP-341/342 (Taproot) functionality following
/// the Bitcoin Development Framework v2.5 requirements.
pub struct TaprootImplementation {
    // Implementation details
}
```

## Code Structure and Organization

### 1. Module Organization

Code must be organized according to the following structure:

- **Bitcoin Core**: `/src/bitcoin/` - All Bitcoin protocol implementations
- **Layer 2**: `/src/layer2/` - Lightning, RGB, and other L2 implementations
- **DLC**: `/src/bitcoin/dlc/` - Discrete Log Contract implementations
- **Web5**: `/src/web5/` - Web5 and DID/DWN implementations
- **Security**: `/src/security/` - Security-related functionality
- **Core**: `/src/core/` - Core system components

### 2. Implementation Requirements by Component Type

#### Bitcoin Components

- Must implement relevant BIPs (341/342 for Taproot, 174 for PSBT)
- Must use zero-knowledge techniques when appropriate
- Must include comprehensive testing for consensus-critical code
- Must be labeled with at least AIR-2, AIS-3, AIT-3, BPC-3

#### Layer 2 Components

- Must maintain Layer 1 security guarantees
- Must include fallback mechanisms to Layer 1
- Must provide proper validation for all cross-layer operations
- Must be labeled with at least AIR-2, AIS-3, AIT-2, BPC-2

#### DLC Components

- Must implement non-interactive oracle patterns
- Must ensure transaction indistinguishability
- Must use Schnorr-based signatures for oracles
- Must implement 2-of-2 MuSig for execution
- Must be labeled with at least AIR-3, AIS-3, AIP-3, AIT-3, BPC-3

#### Web5 Components

- Must properly implement DID specifications
- Must handle DWN protocol correctly
- Must ensure privacy and security for all user data
- Must be labeled with at least AIR-2, AIS-2, AIT-2, W5C-2, DID-2

## Coding Standards

### 1. Documentation Requirements

All code must include:

- Function-level documentation describing purpose, parameters, and return values
- Module-level documentation describing overall functionality
- Appropriate AI labels for all components
- Reference to relevant BIPs, specifications, or protocols
- Security and privacy considerations

Example:

```rust
/// [AIR-3][AIS-3][BPC-3] Verify Bitcoin payment using SPV proof
/// 
/// Implements BIP-37 compliant SPV verification to validate Bitcoin payments
/// without requiring a full node, preserving the decentralization principle.
///
/// # Parameters
/// * `tx_hash` - The transaction hash to verify
/// * `block_header` - The block header containing the transaction
/// * `merkle_proof` - The merkle proof for the transaction
///
/// # Returns
/// * `true` if the transaction is valid, `false` otherwise
///
/// # Security Considerations
/// - Validates merkle path completeness
/// - Ensures hash preimage protection
/// - Prevents second-preimage attacks
pub fn verify_bitcoin_payment(tx_hash: &[u8], block_header: &BlockHeader, merkle_proof: &[u8]) -> bool {
    // Implementation
}
```

### 2. Testing Requirements

All code must include:

- Unit tests with at least 90% coverage for non-consensus code
- 100% test coverage for consensus-critical code
- Integration tests simulating real-world usage
- Fuzz testing for security-critical components
- Performance benchmarks for performance-sensitive components

### 3. Error Handling

All code must:

- Use appropriate error types and propagation
- Provide meaningful error messages
- Handle all edge cases appropriately
- Avoid panics in production code
- Log errors at appropriate levels

### 4. Security Requirements

All code must:

- Undergo security review before merging
- Include explicit security considerations in documentation
- Follow constant-time implementations for cryptographic code
- Protect against common vulnerabilities (buffer overflows, timing attacks, etc.)
- Implement proper input validation

## Commit and Pull Request Standards

### 1. Commit Message Format

All commit messages must follow the format specified in [COMMIT_RULES.md](COMMIT_RULES.md):

```
<type>(<scope>): <description>

<body>

Labels: [AIR-3][AIS-2][AIT-2][BPC-3][PFM-2]
```

### 2. Pull Request Requirements

All pull requests must:

- Reference related issues
- Include comprehensive descriptions
- Pass all CI checks
- Include appropriate labels
- Be reviewed by at least one maintainer
- Update relevant documentation

### 3. Review Process

Code reviews must verify:

- Functional correctness
- Code quality and maintainability
- Security considerations
- Performance implications
- Test coverage
- Documentation completeness
- Label accuracy

## Integration with Bitcoin Development Framework

### 1. BIP Implementation

When implementing Bitcoin Improvement Proposals:

- Comprehensively document the BIP being implemented
- Include references to the BIP specification
- Implement all required functionality
- Include test vectors from the BIP specification
- Update the BIP support matrix

### 2. Protocol Validation

All implementations must include:

- Validation against the protocol specification
- Comprehensive error checking
- Proper handling of edge cases
- Performance considerations
- Security implications

### 3. Security Validation

All transactions must pass comprehensive checks:

```rust
/// [AIS-3][BPC-3] Validate transaction structure and security
/// 
/// Ensures transactions comply with all Bitcoin protocol rules and
/// additional security requirements from the Bitcoin Development Framework.
pub fn validate_transaction(tx: &Transaction) -> Result<(), Error> {
    // Validate basic structure
    if tx.inputs.is_empty() || tx.outputs.is_empty() {
        return Err(Error::InvalidStructure("Transaction must have inputs and outputs"));
    }

    // Validate witness data for SegWit compliance
    if !tx.has_witness() {
        return Err(Error::SegWitRequired("Transaction must use SegWit"));
    }

    // Validate Taproot conditions for BIP-341 compliance
    if !check_taproot_conditions(tx) {
        return Err(Error::TaprootComplianceFailure("BIP-341 compliance check failed"));
    }

    // Additional security checks
    // ...

    Ok(())
}
```

## Continuous Integration and Deployment

### 1. CI Pipeline Requirements

The CI pipeline must include:

- Automated testing for all components
- Code quality checks (linting, formatting)
- Security scanning
- Performance benchmarking
- Documentation generation
- AI labeling validation

### 2. Release Process

Releases must follow:

- Semantic versioning
- Comprehensive release notes
- Security review sign-off
- Performance validation
- Documentation updates

## Documentation Standards

### 1. Technical Documentation

All technical documentation must:

- Be kept in sync with code changes
- Include diagrams where appropriate
- Provide examples of usage
- Document security considerations
- Be reviewed for accuracy

### 2. User Documentation

All user documentation must:

- Be clear and concise
- Include step-by-step instructions
- Provide examples and screenshots
- Be reviewed for usability
- Be kept up-to-date with feature changes

## Compliance with AI Labeling System

### 1. Development Stage (60%) Requirements

Components at the development stage must have:

- **Core Categories**: Minimum AIR-1, AIS-1, AIT-1
- **Extended Categories**: Minimum BPC-1 (for Bitcoin components), W5C-1 (for Web5 components)

### 2. Production Stage (90%) Requirements

Components at the production stage must have:

- **Core Categories**: Minimum AIR-2, AIS-2, AIT-2, AIM-1, AIP-1
- **Extended Categories**: Minimum BPC-2, PFM-1, RES-1, SCL-1

### 3. Release Stage (99%) Requirements

Components at the release stage must have:

- **Core Categories**: Minimum AIR-3, AIS-3, AIT-3, AIM-2, AIP-2, AIE-2
- **Extended Categories**: Minimum BPC-3, PFM-2, RES-2, SCL-2, UXA-2

## Special Component Requirements

### 1. DLC Oracle Components

DLC Oracle components must:

- Implement non-interactive patterns
- Follow Transaction Flow: Commitment → Oracle Signature → Execution
- Use Taproot addresses for commitments
- Use Schnorr-based signatures for oracles
- Implement 2-of-2 MuSig for execution
- Achieve minimum AIS-3, AIP-3, AIT-3, BPC-3 ratings

### 2. Taproot Asset Components

Taproot Asset components must:

- Implement BIP-341/342 correctly
- Support React Native mobile integration
- Maintain proper asset metadata
- Ensure transaction privacy
- Achieve minimum AIS-3, BPC-3, UXA-2 ratings

### 3. Lightning Network Components

Lightning Network components must:

- Comply with BOLT specifications
- Implement proper channel management
- Ensure proper error handling and recovery
- Maintain security and privacy
- Achieve minimum AIS-3, PFM-2, RES-3, BPC-2 ratings

## Operational Reliability and AI Quality Assurance

### 1. Progress Monitoring Requirements

All long-running operations must:

- Provide clear progress indicators with percentage completion
- Include timeout mechanisms to prevent indefinite hanging
- Log intermediate progress at appropriate intervals
- Include cancellation capabilities for user-initiated operations
- Implement heartbeat mechanisms for distributed operations

Example implementation:

```rust
/// [AIR-2][AIS-2][RES-3] Execute blockchain synchronization with progress monitoring
/// 
/// Synchronizes the local blockchain with network nodes while providing
/// detailed progress feedback and preventing indefinite hanging.
pub fn sync_blockchain(timeout_seconds: u64) -> Result<SyncStatus, Error> {
    let start_time = Instant::now();
    let mut progress = 0.0;
    
    // Set up progress callback
    let progress_callback = |current: f64| {
        progress = current;
        log::info!("Sync progress: {:.2}%", progress * 100.0);
        
        // Check for timeout
        if start_time.elapsed().as_secs() > timeout_seconds {
            return Err(Error::Timeout("Blockchain sync timed out"));
        }
        
        Ok(())
    };
    
    // Execute with progress monitoring
    let result = blockchain_client.sync(progress_callback)?;
    
    // Final progress log
    log::info!("Sync completed: 100%");
    
    Ok(result)
}
```

### 2. AI Hallucination Prevention

All AI components must:

- Implement fact verification mechanisms against trusted data sources
- Include confidence scoring for all generated outputs
- Apply appropriate thresholds for confidence-based filtering
- Maintain comprehensive logging of verification steps
- Incorporate human-in-the-loop validation for critical operations

Implementation requirements:

```rust
/// [AIR-3][AIS-3][AIE-3] Generate blockchain transaction with hallucination prevention
/// 
/// Creates a transaction with AI assistance while ensuring all generated data
/// is verified against trusted sources to prevent hallucination.
pub fn generate_transaction(parameters: TransactionParameters) -> Result<Transaction, Error> {
    // Generate candidate transaction
    let (candidate_tx, confidence_score) = ai_engine.generate_transaction(parameters)?;
    
    // Apply confidence threshold
    if confidence_score < CONFIG.min_confidence_threshold {
        return Err(Error::LowConfidence("Generated transaction has insufficient confidence score"));
    }
    
    // Verify against blockchain state
    let verification_result = verify_against_blockchain(&candidate_tx)?;
    if !verification_result.is_valid {
        log::warn!("AI hallucination detected: {}", verification_result.reason);
        return Err(Error::VerificationFailed(verification_result.reason));
    }
    
    // Log verification for audit
    log::info!("Transaction verified: confidence={}, verification_steps={}", 
              confidence_score, verification_result.steps.len());
    
    Ok(candidate_tx)
}
```

### 3. Process Hang Prevention

All system processes must:

- Implement watchdog timers for detecting stuck operations
- Use structured concurrency patterns to manage child tasks
- Include cascading timeout mechanisms for dependent operations
- Provide automatic recovery mechanisms for detected hangs
- Log detailed diagnostics when operations exceed expected durations

Example:

```rust
/// [AIR-2][RES-3] Execute network operation with hang prevention
/// 
/// Performs a network operation with proper timeout handling and
/// recovery mechanisms to prevent indefinite hanging.
pub async fn execute_network_operation(params: NetworkParams) -> Result<NetworkResponse, Error> {
    // Create watchdog timer
    let watchdog = Watchdog::new("network_operation", Duration::from_secs(30));
    
    // Execute operation with timeout
    let operation_future = network_client.execute(params);
    match tokio::time::timeout(Duration::from_secs(15), operation_future).await {
        Ok(result) => {
            // Operation completed successfully
            watchdog.stop();
            Ok(result?)
        }
        Err(_) => {
            // Operation timed out
            log::warn!("Network operation timed out, attempting recovery");
            
            // Recovery attempt with shorter timeout
            match tokio::time::timeout(
                Duration::from_secs(5), 
                network_client.execute_recovery(params)
            ).await {
                Ok(recovery_result) => {
                    watchdog.stop();
                    log::info!("Recovery successful after timeout");
                    Ok(recovery_result?)
                }
                Err(_) => {
                    // Recovery also timed out
                    watchdog.trigger_alert();
                    log::error!("Recovery failed, operation hang detected");
                    Err(Error::OperationHang("Network operation and recovery both timed out"))
                }
            }
        }
    }
}
```

## Last Updated

2025-02-24 