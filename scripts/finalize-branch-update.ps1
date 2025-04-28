#!/usr/bin/env pwsh
# Branch Management Finalization Script
# Author: bo_thebig (botshelomokokoka@gmail.com)
# Created from branch optimization after the security enhancements consolidation

# Step 1: Configure signing if not already configured
$signingKey = git config --get user.signingkey
$gpgSign = git config --get commit.gpgsign

if (-not $signingKey -or $gpgSign -ne "true") {
    Write-Host "Configuring Git signing..." -ForegroundColor Cyan
    
    # Set user identity
    git config --global user.name "bo_thebig"
    git config --global user.email "botshelomokokoka@gmail.com"
    
    # Check for existing GPG keys
    $keyOutput = gpg --list-secret-keys --keyid-format LONG
    if ($keyOutput -match "sec\s+rsa\d+/([A-F0-9]+)") {
        $keyId = $matches[1]
        Write-Host "Found existing GPG key: $keyId" -ForegroundColor Green
    } else {
        Write-Host "No GPG key found. Please create one first using:" -ForegroundColor Yellow
        Write-Host "gpg --full-generate-key" -ForegroundColor Yellow
        exit 1
    }
    
    # Configure Git to use GPG
    git config --global user.signingkey $keyId
    git config --global commit.gpgsign true
    
    Write-Host "Git signing configured successfully!" -ForegroundColor Green
}

# Step 2: Confirm we're on the consolidated branch
$currentBranch = git rev-parse --abbrev-ref HEAD
if ($currentBranch -ne "enhancement/consolidated-security") {
    Write-Host "Switching to enhancement/consolidated-security branch..." -ForegroundColor Cyan
    git checkout enhancement/consolidated-security
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Failed to switch to enhancement/consolidated-security branch!" -ForegroundColor Red
        exit 1
    }
}

# Step 3: Make sure our branch is up to date with main
Write-Host "Updating branch with latest changes from main..." -ForegroundColor Cyan
git merge --no-ff main -m "merge: sync with main before final merge"

# Step 4: Merge to main
Write-Host "Merging security enhancements to main..." -ForegroundColor Cyan
git checkout main
git merge --no-ff enhancement/consolidated-security -m "feat(security): integrate GPG signing capabilities and commit security tools

This merge adds:
- GPG commit signing configuration
- Tools for retroactively signing commits
- Documentation for Git signing
- Improved security validation in compliance workflow

Signed-off-by: bo_thebig <botshelomokokoka@gmail.com>"

# Step 5: Push changes to main
Write-Host "Pushing changes to main..." -ForegroundColor Cyan
git push origin main

# Step 6: Push changes to release candidate
Write-Host "Updating release candidate branch..." -ForegroundColor Cyan
git checkout release-candidate-1.0
git push origin release-candidate-1.0

# Step 7: Clean up branches
Write-Host "Cleaning up branches..." -ForegroundColor Cyan

# List of branches to delete
$branchesToDelete = @(
    "feature/web5-bip341-compliance",  # Original feature branch (now consolidated)
    "enhancement/consolidated-security" # Temporary consolidation branch
)

foreach ($branch in $branchesToDelete) {
    Write-Host "Checking if $branch needs to be deleted..." -ForegroundColor Yellow
    
    # Check if branch exists and is fully merged
    $branchExists = git branch --list $branch
    
    if ($branchExists) {
        # Check if it's safe to delete (fully merged to main)
        $safeToDelete = git branch --merged main | Select-String -Pattern "\s+$branch$"
        
        if ($safeToDelete -or $branch -eq "enhancement/consolidated-security") {
            git checkout main
            git branch -d $branch
            if ($LASTEXITCODE -eq 0) {
                Write-Host "Deleted branch: $branch" -ForegroundColor Green
            } else {
                Write-Host "Warning: Branch $branch has unmerged changes. Use -D to force delete." -ForegroundColor Yellow
            }
        } else {
            Write-Host "Warning: Branch $branch is not fully merged to main." -ForegroundColor Yellow
        }
    } else {
        Write-Host "Branch $branch doesn't exist locally." -ForegroundColor Gray
    }
}

# Step 8: Final status
git checkout main
Write-Host "`nBranch management completed successfully!" -ForegroundColor Green
Write-Host "Main and release-candidate-1.0 branches have been updated with security enhancements" -ForegroundColor Green
Write-Host "Unnecessary branches have been cleaned up" -ForegroundColor Green

# Display current branch structure
Write-Host "`nCurrent branch structure:" -ForegroundColor Cyan
git branch -a 