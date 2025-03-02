# sync_labelling.ps1
#
# PowerShell version of sync_labelling.py
# Synchronizes AI_LABELLING.md across all repositories in the Anya ecosystem.
# Ensures consistent labeling standards across the entire project.
#
# Usage: ./scripts/sync_labelling.ps1 [-Source repo] [-Target repos] [-CheckOnly] [-DryRun]

param(
    [Parameter(HelpMessage="Source repository for label standards")]
    [string]$Source = "anya-core",
    
    [Parameter(HelpMessage="Target repositories (comma-separated)")]
    [string]$Target = "anya-core,anya-web5,anya-mobile,anya-bitcoin,dash33",
    
    [Parameter(HelpMessage="Only check for differences without making changes")]
    [switch]$CheckOnly = $false,
    
    [Parameter(HelpMessage="Show what would be done without making actual changes")]
    [switch]$DryRun = $false,
    
    [Parameter(HelpMessage="Do not commit changes after synchronization")]
    [switch]$NoCommit = $false,
    
    [Parameter(HelpMessage="Use batch_commit.ps1 for committing changes")]
    [switch]$BatchCommit = $false,
    
    [Parameter(HelpMessage="Root directory containing all repositories")]
    [string]$RootDir = "",
    
    [Parameter(HelpMessage="Show help message")]
    [switch]$Help = $false
)

# Constants
$LABELLING_FILE = "AI_LABELLING.md"
$COMMIT_RULES_FILE = "COMMIT_RULES.md"
$LABEL_HISTORY_DIR = ".label_history"

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

Write-Host "Working with base directory: $BASE_DIR" -ForegroundColor Cyan
Write-Host "Using root directory for repositories: $ROOT_DIR" -ForegroundColor Cyan

# Display help information
function Show-Help {
    Write-Host "Labeling Synchronization Tool" -ForegroundColor Cyan
    Write-Host "============================" -ForegroundColor Cyan
    Write-Host "Usage: ./scripts/sync_labelling.ps1 [options]"
    Write-Host ""
    Write-Host "Parameters:" -ForegroundColor Yellow
    Write-Host "  -Source ""REPO""           Source repository for label standards (default: anya-core)"
    Write-Host "  -Target ""REPOS""          Target repositories (comma-separated, default: all)"
    Write-Host "  -CheckOnly               Only check for differences without making changes"
    Write-Host "  -DryRun                  Show what would be done without making actual changes"
    Write-Host "  -NoCommit                Do not commit changes after synchronization"
    Write-Host "  -BatchCommit             Use batch_commit.ps1 for committing changes"
    Write-Host "  -RootDir ""PATH""          Root directory containing all repositories (optional)"
    Write-Host "  -Help                    Show this help message"
    Write-Host ""
    Write-Host "Examples:" -ForegroundColor Green
    Write-Host "  ./scripts/sync_labelling.ps1"
    Write-Host "  ./scripts/sync_labelling.ps1 -CheckOnly"
    Write-Host "  ./scripts/sync_labelling.ps1 -Target ""anya-web5,anya-bitcoin"" -DryRun"
    Write-Host ""
}

