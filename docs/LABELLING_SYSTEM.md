# Comprehensive Labeling System for Multi-Repository Projects

This document describes the comprehensive labeling system used across all Anya Core repositories, including the tools and procedures for maintaining consistency.

## Overview

The Anya Core ecosystem uses a sophisticated labeling system that follows the Bitcoin Development Framework v2.5 standards. This system ensures all components are properly categorized for AI readiness, security, performance, and compliance across multiple repositories.

## Key Tools and Scripts

### 1. Batch Commit Script

#### Bash Version (`batch_commit.sh`)

This script allows you to commit changes to multiple repositories with proper labeling in a single operation.

##### Usage:

```bash
./scripts/batch_commit.sh -m "Commit message" -t "feat" -s "component" -l "AIR-3,AIS-2,AIT-3" [-r "repo1,repo2"] [-v]
```

##### Options:

- `-m, --message MESSAGE`: Commit message (required)
- `-t, --type TYPE`: Commit type (default: feat)
- `-s, --scope SCOPE`: Commit scope (optional)
- `-l, --labels LABELS`: Comma-separated labels (required)
- `-r, --repos REPOSITORIES`: Comma-separated repository list (default: all)
- `-v, --validate`: Validate labels before committing
- `-d, --dry-run`: Show what would be committed without making changes
- `-h, --help`: Show help message

#### PowerShell Version (`batch_commit.ps1`)

Windows-compatible PowerShell version of the batch commit script.

##### Usage:

```powershell
./scripts/batch_commit.ps1 -Message "Commit message" -Type "feat" -Scope "component" -Labels "AIR-3,AIS-2,AIT-3" [-Repos "repo1,repo2"] [-Validate]
```

##### Parameters:

- `-Message "MESSAGE"`: Commit message (required)
- `-Type "TYPE"`: Commit type (default: feat)
- `-Scope "SCOPE"`: Commit scope (optional)
- `-Labels "LABELS"`: Comma-separated labels (required)
- `-Repos "REPOSITORIES"`: Comma-separated repository list (default: auto-detected)
- `-Validate`: Validate labels before committing
- `-DryRun`: Show what would be committed without making changes
- `-RootDir "PATH"`: Root directory containing all repositories (optional)
- `-Help`: Show help message

### 2. Label Synchronization Script

#### Python Version (`sync_labelling.py`)

This script synchronizes the AI_LABELLING.md and COMMIT_RULES.md files across all repositories to ensure consistent labeling standards.

##### Usage:

```bash
python scripts/sync_labelling.py [--source REPO] [--target REPOS] [--check-only] [--dry-run]
```

##### Options:

- `--source`: Source repository for label standards (default: anya-core)
- `--target`: Target repositories (comma-separated, default: all repositories)
- `--check-only`: Only check for differences without making changes
- `--dry-run`: Show what would be done without making actual changes
- `--no-commit`: Do not commit changes after synchronization
- `--batch-commit`: Use batch_commit.sh for committing changes

#### PowerShell Version (`sync_labelling.ps1`)

Windows-compatible PowerShell version of the synchronization script.

##### Usage:

```powershell
./scripts/sync_labelling.ps1 [-Source "REPO"] [-Target "REPOS"] [-CheckOnly] [-DryRun]
```

##### Parameters:

- `-Source "REPO"`: Source repository for label standards (default: anya-core)
- `-Target "REPOS"`: Target repositories (comma-separated, default: auto-detected)
- `-CheckOnly`: Only check for differences without making changes
- `-DryRun`: Show what would be done without making actual changes
- `-NoCommit`: Do not commit changes after synchronization
- `-BatchCommit`: Use batch_commit.ps1 for committing changes
- `-RootDir "PATH"`: Root directory containing all repositories (optional)
- `-Help`: Show help message

### 3. GitHub Actions Workflow

The `sync-labelling.yml` workflow automatically synchronizes labeling files whenever changes are made to `AI_LABELLING.md` or `COMMIT_RULES.md` in the main repository. It can also be manually triggered from the GitHub Actions tab.

## Cross-Platform Compatibility

The labeling system is designed to work seamlessly across different operating systems:

- **Linux/macOS**: Use the Bash scripts (`.sh` extension)
- **Windows**: Use the PowerShell scripts (`.ps1` extension)

Both versions provide identical functionality with platform-appropriate syntax and error handling.

## Quick Start for Windows Users

