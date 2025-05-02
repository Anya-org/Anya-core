#!/usr/bin/env pwsh
# fix_all_branches.ps1
# [AIR-3][AIS-3][BPC-3] Automatic branch fix and PR preparation script

$ErrorActionPreference = "Stop"
$currentBranch = git rev-parse --abbrev-ref HEAD

Write-Host "🛠️ Automatic Branch Fix & PR Preparation Tool" -ForegroundColor Cyan
Write-Host "================================================" -ForegroundColor Cyan
Write-Host "Current branch: $currentBranch" -ForegroundColor Yellow
Write-Host ""

# Define branches to process
$branches = @(
    "feature/bitcoin-core",
    "feature/bitcoin-hexagonal-architecture", 
    "feature/bitcoin-implementation",
    "feature/bitcoin-layer2",
    "feature/bitcoin-testing"
)

function Fix-Branch {
    param (
        [string]$branch
    )
    
    Write-Host "🔄 Processing branch: $branch" -ForegroundColor Green
    
    # Checkout the branch
    Write-Host "  ↪ Checking out branch..." -ForegroundColor Yellow
    git checkout $branch
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ Failed to checkout branch $branch" -ForegroundColor Red
        return $false
    }
    
    # Commit all changes with proper AI labeling
    $hasChanges = (git status --porcelain).Length -gt 0
    if ($hasChanges) {
        Write-Host "  ↪ Found uncommitted changes, committing..." -ForegroundColor Yellow
        git add .
        git commit -m "[AIR-3][AIS-3][BPC-3] Comprehensive compliance and linting fixes"
        if ($LASTEXITCODE -ne 0) {
            Write-Host "❌ Failed to commit changes on branch $branch" -ForegroundColor Red
            return $false
        }
    } else {
        Write-Host "  ↪ No uncommitted changes found" -ForegroundColor Yellow
    }
    
    # Run cargo fmt to fix formatting issues (if cargo is available)
    if (Get-Command cargo -ErrorAction SilentlyContinue) {
        Write-Host "  ↪ Running cargo fmt to fix formatting..." -ForegroundColor Yellow
        cargo fmt
        
        # Check if formatting made changes
        $hasLintChanges = (git status --porcelain).Length -gt 0
        if ($hasLintChanges) {
            git add .
            git commit -m "[AIR-3][AIS-3][BPC-3] Fix formatting issues with cargo fmt"
        }
    } else {
        Write-Host "  ↪ Cargo not found, skipping automatic formatting" -ForegroundColor Yellow
    }
    
    # Check for unpushed commits
    $unpushedCommits = git log --oneline origin/$branch..HEAD
    if ($unpushedCommits) {
        Write-Host "  ↪ Found unpushed commits, pushing to remote..." -ForegroundColor Yellow
        git push origin $branch
        if ($LASTEXITCODE -ne 0) {
            Write-Host "❌ Failed to push changes on branch $branch" -ForegroundColor Red
            return $false
        }
    } else {
        Write-Host "  ↪ No unpushed commits found" -ForegroundColor Yellow
    }
    
    Write-Host "✅ Successfully processed branch $branch" -ForegroundColor Green
    return $true
}

function Fix-CommitMessages {
    param (
        [string]$branch
    )
    
    Write-Host "🔄 Preparing to fix commit messages on branch: $branch" -ForegroundColor Green
    
    # Get list of commits in the past N commits that don't have AI labeling
    $numCommits = 50 # Check last 50 commits
    $nonCompliantCommits = git log -n $numCommits --pretty=format:"%h %s" | Where-Object { $_ -notmatch "\[AIR-\d+\]\[AIS-\d+\]\[BPC-\d+\]" }
    
    if (-not $nonCompliantCommits) {
        Write-Host "  ↪ No non-compliant commit messages found in the last $numCommits commits." -ForegroundColor Yellow
        return
    }
    
    Write-Host "  ↪ Found non-compliant commit messages." -ForegroundColor Yellow
    Write-Host $nonCompliantCommits
    
    # Ask if user wants to automatically fix commit messages
    $confirm = Read-Host "Do you want to automatically fix these commit messages? (y/n)"
    if ($confirm -ne "y") {
        Write-Host "  ↪ Skipping commit message fixes" -ForegroundColor Yellow
        return
    }
    
    # We'll use git filter-branch to rewrite commit messages
    foreach ($commitLine in $nonCompliantCommits) {
        $parts = $commitLine -split " ", 2
        $hash = $parts[0]
        $message = $parts[1]
        
        # Only fix if the message doesn't already have AI labeling
        if ($message -notmatch "\[AIR-\d+\]\[AIS-\d+\]\[BPC-\d+\]") {
            $newMessage = "[AIR-3][AIS-3][BPC-3] $message"
            Write-Host "  ↪ Rewriting: $hash '$message' to '$newMessage'" -ForegroundColor Yellow
            
            # Use git filter-branch to rewrite the commit message
            $filter = "if [ `"`$GIT_COMMIT`" = `"$hash`" ]; then echo `"$newMessage`"; else cat; fi"
            git filter-branch -f --msg-filter "$filter" $hash^..$hash
        }
    }
    
    # Force push the changes (if needed)
    $forcePush = Read-Host "Commit messages have been rewritten. Force push to remote? (y/n)"
    if ($forcePush -eq "y") {
        git push -f origin $branch
    }
}

$results = @{}

# Process each branch
foreach ($branch in $branches) {
    try {
        $success = Fix-Branch -branch $branch
        if ($success) {
            # Ask if user wants to fix commit messages (this is more disruptive so make it optional)
            $fixMessages = Read-Host "Do you want to fix non-compliant commit messages on $branch? This rewrites history and requires force push. (y/n)"
            if ($fixMessages -eq "y") {
                Fix-CommitMessages -branch $branch
            }
        }
        $results[$branch] = $success
    }
    catch {
        Write-Host "❌ Error processing branch $branch : $_" -ForegroundColor Red
        $results[$branch] = $false
    }
}

# Return to original branch
git checkout $currentBranch

# Print summary
Write-Host "`n📊 Branch Processing Summary" -ForegroundColor Cyan
Write-Host "===============================" -ForegroundColor Cyan
foreach ($branch in $branches) {
    $status = if ($results[$branch]) { "✅ Success" } else { "❌ Failed" }
    Write-Host "$branch : $status" -ForegroundColor $(if ($results[$branch]) { "Green" } else { "Red" })
}

# Re-run review script to see if issues were fixed
Write-Host "`n🔍 Re-running branch review script to check results" -ForegroundColor Cyan
& "$PSScriptRoot\scripts\review_all_branches.ps1"

# Hint about merge script
Write-Host "`n🚀 Next steps:" -ForegroundColor Cyan
Write-Host "For branches that are now ready, you can run: .\scripts\bitcoin\merge_pr.ps1" -ForegroundColor Yellow 