# Process help request
if ($Help) {
    Show-Help
    exit 0
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

# Get file hash
function Get-FileHash {
    param(
        [string]$FilePath
    )
    
    if (-not (Test-Path -Path $FilePath)) {
        return $null
    }
    
    $md5 = [System.Security.Cryptography.MD5]::Create()
    $stream = [System.IO.File]::OpenRead($FilePath)
    $hashBytes = $md5.ComputeHash($stream)
    $stream.Close()
    $md5.Dispose()
    
    return [Convert]::ToBase64String($hashBytes)
}

# Save history
function Save-History {
    param(
        [string]$SourcePath,
        [string]$HistoryDir
    )
    
    # Create history directory if it doesn't exist
    if (-not (Test-Path -Path $HistoryDir)) {
        New-Item -Path $HistoryDir -ItemType Directory -Force | Out-Null
    }
    
    # Create timestamped filename
    $timestamp = Get-Date -Format "yyyyMMdd_HHmmss"
    $historyFile = Join-Path -Path $HistoryDir -ChildPath "$LABELLING_FILE`_$timestamp"
    
    # Save a copy
    Copy-Item -Path $SourcePath -Destination $historyFile -Force
    Write-Host "Saved historical copy: $historyFile" -ForegroundColor Green
    
    # Keep only the last 10 historical files
    $allHistoryFiles = Get-ChildItem -Path $HistoryDir |
                      Where-Object { $_.Name -like "$LABELLING_FILE`_*" -and $_.Name -ne $LABELLING_FILE } |
                      Sort-Object -Property LastWriteTime
    
    if ($allHistoryFiles.Count -gt 10) {
        $filesToRemove = $allHistoryFiles | Select-Object -First ($allHistoryFiles.Count - 10)
        foreach ($file in $filesToRemove) {
            Remove-Item -Path $file.FullName -Force
            Write-Host "Removed old history file: $($file.Name)" -ForegroundColor Yellow
        }
    }
}

# Synchronize file
function Sync-File {
    param(
        [string]$SourceRepo,
        [string]$TargetRepo,
        [string]$Filename,
        [bool]$DryRun,
        [bool]$CheckOnly
    )
    
    $sourcePath = Join-Path -Path (Join-Path -Path $ROOT_DIR -ChildPath $SourceRepo) -ChildPath $Filename
    $targetPath = Join-Path -Path (Join-Path -Path $ROOT_DIR -ChildPath $TargetRepo) -ChildPath $Filename
    
    # Convert paths to absolute if they exist
    if (Test-Path -Path $sourcePath) {
        $sourcePath = Resolve-Path -Path $sourcePath
    } else {
        Write-Host "ERROR: Source file $sourcePath does not exist" -ForegroundColor Red
        return $false, $false
    }
    
    # Calculate file hashes
    $sourceHash = Get-FileHash -FilePath $sourcePath
    $targetHash = Get-FileHash -FilePath $targetPath
    
    # Skip if the files are identical
    if ($sourceHash -eq $targetHash -and $targetHash -ne $null) {
        Write-Host "✓ $Filename in $TargetRepo is already up to date" -ForegroundColor Green
        return $false, $true
    }
    
    # Check if target file exists but with different content
    if ($targetHash -ne $null -and $sourceHash -ne $targetHash) {
        Write-Host "! $Filename in $TargetRepo differs from source" -ForegroundColor Yellow
        if ($CheckOnly) {
            return $true, $false
        }
    }
    
    # Copy the file if not in check-only mode
    if (-not $CheckOnly) {
        if ($DryRun) {
            Write-Host "WOULD COPY: $sourcePath -> $targetPath" -ForegroundColor Yellow
        } else {
            # Create target directory if needed
            $targetDir = Split-Path -Parent $targetPath
            if (-not (Test-Path -Path $targetDir)) {
                New-Item -Path $targetDir -ItemType Directory -Force | Out-Null
            }
            
            # Save historical copy
            $historyDir = Join-Path -Path $targetDir -ChildPath $LABEL_HISTORY_DIR
            if (Test-Path -Path $targetPath) {
                Save-History -SourcePath $targetPath -HistoryDir $historyDir
            }
            
            # Copy the file
            Copy-Item -Path $sourcePath -Destination $targetPath -Force
            Write-Host "✓ Updated $Filename in $TargetRepo" -ForegroundColor Green
        }
        
        return $true, $true
    }
    
    return $true, $false
}

# Commit changes
function Commit-Changes {
    param(
        [string]$Repo,
        [string[]]$ChangedFiles,
        [bool]$DryRun,
        [bool]$NoCommit,
        [bool]$BatchCommit
    )
    
    $repoPath = Join-Path -Path $ROOT_DIR -ChildPath $Repo
    
    # No changes to commit
    if ($ChangedFiles.Count -eq 0) {
        return $true
    }
    
    # Don't commit if -NoCommit was specified
    if ($NoCommit) {
        Write-Host "NOT COMMITTING: Changes in $Repo (-NoCommit specified)" -ForegroundColor Yellow
        return $true
    }
    
    # Use batch_commit.ps1 if specified
    if ($BatchCommit) {
        $batchScript = Join-Path -Path $BASE_DIR -ChildPath "scripts\batch_commit.ps1"
        if (-not (Test-Path -Path $batchScript)) {
            Write-Host "ERROR: batch_commit.ps1 not found at $batchScript" -ForegroundColor Red
            return $false
        }
        
        if ($DryRun) {
            Write-Host "WOULD BATCH COMMIT: Changes in $Repo" -ForegroundColor Yellow
            return $true
        }
        
        # Prepare the command
        $params = @{
            Message = "Synchronize AI labelling system across repositories"
            Type = "docs"
            Scope = "labelling"
            Labels = "AIR-3,AIS-3,AIE-3"
            Repos = $Repo
            RootDir = $ROOT_DIR
        }
        
        try {
            & $batchScript @params
            Write-Host "✓ Committed changes in $Repo using batch_commit.ps1" -ForegroundColor Green
            return $true
        } catch {
            $errorMsg = $_.Exception.Message
            Write-Host "ERROR: Failed to commit changes in $Repo`: $errorMsg" -ForegroundColor Red
            return $false
        }
    }
    
    # Direct git commit
    if ($DryRun) {
        Write-Host "WOULD COMMIT: Changes in $Repo" -ForegroundColor Yellow
        return $true
    }
    
    try {
        # Check if git is installed
        if (-not (Test-GitAvailable)) {
            return $false
        }
        
        # Save current location
        $currentLocation = Get-Location
        
        # Change to repository directory
        Set-Location -Path $repoPath
        
        # Add changes
        $null = & git add $ChangedFiles
        
        # Create commit message
        $commitMsg = @"
docs(labelling): synchronize AI labelling system

Labels: [AIR-3][AIS-3][AIE-3]

Ensure consistent labelling standards across all repositories.
"@
        
        # Commit changes
        $tempFile = New-TemporaryFile
        Set-Content -Path $tempFile -Value $commitMsg
        $commitOutput = git commit -F $tempFile.FullName 2>&1
        $commitSuccess = $LASTEXITCODE -eq 0
        Remove-Item -Path $tempFile -Force
        
        if ($commitSuccess) {
            Write-Host "✓ Committed changes in $Repo" -ForegroundColor Green
        } else {
            Write-Host "ERROR: Failed to commit changes in $Repo" -ForegroundColor Red
            Write-Host "Git output: $commitOutput" -ForegroundColor Red
        }
        
        # Return to original location
        Set-Location -Path $currentLocation
        return $commitSuccess
    } catch {
        $errorMsg = $_.Exception.Message
        Write-Host "ERROR: Failed to commit changes in $Repo`: $errorMsg" -ForegroundColor Red
        
        # Make sure we return to the original location
        if ((Get-Location).Path -eq $repoPath) {
            Set-Location -Path $currentLocation
        }
        return $false
    }
}

# Check if repositories exist and are git repositories
function Test-Repositories {
    param(
        [string[]]$Repositories
    )
    
    $validRepos = @()
    $invalidRepos = @()
    
    foreach ($repo in $Repositories) {
        $repoPath = Join-Path -Path $ROOT_DIR -ChildPath $repo
        $gitDir = Join-Path -Path $repoPath -ChildPath ".git"
        
        if (-not (Test-Path -Path $repoPath -PathType Container)) {
            Write-Host "WARNING: Repository $repo not found at $repoPath" -ForegroundColor Yellow
            $invalidRepos += $repo
            continue
        }
        
        if (-not (Test-Path -Path $gitDir -PathType Container)) {
            Write-Host "WARNING: $repo is not a git repository" -ForegroundColor Yellow
            $invalidRepos += $repo
            continue
        }
        
        $validRepos += $repo
    }
    
    return $validRepos, $invalidRepos
}

# Main execution
Write-Host "Synchronizing labelling files from $Source to repositories" -ForegroundColor Cyan
Write-Host "Root Directory: $ROOT_DIR" -ForegroundColor Cyan
Write-Host "Mode: $(if ($CheckOnly) { 'Check only' } else { 'Synchronize' }) $(if ($DryRun) { '(Dry run)' } else { '' })" -ForegroundColor Cyan
Write-Host ""

# Auto-detect repositories if not explicitly provided
if ($Target -eq "anya-core,anya-web5,anya-mobile,anya-bitcoin,dash33") {
    try {
        $detectedRepos = Get-ChildItem -Path $ROOT_DIR -Directory | 
                        Where-Object { Test-Path -Path (Join-Path -Path $_.FullName -ChildPath ".git") -PathType Container } |
                        Select-Object -ExpandProperty Name
        
        if ($detectedRepos.Count -gt 0) {
            $Target = $detectedRepos -join ","
            Write-Host "Auto-detected repositories: $Target" -ForegroundColor Green
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
                $Target = $detectedRepos -join ","
                Write-Host "Auto-detected repositories in subdirectories: $Target" -ForegroundColor Green
            } else {
                # Default to current repository only if nothing else found
                $currentFolder = Split-Path -Leaf $BASE_DIR
                $Target = $currentFolder
                Write-Host "No repositories detected. Using current repository: $Target" -ForegroundColor Yellow
            }
        }
    } catch {
        # In case of error, use current folder
        $currentFolder = Split-Path -Leaf $BASE_DIR
        $Target = $currentFolder
        Write-Host "Repository auto-detection failed. Using current repository: $Target" -ForegroundColor Yellow
    }
}

