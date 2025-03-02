param (
    [Parameter(Mandatory=$false)]
    [int]$CommitThreshold = 5,
    
    [Parameter(Mandatory=$false)]
    [string]$FeatureBranchPattern = "feature/*",
    
    [Parameter(Mandatory=$false)]
    [switch]$MonitorMerges = $true,
    
    [Parameter(Mandatory=$false)]
    [switch]$MonitorCommitCount = $true,
    
    [Parameter(Mandatory=$false)]
    [switch]$PushToRemote = $false,
    
    [Parameter(Mandatory=$false)]
    [string]$RepositoryPath = ""
)

function Get-AiLabel {
    param (
        [string]$CommitMessage
    )
    
    $aiLabels = @("AIR", "AIS", "AIT", "AIM", "AIP", "AIE")
    foreach ($label in $aiLabels) {
        if ($CommitMessage -match "\[$label-(\d{3})\]") {
            return "$label-$($matches[1])"
        }
    }
    
    return ""
}

function Create-Checkpoint {
    param (
        [string]$Name,
        [string]$Message,
        [string]$AiLabel = "",
        [switch]$Push = $false
    )
    
    if ($RepositoryPath -ne "") {
        Push-Location $RepositoryPath
    }
    
    $scriptPath = Join-Path $PSScriptRoot "create_checkpoint.ps1"
    
    $params = @{
        CheckpointName = $Name
        Message = $Message
    }
    
    if ($AiLabel -ne "") {
        $params.Add("AiLabel", $AiLabel)
    }
    
    if ($Push) {
        & $scriptPath @params -PushToRemote
    } else {
        & $scriptPath @params
    }
    
    if ($RepositoryPath -ne "") {
        Pop-Location
    }
}

# Change to the specified repository directory if provided
if ($RepositoryPath -ne "") {
    Push-Location $RepositoryPath
}

# Get the latest commit hash
$latestCommit = git rev-parse HEAD

# Get the latest merge commit if monitoring merges
if ($MonitorMerges) {
    $latestMergeCommit = git log --merges -n 1 --pretty=format:"%H"
    
    if ($latestMergeCommit -eq $latestCommit) {
        # This is a merge commit
        $mergeMessage = git log -1 --pretty=format:"%s"
        $aiLabel = Get-AiLabel -CommitMessage $mergeMessage
        
        # Extract branch names
        $branchInfo = git log -1 --pretty=format:"%b"
        $sourceBranch = ""
        if ($branchInfo -match "Merge branch '([^']+)'") {
            $sourceBranch = $matches[1]
        }
        
        if ($sourceBranch -ne "" -and $sourceBranch -like $FeatureBranchPattern) {
            $featureName = $sourceBranch -replace 'feature/', ''
            $checkpointName = "merge_$featureName"
            $checkpointMessage = "Automated checkpoint after merging feature branch '$sourceBranch'"
            
            Create-Checkpoint -Name $checkpointName -Message $checkpointMessage -AiLabel $aiLabel -Push:$PushToRemote
        }
    }
}

# Monitor commit count if enabled
if ($MonitorCommitCount) {
    # Check the latest checkpoint file to get its commit
    $checkpointFiles = Get-ChildItem -Path "docs/checkpoints" -Filter "*.md" -ErrorAction SilentlyContinue | 
                        Sort-Object LastWriteTime -Descending
    
    if ($checkpointFiles.Count -gt 0) {
        $lastCheckpointFile = $checkpointFiles[0]
        $lastCheckpointContent = Get-Content $lastCheckpointFile.FullName
        $lastCommitLine = $lastCheckpointContent | Where-Object { $_ -match "^Commit: ([a-f0-9]+)$" }
        
        if ($lastCommitLine -match "Commit: ([a-f0-9]+)") {
            $lastCheckpointCommit = $matches[1]
            
            # Count commits since last checkpoint
            $commitCount = git rev-list --count "$lastCheckpointCommit..$latestCommit"
            
            if ($commitCount -ge $CommitThreshold) {
                $checkpointName = "auto_commit_threshold"
                $checkpointMessage = "Automated checkpoint after $commitCount new commits"
                
                # Get the AI label from the latest commit if available
                $latestCommitMessage = git log -1 --pretty=format:"%s"
                $aiLabel = Get-AiLabel -CommitMessage $latestCommitMessage
                
                Create-Checkpoint -Name $checkpointName -Message $checkpointMessage -AiLabel $aiLabel -Push:$PushToRemote
            }
        }
    }
}

# Return to the original directory if a repository path was specified
if ($RepositoryPath -ne "") {
    Pop-Location
}
