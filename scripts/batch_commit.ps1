# batch_commit.ps1
#
# PowerShell version of batch commit script for Anya Core ecosystem
# Applies changes with proper labeling across multiple repositories
#
# Usage: ./scripts/batch_commit.ps1 -Message "Commit message" -Type "feat" -Scope "component" -Labels "AIR-3,AIS-2,AIT-3" [-Repos "repo1,repo2"] [-Validate]

# Default values
param(
    [Parameter(Mandatory=$true, HelpMessage="Commit message (required)")]
    [string]$Message,
    
    [Parameter(HelpMessage="Commit type (default: feat)")]
    [string]$Type = "feat",
    
    [Parameter(HelpMessage="Commit scope (optional)")]
    [string]$Scope = "",
    
    [Parameter(Mandatory=$true, HelpMessage="Comma-separated labels (required)")]
    [string]$Labels,
    
    [Parameter(HelpMessage="Comma-separated repository list (default: all)")]
    [string]$Repos = "",
    
    [Parameter(HelpMessage="Validate labels before committing")]
    [switch]$Validate = $false,
    
    [Parameter(HelpMessage="Show what would be committed without making changes")]
    [switch]$DryRun = $false,
    
    [Parameter(HelpMessage="Show help message")]
    [switch]$Help = $false,
    
    [Parameter(HelpMessage="Root directory containing all repositories")]
    [string]$RootDir = ""
)

# Find the workspace root directory
$SCRIPT_DIR = Split-Path -Parent $MyInvocation.MyCommand.Path
# Check if SCRIPT_DIR ends with 'scripts' (user running from scripts directory)
if ($SCRIPT_DIR -match '\\scripts$') {
    $BASE_DIR = Split-Path -Parent $SCRIPT_DIR
} else {
    # Assume the script is being run from a direct path
    $BASE_DIR = $SCRIPT_DIR
}

if ([string]::IsNullOrEmpty($RootDir)) {
    $ROOT_DIR = Split-Path -Parent $BASE_DIR
} else {
    $ROOT_DIR = $RootDir
}
$LABEL_CACHE_FILE = Join-Path -Path $BASE_DIR -ChildPath ".label_cache.json"

Write-Host "Working with base directory: $BASE_DIR" -ForegroundColor Cyan
Write-Host "Using root directory for repositories: $ROOT_DIR" -ForegroundColor Cyan

# Display help information
function Show-Help {
    Write-Host "Batch Commit Tool with Comprehensive Labeling" -ForegroundColor Cyan
    Write-Host "=============================================" -ForegroundColor Cyan
    Write-Host "Usage: ./scripts/batch_commit.ps1 [options]"
    Write-Host ""
    Write-Host "Parameters:" -ForegroundColor Yellow
    Write-Host "  -Message ""MESSAGE""        Commit message (required)"
    Write-Host "  -Type ""TYPE""              Commit type (default: feat)"
    Write-Host "  -Scope ""SCOPE""            Commit scope (optional)"
    Write-Host "  -Labels ""LABELS""          Comma-separated labels (required)"
    Write-Host "  -Repos ""REPOSITORIES""     Comma-separated repository list (default: all)"
    Write-Host "  -Validate                  Validate labels before committing"
    Write-Host "  -DryRun                    Show what would be committed without making changes"
    Write-Host "  -Help                      Show this help message"
    Write-Host "  -RootDir ""PATH""           Root directory containing all repositories (optional)"
    Write-Host ""
    Write-Host "Examples:" -ForegroundColor Green
    Write-Host '  ./scripts/batch_commit.ps1 -Message "Update AI models" -Type "feat" -Scope "ml" -Labels "AIR-3,AIS-2,AIT-3,AIM-2"'
    Write-Host '  ./scripts/batch_commit.ps1 -Message "Fix security issues" -Type "fix" -Scope "security" -Labels "AIR-3,AIS-3" -Repos "anya-core,anya-bitcoin" -Validate'
    Write-Host ""
    Write-Host "Available commit types:" -ForegroundColor Yellow
    Write-Host "  feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert"
    Write-Host ""
    Write-Host "See docs/standards/AI_LABELING.md for label requirements by component type"
}

# Process help request
if ($Help) {
    Show-Help
    exit 0
}

# Validate commit type
$VALID_TYPES = @("feat", "fix", "docs", "style", "refactor", "perf", "test", "build", "ci", "chore", "revert")
if ($VALID_TYPES -notcontains $Type) {
    Write-Host "Error: Invalid commit type: $Type" -ForegroundColor Red
    Write-Host "Valid types: $($VALID_TYPES -join ', ')" -ForegroundColor Yellow
    exit 1
}

# Format scope if provided
$FormattedScope = ""
if (-not [string]::IsNullOrEmpty($Scope)) {
    $FormattedScope = "($Scope)"
}

# Format labels
# Convert comma-separated list to array
$LABEL_ARRAY = $Labels -split ','
$FORMATTED_LABELS = ""
foreach ($label in $LABEL_ARRAY) {
    # Trim whitespace
    $label = $label.Trim()
    $FORMATTED_LABELS += "[$label]"
}

