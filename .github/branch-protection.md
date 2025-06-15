# Branch Protection Policy for Anya Core

## Protected Branches

### Main Branch (`main`)
- **Status Required Checks**: All CI workflows must pass
- **Require branches to be up to date**: Yes
- **Require signed commits**: Yes
- **Include administrators**: Yes
- **Allow force pushes**: No
- **Allow deletions**: No

## Required Status Checks

1. **Continuous Integration (`ci.yml`)**
   - Rust compilation check
   - Clippy linting
   - Cargo fmt formatting
   - Unit tests
   - Integration tests

2. **Security Scans**
   - Dependency audit
   - CodeQL analysis
   - SAST scanning

3. **Quality Gates**
   - Code coverage minimum 80%
   - No high-severity vulnerabilities
   - Documentation build success

## Branch Protection Rules

### Direct Push Restrictions
- No direct pushes to `main` branch
- All changes must go through Pull Requests
- Minimum 1 required reviewer for PRs
- Dismiss stale reviews when new commits are pushed

### Commit Requirements
- All commits must be signed with GPG
- Commit messages must follow conventional format
- No merge commits (squash and merge only)

### Automated Enforcement
- Branch protection is enforced via GitHub settings
- CI workflows block merging on failure
- Automated dependency updates via Dependabot

## Implementation Steps

1. Enable branch protection in GitHub repository settings
2. Configure required status checks
3. Set up GPG commit signing
4. Configure Dependabot for security updates
5. Add CODEOWNERS file for review requirements
