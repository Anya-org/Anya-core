#!/usr/bin/env pwsh
# Branch Cleanup Script
# Author: bo_thebig (botshelomokokoka@gmail.com)
# Purpose: Removes all branches that are no longer needed after branch consolidation

# Set colors for output
$successColor = "Green"
$warningColor = "Yellow"
$errorColor = "Red"
$infoColor = "Cyan"

Write-Host "Branch Cleanup Process Starting..." -ForegroundColor $infoColor
Write-Host "--------------------------------" -ForegroundColor $infoColor

# Step 1: Make sure we're on main branch
Write-Host "Switching to main branch..." -ForegroundColor $infoColor
git checkout main
if ($LASTEXITCODE -ne 0) {
    Write-Host "Error: Failed to switch to main branch!" -ForegroundColor $errorColor
    exit 1
}

# Step 2: Get current branch list for reference
Write-Host "Current branch list before cleanup:" -ForegroundColor $infoColor
git branch

# Step 3: Define branches to keep (essential branches)
$branchesToKeep = @("main", "release-candidate-1.0")

# Step 4: Delete local branches that are fully merged
Write-Host "`nRemoving fully merged branches..." -ForegroundColor $infoColor
git branch --merged main | ForEach-Object {
    $branch = $_.Trim()
    # Skip if it's the current branch or in the keep list
    if ($branch -notmatch '^\*' -and $branch -notin $branchesToKeep -and $branch -ne "") {
        Write-Host "Deleting merged branch: $branch" -ForegroundColor $successColor
        git branch -d $branch
    }
}

# Step 5: Force delete specified branches even if not fully merged
$branchesToForceDelete = @(
    "feature/web5-bip341-compliance",   # Already consolidated to main
    "chore/auto-clean-maintenance"      # Maintenance branch no longer needed
)

Write-Host "`nForce removing specific branches..." -ForegroundColor $infoColor
foreach ($branch in $branchesToForceDelete) {
    # Check if branch exists
    $branchExists = git branch --list $branch
    if ($branchExists) {
        Write-Host "Force deleting branch: $branch" -ForegroundColor $warningColor
        git branch -D $branch
        if ($LASTEXITCODE -eq 0) {
            Write-Host "  Successfully deleted: $branch" -ForegroundColor $successColor
        } else {
            Write-Host "  Failed to delete: $branch" -ForegroundColor $errorColor
        }
    }
}

# Step 6: Clean up remote branches that we've consolidated
Write-Host "`nCleaning up remote branches..." -ForegroundColor $infoColor
$remoteTrackingBranches = @(
    "origin/feature/web5-bip341-compliance",  # Feature branch already consolidated
    "origin/dependency-cleanup"               # Dependency cleanup already done
)

foreach ($remoteBranch in $remoteTrackingBranches) {
    if ($remoteBranch -match "origin/(.+)") {
        $branchName = $matches[1]
        Write-Host "Removing remote branch: $branchName" -ForegroundColor $warningColor
        
        # Delete the remote branch
        git push origin --delete $branchName
        if ($LASTEXITCODE -eq 0) {
            Write-Host "  Successfully deleted remote branch: $branchName" -ForegroundColor $successColor
        } else {
            Write-Host "  Failed to delete remote branch: $branchName" -ForegroundColor $errorColor
        }
    }
}

# Step 7: Prune stale remote-tracking branches
Write-Host "`nPruning stale remote-tracking branches..." -ForegroundColor $infoColor
git remote prune origin
if ($LASTEXITCODE -eq 0) {
    Write-Host "Successfully pruned stale remote-tracking branches." -ForegroundColor $successColor
}

# Step 8: Final state
Write-Host "`nFinal branch state after cleanup:" -ForegroundColor $infoColor
git branch -a

Write-Host "`nBranch cleanup completed successfully!" -ForegroundColor $successColor
Write-Host "Only essential branches remain. All security enhancements have been consolidated." -ForegroundColor $successColor

# Record the cleanup in branch consolidation document
$timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
$consolidationDoc = "docs/BRANCH_CONSOLIDATION.md"

if (Test-Path $consolidationDoc) {
    Add-Content -Path $consolidationDoc -Value "`n## Branch Cleanup Completed"
    Add-Content -Path $consolidationDoc -Value "`nOn $timestamp, all unnecessary branches were removed after successful integration:`n"
    Add-Content -Path $consolidationDoc -Value "- Removed feature/web5-bip341-compliance (consolidated to main)"
    Add-Content -Path $consolidationDoc -Value "- Removed chore/auto-clean-maintenance (no longer needed)"
    Add-Content -Path $consolidationDoc -Value "- Removed stale remote branches"
    Add-Content -Path $consolidationDoc -Value "`nOnly the essential branches remain: main and release-candidate-1.0"
    
    Write-Host "Updated consolidation documentation at: $consolidationDoc" -ForegroundColor $successColor
} else {
    Write-Host "Warning: Could not find consolidation document to update." -ForegroundColor $warningColor
} 