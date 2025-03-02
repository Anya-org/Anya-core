# batch_commit.ps1
#
# PowerShell version of batch commit script for Anya Core ecosystem
# Applies changes with proper labeling across multiple repositories
#
# Usage: ./batch_commit.ps1 -Message "Commit message" -Type "feat" -Scope "component" -Labels "AIR-3,AIS-2,AIT-3" [-Repos "repo1,repo2"] [-Validate]

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
    [switch]$Help = $false
)

# Base directory
$BASE_DIR = Get-Location
$LABEL_CACHE_FILE = Join-Path -Path $BASE_DIR -ChildPath ".label_cache.json"

# Display help information
function Show-Help {
    Write-Host "Batch Commit Tool with Comprehensive Labeling" -ForegroundColor Cyan
    Write-Host "=============================================" -ForegroundColor Cyan
    Write-Host "Usage: ./batch_commit.ps1 [options]"
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
    Write-Host ""
    Write-Host "Examples:" -ForegroundColor Green
    Write-Host '  ./batch_commit.ps1 -Message "Update AI models" -Type "feat" -Scope "ml" -Labels "AIR-3,AIS-2,AIT-3,AIM-2"'
    Write-Host '  ./batch_commit.ps1 -Message "Fix security issues" -Type "fix" -Scope "security" -Labels "AIR-3,AIS-3" -Repos "anya-core,anya-web5" -Validate'
    Write-Host ""
    Write-Host "Available commit types:" -ForegroundColor Yellow
    Write-Host "  feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert"
    Write-Host ""
    Write-Host "See AI_LABELLING.md for label requirements by component type"
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

# Get list of repositories
if ([string]::IsNullOrEmpty($Repos)) {
    # Default list of repositories
    $Repos = "anya-core,anya-web5,anya-mobile,anya-bitcoin,dash33"
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

# Process each repository
foreach ($repo in $REPO_ARRAY) {
    $repo_path = Join-Path -Path (Split-Path -Parent $BASE_DIR) -ChildPath $repo
    
    # Skip if repository doesn't exist
    if (-not (Test-Path -Path $repo_path -PathType Container)) {
        Write-Host "Warning: Repository $repo not found at $repo_path" -ForegroundColor Yellow
        continue
    }
    
    Write-Host "Processing repository: $repo" -ForegroundColor Cyan
    
    # Check if there are any changes to commit
    Push-Location -Path $repo_path
    git update-index -q --refresh
    $hasChanges = $false
    $gitStatus = git status -s
    if (-not [string]::IsNullOrEmpty($gitStatus)) {
        $hasChanges = $true
    }
    
    if (-not $hasChanges) {
        Write-Host "No changes to commit in $repo" -ForegroundColor Yellow
        Pop-Location
        continue
    }
    
    # Perform the commit
    if ($DryRun) {
        Write-Host "DRY RUN: Would commit changes in $repo with message:" -ForegroundColor Yellow
        Write-Host $COMMIT_MESSAGE
    } else {
        Write-Host "Committing changes in $repo..." -ForegroundColor Green
        git add .
        # Creating a temporary file for the commit message
        $tempFile = New-TemporaryFile
        Set-Content -Path $tempFile -Value $COMMIT_MESSAGE
        git commit -F $tempFile
        Remove-Item -Path $tempFile
        Write-Host "Changes committed successfully in $repo" -ForegroundColor Green
    }
    
    Pop-Location
    Write-Host ""
}

Write-Host "Batch commit process completed." -ForegroundColor Green 