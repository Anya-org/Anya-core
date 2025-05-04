#!/bin/bash
set -e

echo "=== Verifying Anya DAO Clarity Contracts ==="
echo "============================================"

# Verify syntax of all contracts
echo "Checking syntax for all contracts..."
clarinet check

# Run static analysis
echo -e "\nRunning static analysis..."
clarinet analyze

# Check for common issues and best practices
echo -e "\nChecking for known issues..."
clarinet lint

echo -e "\n✅ All contracts verified successfully!" 