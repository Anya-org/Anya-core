# Checkpoint Creation Guide

**Date:** 2025-03-02
**Version:** 1.0

## Quick Start

### Creating a Checkpoint Manually

```powershell
# Navigate to the repository root
cd <repository-path>

# Create a checkpoint
.\scripts\create_checkpoint.ps1 -CheckpointName "descriptive_name" -Message "Detailed checkpoint message"

# Create a checkpoint with AI label
.\scripts\create_checkpoint.ps1 -CheckpointName "descriptive_name" -Message "Detailed checkpoint message" -AiLabel "AIP-001"

# Create and push a checkpoint
.\scripts\create_checkpoint.ps1 -CheckpointName "descriptive_name" -Message "Detailed checkpoint message" -AiLabel "AIP-001" -PushToRemote
```

## When to Create Checkpoints

Create checkpoints at significant points in development:

- After completing a major feature
- After fixing a critical bug
- Before making significant architectural changes
- At release points
- Before dependency upgrades

## AI Labeling System

When creating checkpoints, use appropriate AI labels when applicable:

| Label | Description | Use Case |
|-------|-------------|----------|
| AIR-XXX | Anya Intelligence Research | Research or experimental features |
| AIS-XXX | Anya Intelligence Security | Security-related features or fixes |
| AIT-XXX | Anya Intelligence Testing | Test infrastructure or methodology |
| AIM-XXX | Anya Intelligence Metrics | Metrics and monitoring features |
| AIP-XXX | Anya Intelligence Pattern | Design patterns and architectural changes |
| AIE-XXX | Anya Intelligence Enhancement | General enhancements and improvements |

Where XXX is a three-digit number (e.g., AIP-001).

## Checkpoint Naming Best Practices

- Use snake_case for checkpoint names
- Be descriptive but concise
- Include the feature or component name when relevant
- Examples:
  - `user_authentication_complete`
  - `bitcoin_anchoring_implemented`
  - `performance_optimization_phase1`
  - `security_fix_cve_2025_1234`

## Automatic Checkpoints

Automated checkpoints are created for:

1. **Pull Request Merges**:
   - When a PR is merged to main
   - Naming convention: `merge_<branch-name>`

2. **Significant Pushes**:
   - When pushing directly to main
   - Naming convention: `push_<commit-hash>`

3. **Commit Thresholds**:
   - When commit count reaches threshold since last checkpoint
   - Naming convention: `auto_commit_threshold`

## Using Checkpoints for Development

### Listing Available Checkpoints

```powershell
# List all Git tags (checkpoints)
git tag -l

# View checkpoint documentation
Get-ChildItem -Path "docs/checkpoints" | Sort-Object LastWriteTime
```

### Comparing With a Checkpoint

```powershell
# Compare current state with a checkpoint
git diff <checkpoint-tag>

# See what files changed since a checkpoint
git diff --name-status <checkpoint-tag>
```

### Restoring to a Checkpoint (Caution!)

```powershell
# First create a backup branch of your current state
git checkout -b backup_before_restore

# Restore to a checkpoint
git checkout <checkpoint-tag>
git checkout -b restored_from_checkpoint
```

## Checkpoint File Structure

Each checkpoint creates a Markdown file in `docs/checkpoints/` with:

1. Checkpoint name and timestamp
2. AI label (if applicable)
3. Message describing the checkpoint
4. Commit information at checkpoint time
5. Files changed in the latest commit
6. Repository status overview

## Troubleshooting

### Checkpoint Creation Fails

1. Ensure you're at the repository root
2. Check for any pending changes (git status)
3. Verify the directory structure (docs/checkpoints/ should exist)
4. Check AI label format (if used)

### Checkpoint Not Showing in Remote

1. Use the `-PushToRemote` flag
2. Manually push tags: `git push origin <tag-name>`
3. Check for push permissions

## Additional Resources

- [Full Checkpoint System Documentation](CHECKPOINT_SYSTEM.md)
- [Git Tag Documentation](https://git-scm.com/docs/git-tag)
- [GitHub Releases](https://docs.github.com/en/repositories/releasing-projects-on-github/about-releases)

## Support

For assistance with the checkpoint system, contact the DevOps team or repository maintainers.
