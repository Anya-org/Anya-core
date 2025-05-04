# Sign Previous Commits Script
# This script helps retroactively sign git commits with your GPG key

param (
    [int]$CommitCount = 10,
    [string]$BranchName = "",
    [switch]$DryRun = $false
)

$ErrorActionPreference = "Stop"

# Function to display script usage
function Show-Usage {
    Write-Host "Sign Previous Commits Script" -ForegroundColor Cyan
    Write-Host "This script helps retroactively sign git commits with your GPG key" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Usage:" -ForegroundColor Yellow
    Write-Host "  .\sign-previous-commits.ps1 [-CommitCount <count>] [-BranchName <branch>] [-DryRun]" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Parameters:" -ForegroundColor Yellow
    Write-Host "  -CommitCount <count>   Number of commits to examine (default: 10)" -ForegroundColor Yellow
    Write-Host "  -BranchName <branch>   Branch to rebase (default: current branch)" -ForegroundColor Yellow
    Write-Host "  -DryRun                Show what would be done without making changes" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Example:" -ForegroundColor Green
    Write-Host "  .\sign-previous-commits.ps1 -CommitCount 5" -ForegroundColor Green
    Write-Host "  .\sign-previous-commits.ps1 -BranchName feature/my-branch -DryRun" -ForegroundColor Green
}

# Function to check if Git is installed
function Test-GitInstalled {
    try {
        $null = git --version
        return $true
    }
    catch {
        Write-Host "Git is not installed or not in PATH." -ForegroundColor Red
        return $false
    }
}

# Function to check if we're in a git repository
function Test-GitRepository {
    try {
        $null = git rev-parse --is-inside-work-tree
        return $true
    }
    catch {
        Write-Host "Current directory is not a Git repository." -ForegroundColor Red
        return $false
    }
}

# Function to check if GPG signing is configured
function Test-GitSigningConfigured {
    $signingKey = git config --get user.signingkey
    $gpgSign = git config --get commit.gpgsign

    if (-not $signingKey -or $gpgSign -ne "true") {
        Write-Host "Git GPG signing is not properly configured." -ForegroundColor Red
        Write-Host "Please run .\configure-git-signing.ps1 first." -ForegroundColor Yellow
        return $false
    }
    return $true
}

# Function to get the current branch name
function Get-CurrentBranch {
    return git rev-parse --abbrev-ref HEAD
}

# Function to get commits that need signing
function Get-UnsignedCommits {
    param (
        [int]$Count
    )
    
    $commits = @()
    $gitLog = git log --format="%h %G? %an %s" -n $Count
    
    foreach ($line in $gitLog) {
        $parts = $line -split ' ', 3
        $hash = $parts[0]
        $signStatus = $parts[1]
        $rest = $parts[2]
        
        # N = no signature
        # B = bad signature
        # U = unknown signature
        if ($signStatus -eq "N" -or $signStatus -eq "B" -or $signStatus -eq "U") {
            $commits += [PSCustomObject]@{
                Hash = $hash
                Status = $signStatus
                Details = $rest
            }
        }
    }
    
    return $commits
}

# Function to create a rebase todo script
function Create-RebaseTodoScript {
    param (
        [array]$Commits,
        [string]$OutputPath
    )
    
    $rebaseContent = @()
    
    foreach ($commit in $Commits) {
        $rebaseContent += "exec git commit --amend --no-edit -S"
        $rebaseContent += "pick $($commit.Hash)"
    }
    
    # Remove the last "pick" line to avoid adding an extra commit
    if ($rebaseContent.Count -gt 0) {
        $rebaseContent = $rebaseContent[0..($rebaseContent.Count - 2)]
    }
    
    $rebaseContent | Set-Content -Path $OutputPath
    return $rebaseContent
}

# Main script execution
if (-not (Test-GitInstalled)) {
    exit 1
}

if (-not (Test-GitRepository)) {
    exit 1
}

if (-not (Test-GitSigningConfigured)) {
    exit 1
}

# Determine the branch to work with
if (-not $BranchName) {
    $BranchName = Get-CurrentBranch
}

# Get unsigned commits
$unsignedCommits = Get-UnsignedCommits -Count $CommitCount
$unsignedCount = $unsignedCommits.Count

if ($unsignedCount -eq 0) {
    Write-Host "No unsigned commits found in the last $CommitCount commits." -ForegroundColor Green
    exit 0
}

# Display unsigned commits
Write-Host "Found $unsignedCount unsigned commits in the last $CommitCount commits:" -ForegroundColor Yellow
foreach ($commit in $unsignedCommits) {
    Write-Host "$($commit.Hash) ($($commit.Status)) - $($commit.Details)" -ForegroundColor White
}

# Confirm action
if (-not $DryRun) {
    Write-Host ""
    Write-Host "WARNING: This will rewrite Git history by adding signatures to these commits." -ForegroundColor Red
    Write-Host "If you've already pushed these commits, you'll need to force push after signing." -ForegroundColor Red
    Write-Host "This can cause problems for other contributors if they've based work on these commits." -ForegroundColor Red
    Write-Host ""
    $confirmation = Read-Host "Do you want to proceed? (y/N)"
    
    if ($confirmation -ne "y" -and $confirmation -ne "Y") {
        Write-Host "Operation cancelled." -ForegroundColor Yellow
        exit 0
    }
    
    # Create a temporary file for the rebase script
    $tempFile = [System.IO.Path]::GetTempFileName()
    
    # Generate the rebase todo script
    $rebaseContent = Create-RebaseTodoScript -Commits $unsignedCommits -OutputPath $tempFile
    
    # Start the rebase process
    $oldestCommit = $unsignedCommits[-1].Hash + "~1"
    
    Write-Host "Starting interactive rebase to sign commits..." -ForegroundColor Cyan
    Write-Host "If Git opens an editor, save and close it to continue." -ForegroundColor Cyan
    
    # Execute the rebase
    git rebase -i $oldestCommit --exec "git commit --amend --no-edit -S"
    
    $rebaseResult = $?
    
    # Cleanup
    Remove-Item -Path $tempFile -Force
    
    if ($rebaseResult) {
        Write-Host "Successfully signed commits!" -ForegroundColor Green
        Write-Host "If these commits were already pushed, you'll need to force push:" -ForegroundColor Yellow
        Write-Host "  git push --force origin $BranchName" -ForegroundColor Yellow
    }
    else {
        Write-Host "Rebase failed or was aborted." -ForegroundColor Red
        Write-Host "You may need to run 'git rebase --abort' to cleanup." -ForegroundColor Yellow
    }
}
else {
    # Dry run mode
    Write-Host ""
    Write-Host "DRY RUN: The following actions would be taken:" -ForegroundColor Cyan
    Write-Host " - Interactive rebase would be started from commit $($unsignedCommits[-1].Hash)~1" -ForegroundColor White
    Write-Host " - Each commit would be amended with your GPG signature" -ForegroundColor White
    Write-Host " - You would need to force push after completion" -ForegroundColor White
    Write-Host ""
    Write-Host "To execute these actions, run without the -DryRun parameter." -ForegroundColor Yellow
} 