# Anya Core Git Workflow Guide

*Last Updated: June 21, 2025*

## Table of Contents

- [Introduction](#introduction)
- [Git Workflow Overview](#git-workflow-overview)
- [Branch Strategy](#branch-strategy)
- [Development Process](#development-process)
- [Commit Guidelines](#commit-guidelines)
- [Pull Request Process](#pull-request-process)
- [Testing Requirements](#testing-requirements)
- [Merge Process](#merge-process)
- [Handling Release Branches](#handling-release-branches)
- [Post-Merge Automation](#post-merge-automation)
- [Troubleshooting Common Issues](#troubleshooting-common-issues)

## Introduction

This document outlines the standard git workflow for the Anya Core project. Following these guidelines ensures consistent, high-quality code and a clean repository history.

## Git Workflow Overview

We follow a feature branch workflow with pull requests, automated testing, and code reviews.

```
main                    ──────────────────────────────────▶
                         ╲     ╲           ╱      ╱
feature/add-layer2        ╲     ╲         ╱      ╱
                           ╲     ╲       ╱      ╱
bugfix/fix-stacks-client    ╲     ╲     ╱      ╱
                             ╲     ╲   ╱      ╱
```

## Branch Strategy

- **main**: Production-ready code, protected
- **feature/*****: Feature development
- **bugfix/*****: Bug fixes
- **hotfix/*****: Urgent production fixes
- **release/vX.Y.Z**: Release preparation

## Development Process

1. **Start Fresh**:

   ```bash
   git checkout main
   git pull
   ```

2. **Create a Branch**:

   ```bash
   git checkout -b feature/your-feature-name
   # OR
   git checkout -b bugfix/issue-description
   ```

3. **Development Loop**:

   ```bash
   # Make changes
   git add .
   git commit -m "feat: add feature X"
   # Run tests
   cargo test
   ```

4. **Keep Branch Updated**:

   ```bash
   git fetch origin
   git rebase origin/main
   # Resolve conflicts if any
   ```

5. **Push to Remote**:

   ```bash
   git push origin feature/your-feature-name
   ```

## Commit Guidelines

We follow [Conventional Commits](https://www.conventionalcommits.org/) format:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

Types:

- **feat**: New feature
- **fix**: Bug fix
- **docs**: Documentation
- **style**: Formatting changes
- **refactor**: Code change that neither fixes a bug nor adds a feature
- **perf**: Performance improvement
- **test**: Adding or updating tests
- **chore**: Maintenance tasks

Examples:

```
feat(layer2): add Default trait implementation for client types
fix(validation): resolve "use of moved value" error
docs: update installation instructions
```

## Pull Request Process

1. **Create PR**: Through GitHub interface
2. **PR Template**: Fill out the PR template completely
3. **CI Checks**: Ensure all automated checks pass
4. **Code Review**: Request reviews from relevant team members
5. **Iteration**: Address review comments
6. **Approval**: Obtain necessary approvals
7. **Merge**: Use the "Squash and merge" option

## Testing Requirements

All code changes must include:

1. **Unit Tests**: For new functions and methods
2. **Integration Tests**: For component interactions
3. **Manual Testing**: Follow test plan for UI/UX features
4. **CI Validation**: All tests must pass in the CI pipeline

## Merge Process

1. **Pre-merge Checks**:
   - All CI checks passing
   - Approvals from required reviewers
   - No unresolved conversations
   - Branch up to date with main

2. **Merge Options**:
   - **Squash and merge**: For feature branches (default)
   - **Merge commit**: For release branches
   - **Rebase**: For hotfixes or simple changes

3. **Post-merge Actions**:
   - Delete the branch
   - Update related issues
   - Notify team if needed

## Handling Release Branches

1. **Creating Release**:

   ```bash
   git checkout main
   git pull
   git checkout -b release/vX.Y.Z
   # Update version numbers
   git commit -m "chore(release): prepare vX.Y.Z"
   ```

2. **Publishing Release**:

   ```bash
   git tag vX.Y.Z
   git push origin vX.Y.Z
   git push origin release/vX.Y.Z
   ```

## Post-Merge Automation

After merging to main:

1. CI builds documentation
2. CI runs comprehensive tests
3. CI generates release artifacts

## Troubleshooting Common Issues

### Merge Conflicts

```bash
# Resolve conflicts
git add .
git rebase --continue
# If things get too complicated
git rebase --abort
```

### Force Push Required

```bash
# Use with caution, only on your feature branches
git push --force-with-lease origin feature/your-feature-name
```

### Commit to Wrong Branch

```bash
git stash
git checkout correct-branch
git stash pop
```

### Need to Undo Last Commit

```bash
# Keep changes, just undo commit
git reset --soft HEAD~1
# Discard changes completely
git reset --hard HEAD~1

## See Also

- [Architecture](docs/ARCHITECTURE.md)
- [Master Implementation Plan](MASTER_IMPLEMENTATION_PLAN_CANONICAL.md)
- [Contributing Guide](docs/CONTRIBUTING.md)
```