If you're having issues running the scripts on Windows, follow these simplified steps:

1. **Open PowerShell and navigate to the anya-core directory**:
   ```powershell
   cd C:\path\to\OPSource\anya-core
   ```

2. **Run the batch commit script directly from PowerShell**:
   ```powershell
   # Run script with full path specification
   .\scripts\batch_commit.ps1 -Message "Your commit message" -Type "feat" -Labels "AIR-3,AIS-2"
   
   # If running outside the repository:
   & "C:\path\to\OPSource\anya-core\scripts\batch_commit.ps1" -Message "Your commit message" -Type "feat" -Labels "AIR-3,AIS-2"
   ```

3. **Specify repositories explicitly if auto-detection fails**:
   ```powershell
   .\scripts\batch_commit.ps1 -Message "Update docs" -Type "docs" -Labels "AIR-3,AIS-2" -Repos "anya-core"
   ```

4. **For unusual repository structures, use the RootDir parameter**:
   ```powershell
   .\scripts\batch_commit.ps1 -Message "Fix bug" -Type "fix" -Labels "AIR-3,AIS-2" -RootDir "C:\Users\username\Projects"
   ```

5. **If you receive `-l` parameter errors**, you're likely using the wrong script format:
   ```powershell
   # Wrong (Bash format)
   .\scripts\batch_commit.sh -m "Message" -l "AIR-3,AIS-2"  # This won't work in PowerShell!
   
   # Correct (PowerShell format)
   .\scripts\batch_commit.ps1 -Message "Message" -Labels "AIR-3,AIS-2"  # Use this instead
   ```

### Windows-Specific Setup

When running the PowerShell scripts on Windows:

1. Ensure PowerShell execution policy allows running scripts:
   ```powershell
   Set-ExecutionPolicy -Scope CurrentUser RemoteSigned
   ```

2. Always run the scripts from the repository's root directory:
   ```powershell
   cd C:\path\to\anya-core
   ./scripts/batch_commit.ps1 -Help
   ```

3. If repositories are not in the expected location (i.e., not all in the same parent directory), use the `-RootDir` parameter:
   ```powershell
   ./scripts/batch_commit.ps1 -Message "Update" -Type "fix" -Labels "AIR-3,AIS-2" -RootDir "C:\Projects"
   ```

## Repository Structure

The scripts expect a specific repository structure:

```
parent-directory/
├── anya-core/
│   ├── .git/
│   ├── scripts/
│   │   ├── batch_commit.ps1
│   │   ├── batch_commit.sh
│   │   ├── sync_labelling.ps1
│   │   └── sync_labelling.py
│   └── AI_LABELLING.md
├── anya-bitcoin/
│   ├── .git/
│   └── AI_LABELLING.md
├── anya-web5/
│   ├── .git/
│   └── AI_LABELLING.md
└── ... other repositories
```

If your repositories are organized differently, use the `-RootDir` parameter to specify the parent directory containing all repositories.

## Labeling System Details

For detailed information about the labeling system itself, see [AI_LABELLING.md](../AI_LABELLING.md).

## Label Integration Process

### New Components

When adding new components to any repository:

1. **Assign Labels**: Determine appropriate Core and Extended category labels based on component type
2. **Document Labels**: Add labels to component headers and documentation
3. **Commit with Labels**: Use the batch commit script to commit with proper labels
4. **Verify Compliance**: Ensure the component meets the requirements for its assigned labels

### Updating Existing Components

When updating components:

1. **Review Labels**: Check if current labels are still appropriate
2. **Update Labels**: Modify labels if necessary based on changes
3. **Commit with Labels**: Use the batch commit script to commit with the updated labels
4. **Document Changes**: Update documentation if label changes affect component requirements

### Synchronizing Label Standards

To ensure consistent labeling across all repositories:

1. **Update Master File**: Make changes to `AI_LABELLING.md` in the anya-core repository
2. **Synchronize**: Run the synchronization script manually or let the GitHub Actions workflow handle it
3. **Verify**: Check that all repositories have the updated labeling standards
4. **Document**: Inform the team about any significant changes

## Development Workflow with Labels

### 1. Feature Development

When developing new features:

1. Create a feature branch from `develop`
2. Define appropriate labels for new components
3. Use batch commits with proper labels during development
4. Ensure all components meet their label requirements
5. Submit a pull request with comprehensive labels

### 2. Code Review

During code review, specifically check:

