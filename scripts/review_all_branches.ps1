# Review All Branches Script
# [AIR-3][AIS-3][BPC-3][AIT-3][RES-3]
# This script reviews all feature branches and prepares PRs

Write-Host "Branch Review and PR Preparation Tool" -ForegroundColor Cyan
Write-Host "-----------------------------------" -ForegroundColor Cyan
Write-Host ""

# Setup
$ErrorActionPreference = "Stop"
$rootDir = $PSScriptRoot | Split-Path
Set-Location $rootDir

function Write-Step {
    param (
        [string]$step,
        [string]$description,
        [string]$color = "White"
    )
    
    Write-Host "[$step] " -ForegroundColor Cyan -NoNewline
    Write-Host $description -ForegroundColor $color
}

function Get-FeatureBranches {
    $branches = & git branch | Where-Object { $_ -match "\s+feature/" } | ForEach-Object { $_.Trim() }
    return $branches
}

function Get-BranchStatus {
    param (
        [string]$branch
    )
    
    # Get current branch to restore it later
    $currentBranch = & git rev-parse --abbrev-ref HEAD
    
    # Checkout the branch
    Write-Step "Info" "Checking out branch $branch..." "Yellow"
    & git checkout $branch 2>&1 | Out-Null
    
    # Pull latest changes
    Write-Step "Info" "Pulling latest changes for $branch..." "Yellow"
    & git pull 2>&1 | Out-Null
    
    # Check if branch has unpushed commits
    $unpushedCommits = & git log --oneline origin/$branch..HEAD 2>&1
    $hasUnpushedCommits = $unpushedCommits -ne $null -and $unpushedCommits.Count -gt 0
    
    # Check if branch has uncommitted changes
    $uncommittedChanges = & git status --porcelain 2>&1
    $hasUncommittedChanges = $uncommittedChanges -ne $null -and $uncommittedChanges.Count -gt 0
    
    # Get commit count and latest commit
    $commitCount = (& git rev-list --count HEAD) -as [int]
    $latestCommit = & git log -1 --pretty=format:"%h - %s" 2>&1
    
    # Check if there are existing PRs
    $prUrl = "https://github.com/Anya-org/Anya-core/pulls?q=is%3Apr+head%3A$branch"
    
    # Return status object
    $status = [PSCustomObject]@{
        Branch = $branch
        CommitCount = $commitCount
        LatestCommit = $latestCommit
        HasUnpushedCommits = $hasUnpushedCommits
        HasUncommittedChanges = $hasUncommittedChanges
        PRUrl = $prUrl
    }
    
    # Restore original branch
    & git checkout $currentBranch 2>&1 | Out-Null
    
    return $status
}

function Check-BranchCompliance {
    param (
        [string]$branch
    )
    
    # Get current branch to restore it later
    $currentBranch = & git rev-parse --abbrev-ref HEAD
    
    # Checkout the branch
    Write-Step "Info" "Checking out branch $branch for compliance check..." "Yellow"
    & git checkout $branch 2>&1 | Out-Null
    
    # Check for AI labeling compliance
    $recentCommits = & git log -n 10 --pretty=format:"%s" 2>&1
    $complianceIssues = @()
    
    foreach ($commit in $recentCommits) {
        if (-not ($commit -match "\[AIR-\d+\]\[AIS-\d+\]\[BPC-\d+\]")) {
            $complianceIssues += $commit
        }
    }
    
    # Check for hexagonal architecture compliance (for Bitcoin branches)
    $architectureCompliance = $true
    if ($branch -match "bitcoin") {
        $interfaceCount = (Get-ChildItem -Path "src/bitcoin/interface" -Filter "*.rs" -Recurse -ErrorAction SilentlyContinue).Count
        if ($interfaceCount -eq 0) {
            $architectureCompliance = $false
        }
    }
    
    # Check for documentation
    $documentationCompliance = $true
    if ($branch -match "bitcoin") {
        $docsCount = (Get-ChildItem -Path "docs/bitcoin" -Filter "*.md" -Recurse -ErrorAction SilentlyContinue).Count
        if ($docsCount -lt 3) {
            $documentationCompliance = $false
        }
    }
    
    # Return compliance object
    $compliance = [PSCustomObject]@{
        Branch = $branch
        CommitCompliance = $complianceIssues.Count -eq 0
        ArchitectureCompliance = $architectureCompliance
        DocumentationCompliance = $documentationCompliance
        NonCompliantCommits = $complianceIssues
    }
    
    # Restore original branch
    & git checkout $currentBranch 2>&1 | Out-Null
    
    return $compliance
}

