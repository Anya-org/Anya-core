# Branch Rule Violation Resolution Guide

*Created: June 23, 2025*

## Current Issue

A push attempt to the `main` branch was rejected due to the following repository rule violations:

1. Changes must be made through a pull request
2. Changes must be made through the merge queue
3. The branch must not contain merge commits

Specifically, commit `b1123bc76e63c530d430dfd1c94d5b57324a56d6` was flagged as a merge commit that violates the rules.

## Quick Resolution Steps

Follow these steps to resolve the current issue:

### 1. Identify Changes Needing Migration

```bash
# View the problematic merge commit
git show b1123bc76e63c530d430dfd1c94d5b57324a56d6

# List commits that are ahead of origin/main
git log --oneline main --not origin/main
```

### 2. Create a Clean Branch

```bash
# Create a new branch from the latest remote main
git checkout -b clean-updates origin/main

# Cherry-pick necessary commits (excluding the merge commit)
# For each commit to include:
git cherry-pick <commit-hash>
```

### 3. Push and Create PR

```bash
# Push the new branch
git push origin clean-updates

# Create a Pull Request in GitHub
# Ensure it passes all checks and gets proper review
```

### 4. Use the Merge Queue

Once approved, use the GitHub merge queue rather than directly merging to ensure compliance with repository rules.

## Preventing Future Issues

1. **No Direct Pushes to Main**: Always work in feature branches
2. **No Merge Commits to Main**: Use squash merge or rebase
3. **Always Use Pull Requests**: Never bypass the code review process
4. **Respect the Merge Queue**: Wait for CI checks and required approvals

## Note on Branch Management

As our detailed [Git Workflow](./GIT_WORKFLOW.md) explains, our `main` branch is protected and requires a specific process for changes. This document supplements that workflow with specific guidance for resolving the current branch rule violations.

This is a temporary document meant to address the immediate issue. For long-term workflow guidance, refer to our [Git Workflow](./GIT_WORKFLOW.md).
