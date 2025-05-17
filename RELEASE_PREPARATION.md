git checkout -b release/0.3.0-rc1
git push -u origin release/0.3.0-rc1# Release Candidate Preparation Guide

This document outlines the steps needed to prepare the Anya Core codebase for the Release Candidate (RC) version.

## 1. Version Update

- Update the version number in `Cargo.toml` to `0.3.0-rc.1`
- Update all dependency references to match the RC version

## 2. Branch Management

### 2.1. Branch Strategy

1. Create a new `release/0.3.0-rc1` branch from `develop`
2. Freeze feature development on this branch
3. Only allow bug fixes to be merged into the release branch
4. Maintain `develop` for ongoing feature development

### 2.2. Branch Commands

```bash
# Create release branch
git checkout develop
git pull origin develop
git checkout -b release/0.3.0-rc1
git push -u origin release/0.3.0-rc1

# For bug fixes to the release branch
git checkout release/0.3.0-rc1
git pull
# Make changes
git commit -m "Fix: description of the bug fix"
git push origin release/0.3.0-rc1
```

## 3. Documentation Updates

- [x] Update main README.md with HSM feature information
- [x] Create HSM feature guide documentation
- [ ] Review and update all API documentation
- [ ] Update installation guide with feature flag information
- [ ] Create release notes document

## 4. Testing Requirements

### 4.1. Required Tests

- [ ] Full unit test suite must pass
- [ ] Integration tests must pass
- [ ] Feature flag tests must pass (both with and without HSM enabled)
- [ ] Cross-platform testing (Linux, macOS, Windows)
- [ ] Hardware integration testing when applicable

### 4.2. Test Commands

```bash
# Run all tests without HSM
cargo test --no-default-features

# Run all tests with HSM
cargo test --features hsm

# Run full test suite with all features
cargo test --features complete
```

## 5. Build Verification

- [ ] Verify the build succeeds on all target platforms
- [ ] Ensure all feature flag combinations build successfully
- [ ] Check for compiler warnings and address them
- [ ] Validate binary size and dependencies

## 6. Known Issues

The following known issues are accepted for the RC release:

1. HSM Hardware detection is limited and requires manual configuration
2. Some TPM operations may fail on certain hardware configurations
3. Bitcoin-specific HSM operations need further testing with real hardware
4. Performance optimizations pending for hardware providers

## 7. Final Release Checklist

Before creating the final RC tag:

- [ ] All documentation updated and reviewed
- [ ] All tests passing
- [ ] Build verification completed
- [ ] Known issues documented
- [ ] Release notes finalized
- [ ] Cross-team sign-off obtained

## 8. Creating the RC Tag

Once all checklist items are complete:

```bash
git tag -a v0.3.0-rc1 -m "Release Candidate 1 for version 0.3.0"
git push origin v0.3.0-rc1
```

## 9. Post-RC Process

1. Monitor for critical issues
2. Address RC feedback
3. Prepare for final release
4. Consider RC2 if significant issues are found
