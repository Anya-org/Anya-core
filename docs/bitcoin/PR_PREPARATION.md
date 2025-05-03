# Pull Request Preparation Guide
[AIR-3][AIS-3][BPC-3][AIT-3][RES-3]

This document provides instructions for creating a Pull Request for the Bitcoin hexagonal architecture implementation.

## PR Creation Instructions

1. Go to the GitHub repository: https://github.com/Anya-org/Anya-core
2. Click on "Pull requests" tab
3. Click the "New pull request" button
4. Set the base branch to `feature/bitcoin-implementation`
5. Set the compare branch to `feature/bitcoin-hexagonal-architecture`
6. Click "Create pull request"

## PR Details

### Title
[AIR-3][AIS-3][BPC-3] Implement Hexagonal Architecture for Bitcoin Module

### Description
Use the template from `.github/PULL_REQUEST_TEMPLATE/bitcoin-hexagonal-architecture.md`

### Reviewers
Assign at least one Bitcoin protocol expert as a reviewer

### Labels
- bitcoin
- architecture
- enhancement
- bip-implementation

### Issue Links
Link to any related issues (e.g., the compilation issues)

## Before Submitting

- [x] All files committed to the branch
- [x] Documentation updated
- [ ] Compilation issues documented
- [ ] Tests added for new functionality
- [x] PR template followed

## After PR Creation

1. Monitor the PR for feedback
2. Address any review comments
3. Fix compilation issues documented in the todos
4. Update the PR with additional commits as needed
5. Ensure all CI checks pass

## Merge Strategy

Once approved, the PR will be merged into the `feature/bitcoin-implementation` branch using a merge commit strategy to preserve the commit history. 