# Auto-detect repositories in the root directory
try {
    $detectedRepos = Get-ChildItem -Path $ROOT_DIR -Directory | 
                     Where-Object { Test-Path -Path (Join-Path -Path $_.FullName -ChildPath ".git") -PathType Container } |
                     Select-Object -ExpandProperty Name
    
    if ($detectedRepos.Count -gt 0) {
        $Repos = $detectedRepos -join ","
        Write-Host "Auto-detected repositories: $Repos" -ForegroundColor Green
    } else {
        # Search one level deeper in case of nested structure
        Write-Host "No repositories found at top level, searching one level deeper..." -ForegroundColor Yellow
        $detectedRepos = Get-ChildItem -Path $ROOT_DIR -Directory | 
                         ForEach-Object { 
                             Get-ChildItem -Path $_.FullName -Directory -ErrorAction SilentlyContinue
                         } |
                         Where-Object { Test-Path -Path (Join-Path -Path $_.FullName -ChildPath ".git") -PathType Container } |
                         Select-Object -ExpandProperty Name
        
        if ($detectedRepos.Count -gt 0) {
            $Repos = $detectedRepos -join ","
            Write-Host "Auto-detected repositories in subdirectories: $Repos" -ForegroundColor Green
        } else {
            # Default to current repository only if nothing else found
            $currentFolder = Split-Path -Leaf $BASE_DIR
            $Repos = $currentFolder
            Write-Host "No repositories detected. Using current repository: $Repos" -ForegroundColor Yellow
        }
    }
} catch {
    # Default to current repository if detection fails
    $currentFolder = Split-Path -Leaf $BASE_DIR
    $Repos = $currentFolder
    Write-Host "Repository detection failed. Using current repository: $Repos" -ForegroundColor Yellow
}
$REPO_ARRAY = $Repos -split ','

# Function to validate labels
function Test-Labels {
    param(
        [string]$Component,
        [string]$LabelList
    )
    
    # Load validation rules based on component
    $required = @()
    $recommended = @()
    
    switch -Regex ($Component) {
        "bitcoin|btc|lightning|ln" {
            $required = @("AIR", "AIS", "AIT", "BPC")
            $recommended = @("PFM", "SCL", "RES")
        }
        "web5|dwn|did" {
            $required = @("AIR", "AIS", "AIT", "W5C", "DID")
            $recommended = @("PFM", "SCL", "RES")
        }
        "ml|ai|model" {
            $required = @("AIR", "AIS", "AIT", "AIM", "AIP", "AIE")
            $recommended = @("PFM", "SCL", "RES")
        }
        "ui|ux|frontend" {
            $required = @("AIR", "UXA")
            $recommended = @("PFM", "AIP")
        }
        "api|service" {
            $required = @("AIR", "AIS", "AIP")
            $recommended = @("PFM", "SCL", "RES")
        }
        "core|system" {
            $required = @("AIR", "AIS", "AIT", "PFM", "RES", "SCL")
            $recommended = @()
        }
        "dao|governance" {
            $required = @("AIR", "AIS", "AIT", "DAO")
            $recommended = @("PFM", "RES", "SCL")
        }
        default {
            # Default requirements
            $required = @("AIR", "AIS")
            $recommended = @("AIT", "PFM")
        }
    }
    
    # Check for required labels
    $missing_required = @()
    foreach ($req in $required) {
        if ($LabelList -notmatch $req) {
            $missing_required += $req
        }
    }
    
    # Check for recommended labels
    $missing_recommended = @()
    foreach ($rec in $recommended) {
        if ($LabelList -notmatch $rec) {
            $missing_recommended += $rec
        }
    }
    
    # Output validation results
    if ($missing_required.Count -gt 0) {
        Write-Host "Error: Missing required labels for $Component`: $($missing_required -join ', ')" -ForegroundColor Red
        return $false
    }
    
    if ($missing_recommended.Count -gt 0) {
        Write-Host "Warning: Missing recommended labels for $Component`: $($missing_recommended -join ', ')" -ForegroundColor Yellow
    }
    
    return $true
}

# Function to create full commit message
function Get-CommitMessage {
    param(
        [string]$Message,
        [string]$Type,
        [string]$Scope,
        [string]$FormattedLabels
    )
    
    # Create conventional commit format
    $commitMsg = "$Type$Scope`: $Message`n`nLabels: $FormattedLabels"
    return $commitMsg
}

# Function to check if git is available
function Test-GitAvailable {
    try {
        $null = & git --version
        return $true
    }
    catch {
        Write-Host "Error: Git is not available on this system or not in the PATH." -ForegroundColor Red
        Write-Host "Please install Git or add it to your PATH variable." -ForegroundColor Yellow
        return $false
    }
}

# Check if git is available
if (-not (Test-GitAvailable)) {
    exit 1
}

# Main execution
Write-Host "Batch Commit with Comprehensive Labeling" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Commit Type: $Type"
if (-not [string]::IsNullOrEmpty($Scope)) {
    Write-Host "Scope: $Scope"
}
Write-Host "Message: $Message"
Write-Host "Labels: $FORMATTED_LABELS"
Write-Host "Repositories: $($Repos -replace ',', ', ')"
Write-Host "Root Directory: $ROOT_DIR"
Write-Host ""

