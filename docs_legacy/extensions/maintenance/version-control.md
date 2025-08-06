# Version Control and Git Workflows

[AIR-3][AIS-3][AIT-3][RES-3] **Comprehensive version control guide for Anya Core extension development with Git workflows, branching strategies, and collaborative development practices.**

*Last updated: June 7, 2025*

## Table of Contents

- [Git Workflow Overview](#git-workflow-overview)
- [Branching Strategy](#branching-strategy)
- [Commit Guidelines](#commit-guidelines)
- [Release Management](#release-management)
- [Collaborative Development](#collaborative-development)
- [Code Review Process](#code-review-process)
- [Continuous Integration](#continuous-integration)
- [Security and Compliance](#security-and-compliance)
- [Advanced Git Techniques](#advanced-git-techniques)

## Git Workflow Overview

Anya Core extension development follows a structured Git workflow optimized for Bitcoin, Web5, and ML development:

### Core Principles

1. **Feature Branch Workflow**: All development happens in feature branches
2. **Semantic Versioning**: Strict adherence to SemVer for releases
3. **Conventional Commits**: Standardized commit message format
4. **Automated Testing**: CI/CD pipelines for all changes
5. **Security First**: Security scanning and audit trails

### Repository Structure

```
my-extension/
├── .git/                      # Git repository data
├── .github/                   # GitHub workflows and templates
│   ├── workflows/            # CI/CD workflows
│   ├── ISSUE_TEMPLATE/       # Issue templates
│   └── PULL_REQUEST_TEMPLATE.md
├── src/                      # Source code
├── tests/                    # Test suite
├── docs/                     # Documentation
├── examples/                 # Usage examples
├── .gitignore               # Git ignore rules
├── .gitattributes           # Git attributes
├── CHANGELOG.md             # Release changelog
└── CONTRIBUTING.md          # Contribution guidelines
```

### Git Configuration

```bash
# Global Git configuration for Anya development
git config --global user.name "Your Name"
git config --global user.email "your.email@example.com"
git config --global init.defaultBranch main
git config --global pull.rebase true
git config --global core.autocrlf input
git config --global core.editor "code --wait"

# Anya-specific configurations
git config --global commit.template ~/.gitmessage
git config --global core.hooksPath ~/.config/git/hooks
```

### Git Message Template

Create `~/.gitmessage`:

```
# <type>(<scope>): <subject>
#
# <body>
#
# <footer>
#
# Type can be:
#   feat     (new feature)
#   fix      (bug fix)
#   docs     (documentation)
#   style    (formatting, missing semi colons, etc)
#   refactor (refactoring production code)
#   test     (adding missing tests, refactoring tests)
#   chore    (updating grunt tasks etc)
#   perf     (performance improvements)
#   ci       (CI related changes)
#   build    (build system changes)
#   security (security improvements)
#
# Scope can be:
#   bitcoin  (Bitcoin-related changes)
#   web5     (Web5-related changes)
#   ml       (ML-related changes)
#   core     (Core extension changes)
#   docs     (Documentation changes)
#   tests    (Test-related changes)
#
# Subject line should:
#   - Use imperative, present tense ("change" not "changed")
#   - Not capitalize first letter
#   - Not end with a period
#   - Be no longer than 50 characters
#
# Body should:
#   - Explain what and why vs. how
#   - Include motivation for the change
#   - Wrap at 72 characters
#
# Footer should:
#   - Reference issues and pull requests
#   - Include breaking change information
#   - Note any co-authors
```

## Branching Strategy

### GitFlow for Extensions

```
main
├── develop
│   ├── feature/bitcoin-wallet-integration
│   ├── feature/web5-did-resolver
│   ├── feature/ml-inference-optimization
│   └── hotfix/security-patch-cve-2025-1234
├── release/v1.2.0
└── hotfix/v1.1.1
```

### Branch Types

#### Main Branch

- **Purpose**: Production-ready code
- **Protection**: Branch protection enabled
- **Merges**: Only from release and hotfix branches
- **Naming**: `main`

```bash
# Main branch setup
git checkout main
git branch --set-upstream-to=origin/main main
```

#### Develop Branch

- **Purpose**: Integration branch for features
- **Protection**: Require pull request reviews
- **Merges**: From feature branches
- **Naming**: `develop`

```bash
# Create and setup develop branch
git checkout -b develop main
git push -u origin develop
```

#### Feature Branches

- **Purpose**: New features and enhancements
- **Lifetime**: Until feature completion
- **Naming**: `feature/<description>`

```bash
# Create feature branch
git checkout develop
git pull origin develop
git checkout -b feature/bitcoin-lightning-integration

# Work on feature
git add .
git commit -m "feat(bitcoin): add lightning network channel management"

# Push feature branch
git push -u origin feature/bitcoin-lightning-integration
```

#### Release Branches

- **Purpose**: Prepare new release versions
- **Lifetime**: Until release completion
- **Naming**: `release/v<version>`

```bash
# Create release branch
git checkout develop
git pull origin develop
git checkout -b release/v1.2.0

# Prepare release
echo "1.2.0" > VERSION
git add VERSION
git commit -m "chore: bump version to 1.2.0"

# Finish release
git checkout main
git merge --no-ff release/v1.2.0
git tag -a v1.2.0 -m "Release version 1.2.0"
git checkout develop
git merge --no-ff release/v1.2.0
```

#### Hotfix Branches

- **Purpose**: Critical fixes for production
- **Lifetime**: Until fix deployment
- **Naming**: `hotfix/v<version>` or `hotfix/<issue>`

```bash
# Create hotfix branch
git checkout main
git pull origin main
git checkout -b hotfix/v1.1.1

# Apply fix
git add .
git commit -m "fix(security): patch CVE-2025-1234 in bitcoin RPC client"

# Finish hotfix
git checkout main
git merge --no-ff hotfix/v1.1.1
git tag -a v1.1.1 -m "Hotfix version 1.1.1"
git checkout develop
git merge --no-ff hotfix/v1.1.1
```

### Branch Protection Rules

```yaml
# .github/branch-protection.yml
protection_rules:
  main:
    required_status_checks:
      - continuous-integration
      - security-scan
      - performance-test
    enforce_admins: true
    required_pull_request_reviews:
      required_approving_review_count: 2
      dismiss_stale_reviews: true
      require_code_owner_reviews: true
    restrictions:
      users: []
      teams: ["core-maintainers"]

  develop:
    required_status_checks:
      - continuous-integration
      - unit-tests
    required_pull_request_reviews:
      required_approving_review_count: 1
      dismiss_stale_reviews: true
```

## Commit Guidelines

### Conventional Commits

Follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

### Commit Types

#### Feature Development

```bash
# New features
git commit -m "feat(bitcoin): add multi-signature wallet support"
git commit -m "feat(web5): implement DID key rotation"
git commit -m "feat(ml): add ONNX model optimization"

# Enhancements
git commit -m "feat(core): improve extension loading performance"
git commit -m "feat(bitcoin): add fee estimation with RBF support"
```

#### Bug Fixes

```bash
# Bug fixes
git commit -m "fix(bitcoin): resolve wallet balance calculation error"
git commit -m "fix(web5): handle DID resolution timeout gracefully"
git commit -m "fix(ml): memory leak in model inference loop"

# Critical fixes
git commit -m "fix(security): prevent private key exposure in logs"
```

#### Documentation

```bash
# Documentation updates
git commit -m "docs(api): add Bitcoin wallet API examples"
git commit -m "docs(readme): update installation instructions"
git commit -m "docs(web5): add DID method comparison guide"
```

#### Performance and Optimization

```bash
# Performance improvements
git commit -m "perf(bitcoin): optimize UTXO selection algorithm"
git commit -m "perf(ml): implement model caching for faster inference"
git commit -m "perf(core): reduce extension startup time by 50%"
```

#### Refactoring

```bash
# Code refactoring
git commit -m "refactor(bitcoin): extract transaction builder to separate module"
git commit -m "refactor(web5): simplify DID resolver interface"
git commit -m "refactor(ml): reorganize model management code"
```

#### Testing

```bash
# Test additions
git commit -m "test(bitcoin): add integration tests for Lightning Network"
git commit -m "test(web5): increase DID resolution test coverage to 95%"
git commit -m "test(ml): add performance benchmarks for model inference"
```

#### Build and CI

```bash
# Build system changes
git commit -m "build: update Rust to 1.70.0"
git commit -m "ci: add automated security scanning"
git commit -m "chore: update dependencies to latest versions"
```

### Commit Message Best Practices

#### Subject Line

- Use imperative mood ("add" not "added")
- Keep under 50 characters
- Don't end with period
- Be specific and descriptive

#### Body

- Wrap at 72 characters
- Explain what and why, not how
- Use present tense
- Include motivation and context

#### Footer

- Reference issues and pull requests
- Include breaking change information
- Note co-authors

### Example Quality Commits

```bash
# Excellent commit example
git commit -m "feat(bitcoin): implement hardware wallet integration

Add support for Ledger and Trezor hardware wallets through the HWI
library. This enables secure private key management for Bitcoin
transactions without exposing keys to the host system.

Changes include:
- HWI library integration
- Hardware wallet detection and enumeration
- Transaction signing through hardware devices
- Error handling for device communication failures

Closes #123
Breaks compatibility with wallet configurations using software-only keys

Co-authored-by: Alice Developer <alice@example.com>"
```

### Commit Hooks

Set up commit hooks for quality assurance:

```bash
# Pre-commit hook (.git/hooks/pre-commit)
#!/bin/sh
# Anya Core extension pre-commit hook

echo "Running pre-commit checks..."

# Rust formatting check
if ! cargo fmt -- --check; then
    echo "Error: Code is not properly formatted. Run 'cargo fmt' to fix."
    exit 1
fi

# Rust linting
if ! cargo clippy -- -D warnings; then
    echo "Error: Clippy found issues. Fix them before committing."
    exit 1
fi

# Run tests
if ! cargo test --all-features; then
    echo "Error: Tests failed. Fix them before committing."
    exit 1
fi

# Security audit
if ! cargo audit; then
    echo "Warning: Security vulnerabilities found. Consider updating dependencies."
fi

echo "Pre-commit checks passed!"
```

```bash
# Commit message hook (.git/hooks/commit-msg)
#!/bin/sh
# Anya Core extension commit message validation

commit_regex='^(feat|fix|docs|style|refactor|test|chore|perf|ci|build|security)(\(.+\))?: .{1,50}'

if ! grep -qE "$commit_regex" "$1"; then
    echo "Invalid commit message format!"
    echo "Format: <type>[optional scope]: <description>"
    echo "Example: feat(bitcoin): add multi-signature wallet support"
    exit 1
fi
```

## Release Management

### Semantic Versioning

Follow [Semantic Versioning](https://semver.org/) strictly:

- **MAJOR**: Breaking changes or incompatible API changes
- **MINOR**: New features that are backward compatible
- **PATCH**: Backward compatible bug fixes

### Version Bumping

```bash
# Patch release (bug fixes)
echo "1.2.1" > VERSION
git add VERSION
git commit -m "chore: bump version to 1.2.1"
git tag -a v1.2.1 -m "Patch release 1.2.1"

# Minor release (new features)
echo "1.3.0" > VERSION
git add VERSION
git commit -m "chore: bump version to 1.3.0"
git tag -a v1.3.0 -m "Minor release 1.3.0"

# Major release (breaking changes)
echo "2.0.0" > VERSION
git add VERSION
git commit -m "chore: bump version to 2.0.0"
git tag -a v2.0.0 -m "Major release 2.0.0"
```

### Release Process

#### 1. Prepare Release Branch

```bash
# Create release branch
git checkout develop
git pull origin develop
git checkout -b release/v1.3.0

# Update version files
echo "1.3.0" > VERSION
sed -i 's/version = "1.2.0"/version = "1.3.0"/' Cargo.toml
sed -i 's/version = "1.2.0"/version = "1.3.0"/' extension.toml

# Update changelog
anya changelog generate --version 1.3.0 --output CHANGELOG.md
```

#### 2. Finalize Release

```bash
# Commit version updates
git add VERSION Cargo.toml extension.toml CHANGELOG.md
git commit -m "chore: prepare release 1.3.0"

# Run release tests
cargo test --all-features --release
anya test extension . --comprehensive

# Push release branch
git push -u origin release/v1.3.0
```

#### 3. Merge and Tag

```bash
# Merge to main
git checkout main
git pull origin main
git merge --no-ff release/v1.3.0

# Create and push tag
git tag -a v1.3.0 -m "Release version 1.3.0

Features:
- Added Bitcoin Lightning Network support
- Improved Web5 DID resolution performance
- Enhanced ML model caching

Bug fixes:
- Fixed wallet balance calculation edge case
- Resolved DID document parsing issue
- Fixed memory leak in model inference

Breaking changes:
- Updated Bitcoin RPC interface (see migration guide)

Full changelog: https://github.com/user/extension/compare/v1.2.0...v1.3.0"

git push origin main --tags

# Merge back to develop
git checkout develop
git merge --no-ff release/v1.3.0
git push origin develop

# Clean up release branch
git branch -d release/v1.3.0
git push origin --delete release/v1.3.0
```

### Automated Release Workflow

```yaml
# .github/workflows/release.yml
name: Release
on:
  push:
    tags:
      - 'v*'

jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
        with:
          fetch-depth: 0

      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Build release
        run: |
          cargo build --release --all-features
          anya ext package --release

      - name: Run tests
        run: |
          cargo test --all-features --release
          anya test extension . --comprehensive

      - name: Security scan
        run: |
          cargo audit
          anya security-scan .

      - name: Create GitHub release
        uses: actions/create-release@v1
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          tag_name: ${{ github.ref }}
          release_name: Release ${{ github.ref }}
          body_path: RELEASE_NOTES.md
          draft: false
          prerelease: false

      - name: Publish to registry
        run: |
          anya ext publish --registry community
        env:
          ANYA_TOKEN: ${{ secrets.ANYA_TOKEN }}
```

## Collaborative Development

### Fork and Pull Request Workflow

#### For External Contributors

```bash
# Fork repository
gh repo fork anya-org/my-extension

# Clone fork
git clone https://github.com/your-username/my-extension.git
cd my-extension

# Add upstream remote
git remote add upstream https://github.com/anya-org/my-extension.git

# Create feature branch
git checkout -b feature/my-contribution

# Make changes and commit
git add .
git commit -m "feat(bitcoin): add new transaction validation feature"

# Push to fork
git push origin feature/my-contribution

# Create pull request
gh pr create --base develop --title "feat(bitcoin): add new transaction validation feature"
```

#### For Internal Contributors

```bash
# Clone repository
git clone https://github.com/anya-org/my-extension.git
cd my-extension

# Create feature branch
git checkout develop
git pull origin develop
git checkout -b feature/internal-feature

# Development workflow
git add .
git commit -m "feat(web5): enhance DID document validation"
git push -u origin feature/internal-feature

# Create pull request
gh pr create --base develop
```

### Merge Strategies

#### Feature Merges

```bash
# Squash and merge for clean history
git checkout develop
git merge --squash feature/bitcoin-lightning-integration
git commit -m "feat(bitcoin): add Lightning Network integration

Complete Lightning Network integration including:
- Channel management
- Payment routing
- Invoice generation
- Watchtower support

Closes #456"
```

#### Release Merges

```bash
# No-fast-forward merge to preserve branch structure
git checkout main
git merge --no-ff release/v1.3.0
```

### Conflict Resolution

```bash
# When conflicts occur during merge
git checkout feature/my-feature
git rebase develop

# Resolve conflicts in editor
# Stage resolved files
git add resolved-file.rs

# Continue rebase
git rebase --continue

# Force push rebased branch
git push --force-with-lease origin feature/my-feature
```

## Code Review Process

### Pull Request Template

```markdown
<!-- .github/PULL_REQUEST_TEMPLATE.md -->
## Description
Brief description of changes and motivation.

## Type of Change
- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update
- [ ] Performance improvement
- [ ] Refactoring (no functional changes)

## Component Areas
- [ ] Bitcoin integration
- [ ] Web5 identity/credentials
- [ ] ML inference/training
- [ ] Core extension system
- [ ] Security/cryptography
- [ ] Documentation

## Testing
- [ ] Unit tests pass locally
- [ ] Integration tests pass locally
- [ ] Performance tests (if applicable)
- [ ] Manual testing completed

## Security Considerations
- [ ] No sensitive data exposed
- [ ] Cryptographic functions reviewed
- [ ] Input validation implemented
- [ ] Security scan passed

## Documentation
- [ ] Code is self-documenting
- [ ] API documentation updated
- [ ] README updated (if needed)
- [ ] Changelog updated

## Checklist
- [ ] My code follows the project's style guidelines
- [ ] I have performed a self-review of my own code
- [ ] I have commented my code, particularly in hard-to-understand areas
- [ ] I have made corresponding changes to the documentation
- [ ] My changes generate no new warnings
- [ ] I have added tests that prove my fix is effective or that my feature works
- [ ] New and existing unit tests pass locally with my changes

## Related Issues
Closes #(issue number)
```

### Review Guidelines

#### For Reviewers

1. **Code Quality**
   - Check for Rust best practices
   - Verify error handling
   - Ensure memory safety
   - Review algorithm efficiency

2. **Security Review**
   - Check for vulnerabilities
   - Verify cryptographic usage
   - Review permission requirements
   - Validate input sanitization

3. **Bitcoin-Specific Review**
   - Verify BIP compliance
   - Check transaction handling
   - Review script validation
   - Ensure fee calculation accuracy

4. **Web5-Specific Review**
   - Verify DID method compliance
   - Check credential format validity
   - Review protocol implementation
   - Ensure privacy preservation

5. **ML-Specific Review**
   - Check model compatibility
   - Review inference accuracy
   - Verify resource usage
   - Validate performance metrics

#### Review Commands

```bash
# Checkout PR locally
gh pr checkout 123

# Run comprehensive tests
cargo test --all-features
anya test extension . --comprehensive

# Security scan
cargo audit
anya security-scan .

# Performance analysis
cargo bench
anya benchmark extension .

# Code quality check
cargo clippy -- -D warnings
cargo fmt -- --check
```

### Automated Review Checks

```yaml
# .github/workflows/pr-checks.yml
name: Pull Request Checks
on:
  pull_request:
    branches: [main, develop]

jobs:
  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          components: rustfmt, clippy

      - name: Check formatting
        run: cargo fmt -- --check

      - name: Run clippy
        run: cargo clippy -- -D warnings

  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Run tests
        run: cargo test --all-features

      - name: Run integration tests
        run: anya test extension . --ci

  security:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Security audit
        run: cargo audit

      - name: Anya security scan
        run: anya security-scan .
```

## Continuous Integration

### GitHub Actions Workflow

```yaml
# .github/workflows/ci.yml
name: Continuous Integration

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main, develop]

env:
  CARGO_TERM_COLOR: always

jobs:
  check:
    name: Check
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          profile: minimal
          toolchain: stable
          override: true
      - uses: actions-rs/cargo@v1
        with:
          command: check

  test:
    name: Test Suite
    runs-on: ubuntu-latest
    strategy:
      matrix:
        rust: [stable, beta, nightly]
        os: [ubuntu-latest, windows-latest, macos-latest]
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          profile: minimal
          toolchain: ${{ matrix.rust }}
          override: true
      - uses: actions-rs/cargo@v1
        with:
          command: test
          args: --all-features

  fmt:
    name: Rustfmt
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          profile: minimal
          toolchain: stable
          override: true
          components: rustfmt
      - uses: actions-rs/cargo@v1
        with:
          command: fmt
          args: --all -- --check

  clippy:
    name: Clippy
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          profile: minimal
          toolchain: stable
          override: true
          components: clippy
      - uses: actions-rs/cargo@v1
        with:
          command: clippy
          args: -- -D warnings

  security:
    name: Security Audit
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/audit-check@v1
        with:
          token: ${{ secrets.GITHUB_TOKEN }}

  coverage:
    name: Code Coverage
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true
      - uses: actions-rs/tarpaulin@v0.1
        with:
          args: '--all-features --workspace --timeout 600 --out Xml'
      - uses: codecov/codecov-action@v3
```

### Pre-commit Configuration

```yaml
# .pre-commit-config.yaml
repos:
  - repo: https://github.com/pre-commit/pre-commit-hooks
    rev: v4.4.0
    hooks:
      - id: trailing-whitespace
      - id: end-of-file-fixer
      - id: check-yaml
      - id: check-toml
      - id: check-merge-conflict

  - repo: local
    hooks:
      - id: rust-linting
        name: Rust linting
        entry: cargo clippy --all-targets --all-features -- -D warnings
        language: system
        types: [rust]
        pass_filenames: false

      - id: rust-formatting
        name: Rust formatting
        entry: cargo fmt --all -- --check
        language: system
        types: [rust]
        pass_filenames: false

      - id: rust-testing
        name: Rust testing
        entry: cargo test --all-features
        language: system
        types: [rust]
        pass_filenames: false
```

## Security and Compliance

### Signed Commits

```bash
# Generate GPG key
gpg --full-generate-key

# List keys
gpg --list-secret-keys --keyid-format LONG

# Configure Git to use GPG key
git config --global user.signingkey YOUR_GPG_KEY_ID
git config --global commit.gpgsign true
git config --global tag.gpgsign true

# Sign commits
git commit -S -m "feat(bitcoin): add signed transaction support"
```

### Audit Trail

```bash
# View commit history with signatures
git log --show-signature

# Verify specific commit
git verify-commit HEAD

# View detailed commit information
git show --show-signature HEAD
```

### Security Scanning

```bash
# Dependency audit
cargo audit

# Anya security scan
anya security-scan .

# SAST analysis
cargo clippy -- -W clippy::all

# License compliance
cargo license
```

### Compliance Tracking

```bash
# Track compliance status
anya compliance-check .

# Generate compliance report
anya compliance-report --format pdf --output compliance-report.pdf

# Audit log generation
anya audit-log --format json --output audit-log.json
```

## Advanced Git Techniques

### Git Worktrees

```bash
# Create worktree for parallel development
git worktree add ../my-extension-feature feature/bitcoin-integration
cd ../my-extension-feature

# Work in isolation
git add .
git commit -m "feat(bitcoin): implement new feature"

# Switch back to main worktree
cd ../my-extension

# Remove worktree when done
git worktree remove ../my-extension-feature
```

### Bisecting for Bug Hunting

```bash
# Start bisect
git bisect start

# Mark bad commit (current)
git bisect bad

# Mark good commit (known working)
git bisect good v1.2.0

# Git will checkout middle commit
# Test and mark as good or bad
cargo test --all-features
git bisect good  # or git bisect bad

# Continue until bug is found
# Reset when done
git bisect reset
```

### Advanced Rebasing

```bash
# Interactive rebase for commit cleanup
git rebase -i HEAD~5

# Squash commits
git rebase -i --autosquash HEAD~3

# Rebase with strategy
git rebase -X ours develop

# Preserve merge commits
git rebase --preserve-merges develop
```

### Git Hooks for Automation

```bash
# Post-commit hook for notifications
#!/bin/sh
# .git/hooks/post-commit

commit_hash=$(git rev-parse HEAD)
commit_message=$(git log -1 --pretty=%B)

# Notify team of security-related commits
if echo "$commit_message" | grep -q "security\|fix"; then
    curl -X POST "https://hooks.slack.com/services/YOUR/SLACK/WEBHOOK" \
         -H 'Content-type: application/json' \
         --data "{\"text\":\"Security commit: $commit_hash - $commit_message\"}"
fi
```

### Repository Maintenance

```bash
# Garbage collection
git gc --aggressive --prune=now

# Verify repository integrity
git fsck --full

# Clean up remote references
git remote prune origin

# Compress repository
git repack -a -d --depth=250 --window=250
```

## Related Documentation

- **[Maintenance Overview](./README.md)**: General maintenance practices
- **[Updates](./updates.md)**: Extension update management
- **[Deprecation](./deprecation.md)**: Deprecation management
- **[Development Guide](../development/README.md)**: Extension development practices
- **[Publishing Guide](../publishing/README.md)**: Extension publishing process

For Git workflow support and best practices, visit the [Anya Core Documentation](https://docs.anya.org) or join the [Developer Community](https://discord.gg/anya).
