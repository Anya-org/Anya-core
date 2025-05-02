#!/usr/bin/env pwsh
# fix_merge_conflicts.ps1
# [AIR-3][AIS-3][BPC-3] Merge conflict resolution script

$ErrorActionPreference = "Stop"

Write-Host "🔄 Bitcoin Merge Conflict Resolution Tool" -ForegroundColor Cyan
Write-Host "=========================================" -ForegroundColor Cyan
Write-Host ""

# Get current branch
$currentBranch = git rev-parse --abbrev-ref HEAD
Write-Host "Current branch: $currentBranch" -ForegroundColor Yellow

# First, handle the current branch's merge conflicts
function Resolve-CurrentBranchConflicts {
    # Check if we're in the middle of a merge with conflicts
    $mergeInProgress = Test-Path ".git/MERGE_HEAD"
    
    if (-not $mergeInProgress) {
        Write-Host "❌ No merge conflicts detected on current branch" -ForegroundColor Red
        return $false
    }
    
    Write-Host "Found merge conflicts on branch $currentBranch" -ForegroundColor Yellow
    
    # Get list of files with merge conflicts
    $conflictFiles = git diff --name-only --diff-filter=U
    Write-Host "Files with conflicts:" -ForegroundColor Yellow
    $conflictFiles | ForEach-Object { Write-Host "  - $_" -ForegroundColor Yellow }
    
    # Ask if user wants to abort the current merge
    $abort = Read-Host "Do you want to abort the current merge and start fresh? (y/n)"
    if ($abort -eq "y") {
        git merge --abort
        Write-Host "✅ Merge aborted successfully" -ForegroundColor Green
        return $true
    }
    
    # For each file with conflicts, offer options
    foreach ($file in $conflictFiles) {
        Write-Host "Resolving conflicts in: $file" -ForegroundColor Cyan
        
        $choice = Read-Host "How to resolve? (o = ours, t = theirs, m = manual, s = skip)"
        
        switch ($choice) {
            "o" {
                git checkout --ours $file
                git add $file
                Write-Host "  ↪ Used OUR version for $file" -ForegroundColor Green
            }
            "t" {
                git checkout --theirs $file
                git add $file
                Write-Host "  ↪ Used THEIR version for $file" -ForegroundColor Green
            }
            "m" {
                Write-Host "  ↪ Open the file in your editor and resolve conflicts manually." -ForegroundColor Yellow
                Write-Host "  ↪ Once done, run 'git add $file' and continue this script." -ForegroundColor Yellow
                $continue = Read-Host "Press Enter when you've resolved the conflicts for this file"
            }
            "s" {
                Write-Host "  ↪ Skipping $file" -ForegroundColor Yellow
            }
            default {
                Write-Host "  ↪ Invalid choice, skipping $file" -ForegroundColor Red
            }
        }
    }
    
    # Check if all conflicts are resolved
    $remainingConflicts = git diff --name-only --diff-filter=U
    if ($remainingConflicts) {
        Write-Host "❌ There are still unresolved conflicts:" -ForegroundColor Red
        $remainingConflicts | ForEach-Object { Write-Host "  - $_" -ForegroundColor Red }
        
        $force = Read-Host "Do you want to commit anyway? (y/n)"
        if ($force -ne "y") {
            Write-Host "Exiting without completing the merge." -ForegroundColor Yellow
            return $false
        }
    }
    
    # Commit the merge
    $commitMsg = "[AIR-3][AIS-3][BPC-3] Merge conflict resolution for hexagonal architecture"
    git commit -m $commitMsg
    
    Write-Host "✅ Merge conflicts resolved and committed successfully" -ForegroundColor Green
    return $true
}