# Validate labels if requested
if ($Validate) {
    Write-Host "Validating labels..." -ForegroundColor Yellow
    # Use the scope as component
    $valid = Test-Labels -Component $Scope -LabelList $Labels
    
    if (-not $valid) {
        Write-Host "Label validation failed. Run with -Help to see label requirements." -ForegroundColor Red
        exit 1
    }
    
    Write-Host "Label validation passed." -ForegroundColor Green
    Write-Host ""
}

# Generate commit message
$COMMIT_MESSAGE = Get-CommitMessage -Message $Message -Type $Type -Scope $FormattedScope -FormattedLabels $FORMATTED_LABELS

# Display commit details
Write-Host "Commit Message:" -ForegroundColor Yellow
Write-Host "-----------------------------------"
Write-Host $COMMIT_MESSAGE
Write-Host "-----------------------------------"
Write-Host ""

# Track successful and failed operations
$successful_commits = @()
$failed_repos = @()

# Process each repository
foreach ($repo in $REPO_ARRAY) {
    $repo_path = Join-Path -Path $ROOT_DIR -ChildPath $repo
    
    # Skip if repository doesn't exist
    if (-not (Test-Path -Path $repo_path -PathType Container)) {
        Write-Host "Warning: Repository $repo not found at $repo_path" -ForegroundColor Yellow
        $failed_repos += $repo
        continue
    }
    
    # Skip if not a git repository
    $git_dir = Join-Path -Path $repo_path -ChildPath ".git"
    if (-not (Test-Path -Path $git_dir -PathType Container)) {
        Write-Host "Warning: $repo is not a git repository" -ForegroundColor Yellow
        $failed_repos += $repo
        continue
    }
    
    Write-Host "Processing repository: $repo" -ForegroundColor Cyan
    
    # Save current location to return to later
    $currentLocation = Get-Location
    
    # Change to repository directory
    try {
        Set-Location -Path $repo_path -ErrorAction Stop
    }
    catch {
        Write-Host "Error: Failed to change directory to $repo_path" -ForegroundColor Red
        Write-Host "  Details: $($_.Exception.Message)" -ForegroundColor Red
        $failed_repos += $repo
        continue
    }
    
    # Check if there are any changes to commit
    try {
        $hasChanges = $false
        git update-index -q --refresh
        $gitStatus = git status -s
        if (-not [string]::IsNullOrEmpty($gitStatus)) {
            $hasChanges = $true
        }
        
        if (-not $hasChanges) {
            Write-Host "No changes to commit in $repo" -ForegroundColor Yellow
            Set-Location -Path $currentLocation
            continue
        }
    }
    catch {
        Write-Host "Error: Failed to check git status in $repo" -ForegroundColor Red
        Write-Host "  Details: $($_.Exception.Message)" -ForegroundColor Red
        Set-Location -Path $currentLocation
        $failed_repos += $repo
        continue
    }
    
    # Perform the commit
    if ($DryRun) {
        Write-Host "DRY RUN: Would commit changes in $repo with message:" -ForegroundColor Yellow
        Write-Host $COMMIT_MESSAGE
        $successful_commits += $repo
    } else {
        try {
            Write-Host "Committing changes in $repo..." -ForegroundColor Green
            
            # Add changes
            git add .
            
            # Creating a temporary file for the commit message
            $tempFile = New-TemporaryFile
            Set-Content -Path $tempFile -Value $COMMIT_MESSAGE
            
            # Commit changes
            $commitOutput = git commit -F $tempFile 2>&1
            Remove-Item -Path $tempFile -Force
            
            # Check if commit was successful
            if ($LASTEXITCODE -eq 0) {
                Write-Host "Changes committed successfully in $repo" -ForegroundColor Green
                $successful_commits += $repo
            } else {
                Write-Host "Failed to commit changes in $repo" -ForegroundColor Red
                Write-Host "Git output: $commitOutput" -ForegroundColor Red
                $failed_repos += $repo
            }
        }
        catch {
            Write-Host "Error: Exception during commit in $repo" -ForegroundColor Red
            Write-Host "  Details: $($_.Exception.Message)" -ForegroundColor Red
            $failed_repos += $repo
        }
    }
    
    # Return to original location
    Set-Location -Path $currentLocation
    Write-Host ""
}

# Summary
Write-Host "Batch Commit Summary:" -ForegroundColor Cyan
Write-Host "====================" -ForegroundColor Cyan
Write-Host "Successful commits: $($successful_commits.Count)" -ForegroundColor Green
if ($successful_commits.Count -gt 0) {
    Write-Host "  - $($successful_commits -join ', ')" -ForegroundColor Green
}
Write-Host "Failed repositories: $($failed_repos.Count)" -ForegroundColor $(if ($failed_repos.Count -gt 0) { "Red" } else { "Green" })
if ($failed_repos.Count -gt 0) {
    Write-Host "  - $($failed_repos -join ', ')" -ForegroundColor Red
}

Write-Host "`nBatch commit process completed." -ForegroundColor Cyan 