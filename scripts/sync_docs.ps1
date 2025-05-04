# Documentation Synchronization Script
# This script ensures documentation consistency across all Anya Core repositories

param(
    [string]$SourceRepo = "anya-core",
    [string]$TargetRepos = "anya-bitcoin,anya-web5,anya-enterprise",
    [switch]$CheckOnly,
    [switch]$DryRun,
    [string]$RootDir = "."
)

# Configuration
$Config = @{
    RequiredFiles = @(
        "README.md",
        "API.md",
        "CONTRIBUTING.md",
        "SECURITY.md",
        "CHANGELOG.md"
    )
    
    RequiredDirs = @(
        "docs/api",
        "docs/architecture",
        "docs/development",
        "docs/guides",
        "docs/security"
    )
}

# Function to validate documentation structure
function Test-DocStructure {
    param (
        [string]$RepoPath
    )
    
    $missing = @()
    
    # Check required files
    foreach ($file in $Config.RequiredFiles) {
        if (-not (Test-Path (Join-Path $RepoPath $file))) {
            $missing += "Missing file: $file"
        }
    }
    
    # Check required directories
    foreach ($dir in $Config.RequiredDirs) {
        if (-not (Test-Path (Join-Path $RepoPath $dir))) {
            $missing += "Missing directory: $dir"
        }
    }
    
    return $missing
}

# Function to sync documentation
function Sync-Documentation {
    param (
        [string]$SourcePath,
        [string]$TargetPath,
        [string[]]$Files
    )
    
    foreach ($file in $Files) {
        $sourceFile = Join-Path $SourcePath $file
        $targetFile = Join-Path $TargetPath $file
        
        if (Test-Path $sourceFile) {
            if (-not $DryRun) {
                Copy-Item -Path $sourceFile -Destination $targetFile -Force
                Write-Host "Synced: $file"
            } else {
                Write-Host "Would sync: $file"
            }
        }
    }
}

# Main execution
Write-Host "Starting documentation synchronization..."

# Validate source repository
$sourcePath = Join-Path $RootDir $SourceRepo
if (-not (Test-Path $sourcePath)) {
    Write-Error "Source repository not found: $sourcePath"
    exit 1
}

# Process each target repository
$targetRepos = $TargetRepos.Split(",")
foreach ($repo in $targetRepos) {
    Write-Host "`nProcessing repository: $repo"
    
    $targetPath = Join-Path $RootDir $repo
    if (-not (Test-Path $targetPath)) {
        Write-Warning "Target repository not found: $targetPath"
        continue
    }
    
    # Check documentation structure
    $missing = Test-DocStructure -RepoPath $targetPath
    if ($missing.Count -gt 0) {
        Write-Host "Missing documentation in $repo :"
        $missing | ForEach-Object { Write-Host "  - $_" }
    }
    
    if (-not $CheckOnly) {
        # Sync documentation
        Sync-Documentation -SourcePath $sourcePath -TargetPath $targetPath -Files $Config.RequiredFiles
    }
}

Write-Host "`nDocumentation synchronization complete." 