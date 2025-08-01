# Bitcoin Module PR Checklist

[AIR-3][AIS-3][BPC-3][AIT-3][RES-3]

This document provides a comprehensive checklist for preparing, reviewing, and merging Pull Requests for Bitcoin-related changes in the Anya-Core project.

## PR Preparation Checklist

### Code Quality

- [ ] All compilation issues resolved
- [ ] Code follows the project's style guidelines
- [ ] No linting issues (run `cargo clippy`)
- [ ] Proper error handling implemented
- [ ] Security best practices followed
- [ ] Hexagonal architecture principles adhered to

### Testing

- [ ] Unit tests added for new functionality
- [ ] Integration tests added where appropriate
- [ ] Tests for edge cases included
- [ ] All tests pass successfully
- [ ] Security-related tests included

### Documentation

- [ ] Code is well-commented
- [ ] API documentation updated
- [ ] BIP implementation details documented
- [ ] Architecture decisions explained
- [ ] README files updated where necessary

### Bitcoin Compliance

- [ ] Follows BIP specifications
- [ ] Compatible with Bitcoin Core
- [ ] Maintains transaction indistinguishability
- [ ] Preserves decentralization, immutability, and censorship resistance
- [ ] Properly implements Taproot/Tapscript (if applicable)

### Branch Management

- [ ] Feature branch up-to-date with target branch
- [ ] No merge conflicts
- [ ] Commit messages follow [AIR-3][AIS-3][BPC-3] format
- [ ] Proper branch naming convention followed

## PR Review Checklist

### General

- [ ] Code is clear and easy to understand
- [ ] No unnecessary complexity
- [ ] No duplicated code
- [ ] No hardcoded secrets or credentials
- [ ] Performance considerations addressed

### Bitcoin-Specific

- [ ] Properly handles Bitcoin network interactions
- [ ] Transaction validation is secure
- [ ] Script execution follows BIP specifications
- [ ] Correctly implements consensus rules
- [ ] Handles blockchain reorganizations properly

### Security

- [ ] No timing attack vulnerabilities
- [ ] Proper input validation
- [ ] Cryptographic operations use constant-time implementations
- [ ] No potential integer overflow/underflow issues
- [ ] No memory safety issues (for unsafe code)

## Merge Process Checklist

### Pre-Merge

- [ ] All PR checks pass
- [ ] Required reviewers have approved
- [ ] Documentation is complete
- [ ] All TODOs addressed or converted to issues
- [ ] No regressions introduced

### Merge Strategy

- [ ] Use merge commit for large features
- [ ] Squash and merge for small fixes
- [ ] Ensure clean commit history
- [ ] Include proper commit message with issue references

### Post-Merge

- [ ] Verify deployment/integration
- [ ] Clean up feature branch
- [ ] Close related issues
- [ ] Update project documentation
- [ ] Notify team of significant changes

## Automated Tools

The following tools can help with the PR process:

1. **PR Checks Script**: `scripts/bitcoin/run_pr_checks.ps1`
   - Runs validation checks for Bitcoin module PRs

2. **Merge Automation**: `scripts/bitcoin/merge_pr.ps1`
   - Automates the process of merging a Bitcoin feature branch

3. **GitHub Workflow**: `.github/workflows/bitcoin-pr-checks.yml`
   - Automatically runs checks on PR creation and updates

## References

- [Bitcoin Development Framework v2.5](docs/bitcoin/DEVELOPMENT_FRAMEWORK.md)
- [Branching Strategy](BRANCHING_STRATEGY.md)
- [BIP Implementation Index](BIP_IMPLEMENTATION_INDEX.md)
- [Architecture Update Document](ARCHITECTURE_UPDATE.md)
- [PR Preparation Guide](PR_PREPARATION.md) 