1. Label appropriateness and accuracy
2. Compliance with label requirements
3. Documentation of label meanings and implications
4. Consistent application of labels across components

### 3. Continuous Integration

CI/CD pipelines automatically:

1. Validate label syntax in commits
2. Verify component compliance with label requirements
3. Check consistency of labeling across repositories
4. Alert on label-related issues

## Troubleshooting

### Label Validation Failures

If label validation fails:

1. Check the component type and required labels in `AI_LABELLING.md`
2. Ensure all required labels are included in the commit
3. Verify the component meets the requirements for its labels
4. Fix any issues and try again

### Synchronization Issues

If label synchronization fails:

1. Check if you have proper permissions for all repositories
2. Ensure the source file exists and is valid
3. Look for conflicts or local changes in target repositories
4. Try running with `--dry-run` or `-DryRun` to diagnose issues

### Windows-Specific Issues

On Windows systems:

1. Make sure PowerShell execution policy allows running scripts (`Set-ExecutionPolicy RemoteSigned`)
2. Use the `.ps1` versions of the scripts
3. If encountering path issues, ensure proper repository structure or use the `-RootDir` parameter
4. For Git operations, verify that Git is in your PATH environment variable
5. If you receive errors with `-l` parameter, make sure you're using the PowerShell script (.ps1) not the Bash script (.sh)

### Common Errors and Solutions

| Error | Solution |
|-------|----------|
| `The term './scripts/batch_commit.sh' is not recognized...` | Use PowerShell script (`batch_commit.ps1`) on Windows |
| `The term '-l' is not recognized...` | Use PowerShell parameter format: `-Labels` instead of `-l` |
| Repository not found | Use `-RootDir` to specify the correct parent directory |
| Git not found | Install Git and ensure it's in the PATH environment variable |
| Failed to commit changes | Check Git error message and fix the issue (e.g., configuration problems) |

## Best Practices

1. **Label First**: Define appropriate labels before implementing new components
2. **Be Specific**: Use the most specific labels that apply to your component
3. **Document Requirements**: Clearly document how components meet label requirements
4. **Regular Audits**: Periodically audit repositories for label compliance
5. **Stay Updated**: Keep up with changes to the labeling system
6. **Cross-Platform Testing**: Test label operations on both Unix and Windows systems
7. **Use Auto-Detection**: Let the scripts auto-detect repositories where possible
8. **Check Results**: Always verify that commits were successful in the output summary

## Examples

### PowerShell Examples

```powershell
# Batch commit with labels - simple example
./scripts/batch_commit.ps1 -Message "Update ML models" -Type "feat" -Scope "ml" -Labels "AIR-3,AIS-2,AIT-3,AIM-2"

# Batch commit with validation and specific repositories
./scripts/batch_commit.ps1 -Message "Fix security issues" -Type "fix" -Scope "security" -Labels "AIR-3,AIS-3" -Repos "anya-core,anya-bitcoin" -Validate

# Batch commit with custom root directory
./scripts/batch_commit.ps1 -Message "Update configs" -Type "chore" -Labels "AIR-3,AIS-2" -RootDir "C:\Projects\Anya"

# Synchronize labels across all repositories
./scripts/sync_labelling.ps1

# Check for label differences only
./scripts/sync_labelling.ps1 -CheckOnly

# Synchronize labels to specific repositories
./scripts/sync_labelling.ps1 -Target "anya-web5,anya-bitcoin" -DryRun
```

### Bash Examples

```bash
# Batch commit with labels - simple example
./scripts/batch_commit.sh -m "Update ML models" -t "feat" -s "ml" -l "AIR-3,AIS-2,AIT-3,AIM-2"

# Batch commit with validation and specific repositories
./scripts/batch_commit.sh -m "Fix security issues" -t "fix" -s "security" -l "AIR-3,AIS-3" -r "anya-core,anya-bitcoin" -v

# Synchronize labels across all repositories
python scripts/sync_labelling.py

# Check for label differences only
python scripts/sync_labelling.py --check-only

# Synchronize labels to specific repositories
python scripts/sync_labelling.py --target "anya-web5,anya-bitcoin" --dry-run
```

## Additional Resources

- [AI Labeling Guide](../AI_LABELLING.md): Comprehensive AI labeling standards
- [Commit Rules](../COMMIT_RULES.md): Rules for creating commits with labels
- [Component Requirements](../docs/COMPONENT_REQUIREMENTS.md): Detailed requirements for each label level 