# Get list of target repositories
$targetRepos = $Target -split ','

# Validation
if ($targetRepos -notcontains $Source) {
    Write-Host "WARNING: Source repository $Source not in target list. Adding it." -ForegroundColor Yellow
    $targetRepos += $Source
}

# Validate repositories
$validRepos, $invalidRepos = Test-Repositories -Repositories $targetRepos
if ($invalidRepos.Count -gt 0) {
    Write-Host "WARNING: Skipping invalid repositories: $($invalidRepos -join ', ')" -ForegroundColor Yellow
}
$targetRepos = $validRepos

# Files to synchronize
$filesToSync = @($LABELLING_FILE, $COMMIT_RULES_FILE)

# Track repositories with changes
$changesByRepo = @{}
foreach ($repo in $targetRepos) {
    $changesByRepo[$repo] = @()
}

$reposWithDiffs = @()
$reposSynced = @()

# Process each target repository
foreach ($repo in $targetRepos) {
    if ($repo -eq $Source) {
        continue  # Skip the source repository
    }
    
    Write-Host "`nProcessing $repo..." -ForegroundColor Cyan
    $repoHasChanges = $false
    
    # Synchronize each file
    foreach ($filename in $filesToSync) {
        $hasDiff, $synced = Sync-File -SourceRepo $Source -TargetRepo $repo -Filename $filename -DryRun $DryRun -CheckOnly $CheckOnly
        
        if ($hasDiff) {
            $changesByRepo[$repo] += $filename
            $repoHasChanges = $true
        }
        
        if ($synced -and -not $CheckOnly) {
            $reposSynced += $repo
        }
    }
    
    if ($repoHasChanges) {
        $reposWithDiffs += $repo
    }
}

