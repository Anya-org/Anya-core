# [AIR-3][AIS-3][BPC-3] Git Hooks Installation Script
# This script installs Git hooks for AI labeling validation

# Set script version
$VERSION = "1.0.0"

Write-Host "Git Hooks Installation Script v$VERSION" -ForegroundColor Green
Write-Host "This script installs Git hooks to validate AI labels" -ForegroundColor Green
Write-Host ""

# Check if .git directory exists
if (!(Test-Path ".git")) {
    Write-Error "No .git directory found. Make sure you're in the root of a Git repository."
    exit 1
}

# Create hooks directory if it doesn't exist
if (!(Test-Path ".git\hooks")) {
    New-Item -ItemType Directory -Path ".git\hooks"
}

# Create pre-commit hook
$preCommitPath = ".git\hooks\pre-commit"
$preCommitContent = @"
#!/bin/sh
# Pre-commit hook for AI labeling validation

# Check for PowerShell
if command -v pwsh >/dev/null 2>&1; then
    POWERSHELL_CMD=pwsh
elif command -v powershell >/dev/null 2>&1; then
    POWERSHELL_CMD=powershell
else
    echo "PowerShell not found. Please install PowerShell to use this hook."
    exit 1
fi

# Run validation on staged files only
echo "Running AI label validation on staged files..."
STAGED_FILES=\$(git diff --cached --name-only --diff-filter=ACMR)

if [ -z "\$STAGED_FILES" ]; then
    echo "No staged files found."
    exit 0
fi

# Create temp file for staged files
TEMP_FILE=\$(mktemp)
echo \$STAGED_FILES > \$TEMP_FILE

# Run validation script
\$POWERSHELL_CMD -ExecutionPolicy Bypass -File scripts/validate_ai_labels.ps1 -stagedFiles \$TEMP_FILE

RESULT=\$?

# Clean up temp file
rm \$TEMP_FILE

if [ \$RESULT -ne 0 ]; then
    echo ""
    echo "AI label validation failed. Please fix the issues before committing."
    echo "You can run scripts/validate_ai_labels.ps1 -fix to automatically fix issues."
    exit 1
fi

exit 0
"@

# Write pre-commit hook
Set-Content -Path $preCommitPath -Value $preCommitContent

# Make pre-commit hook executable (on Unix-like systems)
if ($IsLinux -or $IsMacOS) {
    & chmod +x $preCommitPath
}

# Create commit-msg hook for commit message validation
$commitMsgPath = ".git\hooks\commit-msg"
$commitMsgContent = @"
#!/bin/sh
# Commit message hook for AI labeling validation

# Check for PowerShell
if command -v pwsh >/dev/null 2>&1; then
    POWERSHELL_CMD=pwsh
elif command -v powershell >/dev/null 2>&1; then
    POWERSHELL_CMD=powershell
else
    echo "PowerShell not found. Please install PowerShell to use this hook."
    exit 1
fi

# Run validation on commit message
\$POWERSHELL_CMD -ExecutionPolicy Bypass -File scripts/validate_commit_message.ps1 \$1

RESULT=\$?

if [ \$RESULT -ne 0 ]; then
    echo ""
    echo "Commit message validation failed. Please fix the issues."
    echo "Format should include AI labels: [AIR-3][AIS-3][BPC-3] Your message"
    exit 1
fi

exit 0
"@

# Write commit-msg hook
Set-Content -Path $commitMsgPath -Value $commitMsgContent

# Make commit-msg hook executable (on Unix-like systems)
if ($IsLinux -or $IsMacOS) {
    & chmod +x $commitMsgPath
}

# Create the validate commit message script
$validateCommitMsgPath = "scripts\validate_commit_message.ps1"
$validateCommitMsgContent = @"
# [AIR-3][AIS-3][BPC-3] Commit Message Validation Script
# This script validates that commit messages follow the AI labeling format

param (
    [string]$commitMsgFile
)

# Valid labels and their levels
\$VALID_CATEGORIES = @(
    # Core categories
    "AIR", "AIS", "AIT", "AIM", "AIP", "AIE",
    # Extended categories
    "BPC", "RES", "SCL", "PFM", "DAO", "DID", "W5C", "UXA"
)

\$MAX_LEVEL = 3  # 0-3 scale

# Skip validation for merge commits
\$commitMsg = Get-Content -Path \$commitMsgFile -Raw
if (\$commitMsg -match "^Merge ") {
    exit 0
}

# Check for AI labels in commit message
\$labelPattern = '\[([A-Z]{2,5})-([0-9])\]'
\$matches = [regex]::Matches(\$commitMsg, \$labelPattern)

if (\$matches.Count -eq 0) {
    Write-Host "Error: Commit message must include AI labels." -ForegroundColor Red
    Write-Host "Example: [AIR-3][AIS-3][BPC-3] Implement secure SPV verification" -ForegroundColor Yellow
    exit 1
}

\$hasError = \$false

foreach (\$match in \$matches) {
    if (\$match.Value -match '\[([A-Z]{2,5})-([0-9])\]') {
        \$category = \$matches[1]
        \$level = [int]\$matches[2]
        
        # Check if category is valid
        if (-not (\$VALID_CATEGORIES -contains \$category)) {
            Write-Host "Error: Invalid category '\$category' in label '\$(\$match.Value)'." -ForegroundColor Red
            \$hasError = \$true
        }
        
        # Check if level is valid
        if (\$level -lt 0 -or \$level -gt \$MAX_LEVEL) {
            Write-Host "Error: Invalid level '\$level' in label '\$(\$match.Value)'. Level must be 0-\$MAX_LEVEL." -ForegroundColor Red
            \$hasError = \$true
        }
    }
}

if (\$hasError) {
    exit 1
}

exit 0
"@

# Write validate commit message script
Set-Content -Path $validateCommitMsgPath -Value $validateCommitMsgContent

Write-Host "Git hooks installed successfully:" -ForegroundColor Green
Write-Host "- Pre-commit hook: $preCommitPath" -ForegroundColor Green
Write-Host "- Commit-msg hook: $commitMsgPath" -ForegroundColor Green
Write-Host "- Validation script: $validateCommitMsgPath" -ForegroundColor Green
Write-Host ""
Write-Host "These hooks will validate AI labels in your code and commit messages." -ForegroundColor Green
Write-Host "To validate your code manually, run: ./scripts/validate_ai_labels.ps1" -ForegroundColor Green 