function Run-PRChecks {
    param (
        [string]$branch
    )
    
    # Get current branch to restore it later
    $currentBranch = & git rev-parse --abbrev-ref HEAD
    
    # Checkout the branch
    Write-Step "Info" "Checking out branch $branch for PR checks..." "Yellow"
    & git checkout $branch 2>&1 | Out-Null
    
    $checkResults = @{}
    
    # Run basic checks
    Write-Step "Check" "Running basic checks for $branch..." "Yellow"
    
    # Check for linting issues
    try {
        $lintOutput = & git diff --check 2>&1
        $checkResults["Linting"] = $lintOutput -eq $null -or $lintOutput.Count -eq 0
    } catch {
        $checkResults["Linting"] = $false
    }
    
    # Run specific checks for Bitcoin branches
    if ($branch -match "bitcoin") {
        Write-Step "Check" "Running Bitcoin-specific checks for $branch..." "Yellow"
        
        # Check for BIP implementation if it's a bitcoin branch
        $hasBipImplementation = Test-Path -Path "core/src/bip"
        $checkResults["BIP Implementation"] = $hasBipImplementation
        
        # Check for interface layer if it's a bitcoin branch
        $hasInterfaceLayer = Test-Path -Path "src/bitcoin/interface"
        $checkResults["Interface Layer"] = $hasInterfaceLayer
        
        # Check for proper documentation
        $hasBipDocumentation = Test-Path -Path "docs/bitcoin/BIP_IMPLEMENTATION_INDEX.md"
        $checkResults["BIP Documentation"] = $hasBipDocumentation
    }
    
    # Restore original branch
    & git checkout $currentBranch 2>&1 | Out-Null
    
    return $checkResults
}

function Prepare-PR {
    param (
        [string]$branch,
        [PSCustomObject]$status,
        [PSCustomObject]$compliance,
        [hashtable]$checkResults
    )
    
    # Determine target branch based on naming convention
    $targetBranch = "dev"
    if ($branch -match "feature/bitcoin-") {
        $targetBranch = "feature/bitcoin-implementation"
    } elseif ($branch -match "feature/web5-") {
        $targetBranch = "feature/web5-implementation"
    } elseif ($branch -match "feature/layer2-") {
        $targetBranch = "feature/layer2-implementation"
    }
    
    # Create PR info object
    $prInfo = [PSCustomObject]@{
        SourceBranch = $branch
        TargetBranch = $targetBranch
        Title = "[AIR-3][AIS-3][BPC-3] PR from $branch to $targetBranch"
        ReadyForPR = $compliance.CommitCompliance -and -not $status.HasUncommittedChanges -and -not $status.HasUnpushedCommits
        FailedChecks = @($checkResults.GetEnumerator() | Where-Object { $_.Value -eq $false } | ForEach-Object { $_.Key })
        ActionItems = @()
    }
    
    # Add action items based on checks
    if (-not $compliance.CommitCompliance) {
        $prInfo.ActionItems += "Fix commit messages to follow AI labeling standards"
    }
    
    if ($status.HasUncommittedChanges) {
        $prInfo.ActionItems += "Commit all changes"
    }
    
    if ($status.HasUnpushedCommits) {
        $prInfo.ActionItems += "Push all commits to remote"
    }
    
    foreach ($check in $prInfo.FailedChecks) {
        $prInfo.ActionItems += "Fix check: $check"
    }
    
    # If Bitcoin branch, check for architecture compliance
    if ($branch -match "bitcoin" -and -not $compliance.ArchitectureCompliance) {
        $prInfo.ActionItems += "Implement hexagonal architecture"
        $prInfo.ReadyForPR = $false
    }
    
    # If Bitcoin branch, check for documentation compliance
    if ($branch -match "bitcoin" -and -not $compliance.DocumentationCompliance) {
        $prInfo.ActionItems += "Add more documentation"
        $prInfo.ReadyForPR = $false
    }
    
    return $prInfo
}

