# Implementation Roadmap PRD

Date: August 8, 2025
Version: 2.0.0

## Week 1-2: Minimal Productionization
- Wire Bitcoin Core RPC into state/fees (height/hash/feerates)
- Add feature flags for LN/RGB/DLC production adapters
- Provide env-driven config and defaults in example layer2.toml

## Week 3-4: LN + RGB MVP
- Implement minimal LN adapter operations (connect, invoice, pay)
- RGB: list assets, basic issue/transfer mock-through real client
- Expand health to include adapter reachability

## Weeks 5-8: DLC + Hardening
- DLC oracle client minimal
- Persistence for adapter state; retries/backoff
- CI: run verifications and warn when peers < min threshold without primary self-node

Success Criteria
- cargo check --all-features PASS
- Layer2 tests PASS + new adapter smoke tests PASS
- Warnings < 10
- Docs in docs/prd kept current

Last Updated: August 8, 2025
