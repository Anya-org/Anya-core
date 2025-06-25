# Git Workflow Testing Checklist

Use this checklist before, during, and after implementing changes to ensure a smooth git workflow.

## Before Starting Development

- [ ] Ensure branch protection is properly configured
  - [ ] Run `git status` to check current repository state
  - [ ] Ensure you're not on the main branch for development

- [ ] Create a fresh branch from updated main
  - [ ] `git checkout main`
  - [ ] `git pull origin main`
  - [ ] `git checkout -b feature/your-feature-name`
  
- [ ] Verify the development environment
  - [ ] Run `cargo check` to ensure code compiles
  - [ ] Run `cargo test` to verify tests pass
  - [ ] Check for any pre-existing issues

## During Development

- [ ] Make changes in small, logical commits
  - [ ] Use conventional commit format (`feat:`, `fix:`, etc.)
  - [ ] Include reference to issue numbers when applicable

- [ ] Keep branch up to date with main
  - [ ] `git fetch origin main`
  - [ ] `git rebase origin/main`
  - [ ] Resolve conflicts if any

- [ ] Regularly run tests
  - [ ] `cargo test`
  - [ ] `cargo clippy`
  - [ ] Fix any issues immediately

## Before Creating Pull Request

- [ ] Final sync with main
  - [ ] `git fetch origin main`
  - [ ] `git rebase origin/main`
  
- [ ] Run full test suite
  - [ ] `cargo test --all-features`
  - [ ] `cargo clippy -- -D warnings`
  - [ ] `cargo fmt -- --check`

- [ ] Review changes
  - [ ] `git log origin/main..HEAD`
  - [ ] `git diff origin/main..HEAD`

- [ ] Push changes to remote
  - [ ] `git push origin feature/your-feature-name`

## Creating Pull Request

- [ ] Create PR via GitHub
  - [ ] Use clear PR title (following conventional commit format)
  - [ ] Fill out PR description with all required information
  - [ ] Link to relevant issues

- [ ] Verify CI checks pass
  - [ ] All automated tests
  - [ ] Linting
  - [ ] Code coverage

## During Review Process

- [ ] Address review comments
  - [ ] Make requested changes
  - [ ] Respond to all comments
  - [ ] Push updates to same branch

- [ ] If main has advanced, rebase again
  - [ ] `git fetch origin main`
  - [ ] `git rebase origin/main`
  - [ ] Force-push updates if necessary: `git push --force-with-lease origin feature/your-feature-name`

## After Merging

- [ ] Clean up local branches
  - [ ] `git checkout main`
  - [ ] `git pull origin main`
  - [ ] `git branch -d feature/your-feature-name`

- [ ] Verify deployment/integration
  - [ ] Check that changes are properly integrated
  - [ ] Run post-merge verification if needed

- [ ] Update issue status
  - [ ] Close related issues if resolved
  - [ ] Update any related documentation

## Troubleshooting Git Issues

### If local branch gets messed up

```bash
# Save your changes
git stash

# Get a fresh copy of the branch
git fetch origin
git checkout -B feature/your-feature-name origin/feature/your-feature-name

# Apply your changes back
git stash pop
```

### If PR has conflicts with main

```bash
git fetch origin
git checkout feature/your-feature-name
git rebase origin/main
# Resolve conflicts
git push --force-with-lease origin feature/your-feature-name
```

### If you committed to the wrong branch

```bash
# Save changes
git stash

# Move to correct branch
git checkout correct-branch

# Apply changes
git stash pop
```