function Generate-PRReport {
    param (
        [array]$prInfos
    )
    
    $reportPath = Join-Path $rootDir "PR_REPORT.md"
    
    $reportContent = @"
# Branch Review and PR Preparation Report
[AIR-3][AIS-3][BPC-3][AIT-3][RES-3]

This report was generated on $(Get-Date) and provides an overview of all feature branches and their PR readiness status.

## Summary

Total branches reviewed: $($prInfos.Count)
Branches ready for PR: $($prInfos | Where-Object { $_.ReadyForPR } | Measure-Object).Count
Branches requiring action: $($prInfos | Where-Object { -not $_.ReadyForPR } | Measure-Object).Count

## Branch Details

"@
    
    foreach ($prInfo in $prInfos) {
        $status = if ($prInfo.ReadyForPR) { "✅ Ready for PR" } else { "❌ Requires Action" }
        
        $reportContent += @"
### $($prInfo.SourceBranch) → $($prInfo.TargetBranch)

**Status**: $status

**Failed Checks**: $($prInfo.FailedChecks -join ", ")

**Action Items**:
$(if ($prInfo.ActionItems.Count -eq 0) { "- None" } else { $prInfo.ActionItems | ForEach-Object { "- $_" } | Out-String })

---

"@
    }
    
    $reportContent | Out-File -FilePath $reportPath -Encoding utf8
    
    Write-Step "Info" "PR report generated at $reportPath" "Green"
    return $reportPath
}

# Main script execution
Write-Step "Step 1/5" "Getting feature branches..." "Yellow"
$featureBranches = Get-FeatureBranches
Write-Host "Found $($featureBranches.Count) feature branches:" -ForegroundColor Yellow
$featureBranches | ForEach-Object { Write-Host "  - $_" -ForegroundColor Yellow }
Write-Host ""

$branchStatuses = @()
$branchCompliances = @()
$branchCheckResults = @{}
$prInfos = @()

Write-Step "Step 2/5" "Checking branch statuses..." "Yellow"
foreach ($branch in $featureBranches) {
    Write-Host "Checking status of $branch..." -ForegroundColor Yellow
    $status = Get-BranchStatus -branch $branch
    $branchStatuses += $status
    
    Write-Host "  - Commit count: $($status.CommitCount)" -ForegroundColor White
    Write-Host "  - Latest commit: $($status.LatestCommit)" -ForegroundColor White
    Write-Host "  - Unpushed commits: $($status.HasUnpushedCommits)" -ForegroundColor White
    Write-Host "  - Uncommitted changes: $($status.HasUncommittedChanges)" -ForegroundColor White
    Write-Host ""
}

Write-Step "Step 3/5" "Checking branch compliance..." "Yellow"
foreach ($branch in $featureBranches) {
    Write-Host "Checking compliance of $branch..." -ForegroundColor Yellow
    $compliance = Check-BranchCompliance -branch $branch
    $branchCompliances += $compliance
    
    Write-Host "  - Commit compliance: $($compliance.CommitCompliance)" -ForegroundColor White
    Write-Host "  - Architecture compliance: $($compliance.ArchitectureCompliance)" -ForegroundColor White
    Write-Host "  - Documentation compliance: $($compliance.DocumentationCompliance)" -ForegroundColor White
    Write-Host ""
}

Write-Step "Step 4/5" "Running PR checks..." "Yellow"
foreach ($branch in $featureBranches) {
    Write-Host "Running PR checks for $branch..." -ForegroundColor Yellow
    $checkResults = Run-PRChecks -branch $branch
    $branchCheckResults[$branch] = $checkResults
    
    foreach ($check in $checkResults.GetEnumerator()) {
        $statusSymbol = if ($check.Value) { "✅" } else { "❌" }
        Write-Host "  - $($check.Key): $statusSymbol" -ForegroundColor White
    }
    Write-Host ""
}

Write-Step "Step 5/5" "Preparing PR information..." "Yellow"
for ($i = 0; $i -lt $featureBranches.Count; $i++) {
    $branch = $featureBranches[$i]
    $status = $branchStatuses[$i]
    $compliance = $branchCompliances[$i]
    $checkResults = $branchCheckResults[$branch]
    
    Write-Host "Preparing PR info for $branch..." -ForegroundColor Yellow
    $prInfo = Prepare-PR -branch $branch -status $status -compliance $compliance -checkResults $checkResults
    $prInfos += $prInfo
    
    $readyStatus = if ($prInfo.ReadyForPR) { "✅ Ready for PR" } else { "❌ Requires Action" }
    Write-Host "  - PR Status: $readyStatus" -ForegroundColor White
    if ($prInfo.ActionItems.Count -gt 0) {
        Write-Host "  - Action Items:" -ForegroundColor White
        $prInfo.ActionItems | ForEach-Object { Write-Host "    - $_" -ForegroundColor White }
    }
    Write-Host ""
}

# Generate final report
$reportPath = Generate-PRReport -prInfos $prInfos

Write-Host ""
Write-Host "Branch review complete! ✅" -ForegroundColor Green
Write-Host "PR report has been generated at: $reportPath" -ForegroundColor Green
Write-Host "For branches that are ready for PR, you can now run scripts/bitcoin/merge_pr.ps1 to create PRs" -ForegroundColor Green 