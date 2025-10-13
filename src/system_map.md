# Anya Core System Map

## Root Structure [AIR-3][AIS-3][RES-3]

**Purpose:** The root of the Anya Core system, orchestrating all major modules and providing the primary configuration, error handling, and initialization logic.

### Main Modules

- `ml`: Machine learning and AI agent system
- `web5`: Web5 protocol and decentralized identity
- `dao`: Decentralized autonomous organization
- `bitcoin`: Bitcoin and Lightning Network integration
- `security`: System security, HSM, and validation
- `layer2`: Layer2 protocol support (e.g., DLC, RGB, Lightning)
- `extensions`: Optional system extensions
- `tokenomics`: Tokenomics engine
- `config`: System configuration
- `tools`: Utilities and developer tools
- `core`: Core logic and orchestrator
- `utils`: Common helpers

### Features

- `hsm`: Hardware Security Module support
- `complete`: Full system including HSM
- `bitcoin_integration`: Bitcoin protocol and Layer2
- `std`: Standard library support

### Error Handling

- Centralized via `AnyaError` and `AnyaResult`

### Initialization

- `AnyaCore` struct and `init()` function
- Async main for API server

### AI Labeling

- [AIR-3][AIS-3][AIT-3][AIP-3][RES-3] present in documentation

---

## Security Module [AIR-3][AIS-3][RES-3]

**Purpose:** Provides core security functionality, including system hardening, input validation, and Hardware Security Module (HSM) support for cryptographic operations. Implements BDF v2.5 security requirements and supports hexagonal architecture.

### Submodules

- `system_hardening`: System-level security configuration and enforcement
- `validation`: Input and process validation routines
- `constant_time`: Constant-time cryptographic operations
- `crypto`: Security-focused cryptographic primitives
- `hsm`: Hardware Security Module support (feature-flagged)
- `hsm_shim`: Fallback implementation when HSM is not enabled

### Key Functions

- `create_system_hardening()`
- `create_basic_security_config(component_name)`
- `initialize()` (async)
- `create_bitcoin_hsm_provider(base_provider)`
- `verify_bitcoin_payment(bitcoin_provider, proof_data)` (async)
- `create_taproot_asset(bitcoin_provider, metadata, supply)` (async)

### AI Labeling

- [AIR-3][AIS-3][AIT-3][AIP-3][RES-3] present in module

### Notes

- `quantum_resistant.rs` is currently not used elsewhere in the codebase. Marked for review, not removal.
- Placeholder code for SPV proof and Taproot asset creation must be replaced with full BDF v2.5-compliant implementations.

---

### Recent Maintenance (Cleanup)

- Removed transitional / duplicate files: `taproot/mod.rs.fix`, `api/routes_new.rs`, `compliance/mod_new.rs`, `security/crypto/random_new.rs`, `web/web5_adapter_new.rs`, `bitcoin/layer2/rgb/mod_new.rs`.
 	- Rationale: superseded by canonical implementations (`taproot/mod.rs`, existing compliance module, existing random.rs, existing web5_adapter.rs) or empty placeholders with no references (verified by repository search).
 	- Action: Updated system map; no functional code paths depended on removed files (grep confirmed zero references).
 	- Compliance: Aligns with Non‑Deviation & Change Control (removal of orphaned modules) in enterprise enforcement profile.

*This map will be updated as each module is reviewed.*

---
<!-- AI Labeling Reference Definitions -->
[AIS-3]: ../docs_legacy/standards/AI_LABELING.md
[AIP-3]: ../docs_legacy/standards/AI_LABELING.md
[RES-3]: ../docs_legacy/standards/AI_LABELING.md