function Clean-Branch {
    param (
        [string]$branch
    )
    
    # Check if branch exists
    $branchExists = git show-ref --verify --quiet refs/heads/$branch
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ Branch $branch does not exist" -ForegroundColor Red
        return $false
    }
    
    # Checkout the branch
    Write-Host "Checking out branch $branch..." -ForegroundColor Yellow
    git checkout $branch
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ Failed to checkout branch $branch" -ForegroundColor Red
        return $false
    }
    
    # Reset any uncommitted changes
    $hasChanges = (git status --porcelain).Length -gt 0
    if ($hasChanges) {
        $reset = Read-Host "Branch $branch has uncommitted changes. Reset them? (y/n)"
        if ($reset -eq "y") {
            git reset --hard HEAD
            Write-Host "  ↪ Reset uncommitted changes" -ForegroundColor Yellow
        }
    }
    
    # Pull latest changes
    Write-Host "Pulling latest changes for $branch..." -ForegroundColor Yellow
    git pull origin $branch
    
    Write-Host "✅ Branch $branch is now clean" -ForegroundColor Green
    return $true
}

function Merge-BitcoinBranches {
    # Define the branch hierarchy for merging
    $branchHierarchy = @(
        "feature/bitcoin-core",
        "feature/bitcoin-implementation",
        "feature/bitcoin-layer2",
        "feature/bitcoin-testing",
        "feature/bitcoin-hexagonal-architecture"
    )
    
    # Clean all branches first
    foreach ($branch in $branchHierarchy) {
        $success = Clean-Branch -branch $branch
        if (-not $success) {
            Write-Host "❌ Failed to clean branch $branch, skipping merge process" -ForegroundColor Red
            return $false
        }
    }
    
    # Start merging from the bottom up
    for ($i = 0; $i -lt $branchHierarchy.Count - 1; $i++) {
        $sourceBranch = $branchHierarchy[$i]
        $targetBranch = $branchHierarchy[$i + 1]
        
        Write-Host "Merging $sourceBranch into $targetBranch..." -ForegroundColor Cyan
        
        # Checkout target branch
        git checkout $targetBranch
        
        # Try to merge
        git merge --no-ff $sourceBranch -m "[AIR-3][AIS-3][BPC-3] Merge $sourceBranch into $targetBranch"
        
        # Check if merge succeeded
        if ($LASTEXITCODE -ne 0) {
            Write-Host "⚠️ Merge conflicts detected, please resolve them manually" -ForegroundColor Yellow
            
            # Resolve conflicts
            $resolved = Resolve-CurrentBranchConflicts
            if (-not $resolved) {
                Write-Host "❌ Failed to resolve conflicts, aborting merge process" -ForegroundColor Red
                return $false
            }
        }
        
        # Push changes
        git push origin $targetBranch
        
        Write-Host "✅ Successfully merged $sourceBranch into $targetBranch" -ForegroundColor Green
    }
    
    return $true
}

# Main execution

# First, deal with any current merge conflicts
if (Test-Path ".git/MERGE_HEAD") {
    Write-Host "Found existing merge in progress, resolving conflicts first..." -ForegroundColor Yellow
    $resolved = Resolve-CurrentBranchConflicts
    if (-not $resolved) {
        Write-Host "❌ Failed to resolve existing merge conflicts. Please fix manually and try again." -ForegroundColor Red
        exit 1
    }
}

# Ask if the user wants to perform automatic merging of all branches
$autoMerge = Read-Host "Do you want to automatically merge all Bitcoin feature branches? (y/n)"
if ($autoMerge -eq "y") {
    $success = Merge-BitcoinBranches
    if ($success) {
        Write-Host "✅ Successfully merged all Bitcoin feature branches" -ForegroundColor Green
    }
    else {
        Write-Host "❌ Failed to complete the branch merging process" -ForegroundColor Red
    }
}

# Return to original branch
git checkout $currentBranch

Write-Host "`n🚀 Next steps:" -ForegroundColor Cyan
Write-Host "1. Run .\fix_all_branches.ps1 to ensure all branches are compliant" -ForegroundColor Yellow
Write-Host "2. Run .\scripts\review_all_branches.ps1 to verify compliance" -ForegroundColor Yellow
Write-Host "3. Run .\scripts\bitcoin\merge_pr.ps1 to create PRs for compliant branches" -ForegroundColor Yellow 