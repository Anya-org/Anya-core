# Checkpoint System

**Date:** 2025-03-02
**Version:** 1.0

## Overview

The checkpoint system provides automated snapshots of repository states at significant points in development, enhancing traceability and facilitating regression testing or rollbacks if needed. Checkpoints are a combination of Git tags and detailed documentation files.

## Features

- **Automated Checkpoint Creation**:
  - After merges to main branch
  - After significant pushes
  - After reaching commit thresholds
  - Manual checkpoint creation

- **AI Label Integration**:
  - Supports AIP (Anya Intelligence Pattern) labels
  - Automatically extracts AI labels from commit messages and PR titles

- **Comprehensive Documentation**:
  - Commit information
  - Files changed
  - Repository status at checkpoint time

## Using the Checkpoint System

### Creating Checkpoints Manually

```powershell
# Basic usage
.\scripts\create_checkpoint.ps1 -CheckpointName "feature_completion" -Message "Feature X completed"

# With AI Label
.\scripts\create_checkpoint.ps1 -CheckpointName "feature_completion" -Message "Feature X completed" -AiLabel "AIP-001"

# Push to remote repository
.\scripts\create_checkpoint.ps1 -CheckpointName "feature_completion" -Message "Feature X completed" -AiLabel "AIP-001" -PushToRemote
```

### Automated Checkpoints

The system automatically creates checkpoints:

1. When a PR is merged to main
2. After significant pushes to main
3. When commit count reaches a threshold since the last checkpoint

These are controlled by GitHub Actions workflows and local scripts.

### Viewing Checkpoints

Checkpoints can be viewed in several ways:

1. **Git Tags**:
   ```powershell
   git tag -l
   git show <tag-name>
   ```

2. **Documentation Files**:
   - Browse to `docs/checkpoints/` directory for detailed checkpoint information files

3. **GitHub Interface**:
   - View tags in the GitHub repository interface
   - View checkpoint files in the GitHub repository

## Checkpoint Naming Convention

- **Manual Checkpoints**: `[AI-Label-]<descriptive-name>`
- **Merge Checkpoints**: `merge_<branch-name>`
- **Push Checkpoints**: `push_<commit-hash>`
- **Commit Threshold Checkpoints**: `auto_commit_threshold`

## AI Label Integration

The checkpoint system supports AI labeling following this format:

- **AIR-XXX**: Anya Intelligence Research
- **AIS-XXX**: Anya Intelligence Security
- **AIT-XXX**: Anya Intelligence Testing
- **AIM-XXX**: Anya Intelligence Metrics
- **AIP-XXX**: Anya Intelligence Pattern
- **AIE-XXX**: Anya Intelligence Enhancement

Where XXX is a three-digit number (e.g., AIP-001).

## Files and Components

1. **create_checkpoint.ps1**:
   - Main script for creating checkpoint tags and files
   - Located in the `scripts` directory

2. **auto_checkpoint.ps1**:
   - Script for automating checkpoint creation based on commit thresholds
   - Located in the `scripts` directory

3. **checkpoint.yml**:
   - GitHub Actions workflow for automated checkpoint creation
   - Located in `.github/workflows` directory

4. **Checkpoint Files**:
   - Stored in `docs/checkpoints` directory
   - Format: `<checkpoint-name>-<timestamp>.md`

## Best Practices

1. Use meaningful names for manual checkpoints
2. Include AI labels when applicable
3. Push checkpoints to remote for team visibility
4. Use checkpoints for rollback references
5. Include checkpoint references in issue/ticket tracking

## Example Workflow

1. Developer completes feature implementation
2. Creates a PR with appropriate title including AI label if applicable
3. PR is reviewed and merged
4. Automated checkpoint is created
5. The checkpoint tag and file are pushed to the repository
6. Team can reference checkpoint in documentation and discussions