# Commit changes if needed
$commitSucceeded = @()
$commitFailed = @()

if (-not $CheckOnly) {
    foreach ($repo in $targetRepos) {
        if ($repo -ne $Source -and $changesByRepo[$repo].Count -gt 0) {
            $success = Commit-Changes -Repo $repo -ChangedFiles $changesByRepo[$repo] -DryRun $DryRun -NoCommit $NoCommit -BatchCommit $BatchCommit
            if ($success) {
                $commitSucceeded += $repo
            } else {
                $commitFailed += $repo
            }
        }
    }
}

# Summary
Write-Host "`nSync Summary:" -ForegroundColor Cyan
Write-Host "=============" -ForegroundColor Cyan
Write-Host "- Repositories with differences: $($reposWithDiffs.Count)" -ForegroundColor Yellow
if ($reposWithDiffs.Count -gt 0) {
    Write-Host "  - $($reposWithDiffs -join ', ')" -ForegroundColor Yellow
}

if (-not $CheckOnly) {
    $uniqueSynced = $reposSynced | Select-Object -Unique
    Write-Host "- Repositories synchronized: $($uniqueSynced.Count)" -ForegroundColor Green
    if ($uniqueSynced.Count -gt 0) {
        Write-Host "  - $($uniqueSynced -join ', ')" -ForegroundColor Green
    }
    
    if (-not $DryRun -and -not $NoCommit) {
        Write-Host "- Commits succeeded: $($commitSucceeded.Count)" -ForegroundColor Green
        if ($commitSucceeded.Count -gt 0) {
            Write-Host "  - $($commitSucceeded -join ', ')" -ForegroundColor Green
        }
        
        Write-Host "- Commits failed: $($commitFailed.Count)" -ForegroundColor $(if ($commitFailed.Count -gt 0) { "Red" } else { "Green" })
        if ($commitFailed.Count -gt 0) {
            Write-Host "  - $($commitFailed -join ', ')" -ForegroundColor Red
        }
    }
}

# Exit with status code
if ($CheckOnly -and $reposWithDiffs.Count -gt 0) {
    exit 1
}

exit 0 