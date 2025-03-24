#!/bin/bash
# Validate full BDF v2.5 compliance
cargo audit --target aarch64-apple-ios --ignore RUSTSEC-2024-0321
anya-audit check --rules all --level strict
cargo test --features "bip341 bip342 bip370 ais3" -- --test-threads=1

# Cryptographic validation
anya-validator --crypto-mode full --report-format json > crypto-report.json

# Generate compliance matrix
anya-docs generate-compliance-matrix --output BDF-STATUS.md 