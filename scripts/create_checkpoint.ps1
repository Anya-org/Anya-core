param (
    [Parameter(Mandatory=$true)]
    [string]$CheckpointName,
    
    [Parameter(Mandatory=$false)]
    [string]$Message = "Automated checkpoint",
    
    [Parameter(Mandatory=$false)]
    [string]$AiLabel = "",
    
    [Parameter(Mandatory=$false)]
    [switch]$PushToRemote = $false
)

# Validate AI label format if provided
if ($AiLabel -ne "") {
    $validLabels = @("AIR", "AIS", "AIT", "AIM", "AIP", "AIE")
    $labelPattern = "^($($validLabels -join '|'))-\d{3}$"
    
    if (-not ($AiLabel -match $labelPattern)) {
        Write-Error "Invalid AI label format. Must be one of $($validLabels -join ', ') followed by a 3-digit number (e.g., AIP-001)"
        exit 1
    }
    
    # Add AI label to checkpoint name and message
    $CheckpointName = "$AiLabel-$CheckpointName"
    $Message = "[$AiLabel] $Message"
}

# Sanitize checkpoint name for tag
$tagName = $CheckpointName -replace '[^a-zA-Z0-9_\-\.]', '_'

# Get current date-time
$timestamp = Get-Date -Format "yyyy-MM-dd_HH-mm-ss"

# Create a tag with message
Write-Host "Creating Git tag: $tagName" -ForegroundColor Green
git tag -a "$tagName" -m "$Message (Created at $timestamp)"

# Create a checkpoint file with details
$checkpointFile = "docs/checkpoints/$tagName-$timestamp.md"

# Create checkpoints directory if it doesn't exist
if (-not (Test-Path "docs/checkpoints")) {
    New-Item -ItemType Directory -Path "docs/checkpoints" -Force | Out-Null
}

# Generate checkpoint content
$content = @"
# Checkpoint: $CheckpointName
**Created**: $timestamp
**AI Label**: $AiLabel
**Message**: $Message

## Commit Information
$(git log -1 --pretty=format:"Commit: %H%nAuthor: %an <%ae>%nDate: %ad%n%n%s%n%n%b")

## Files Changed in Last Commit
$(git show --name-status HEAD)

## Repository Status at Checkpoint
$(git status)
"@

# Write content to file
$content | Out-File -FilePath $checkpointFile -Encoding utf8

Write-Host "Created checkpoint file: $checkpointFile" -ForegroundColor Green

# Push to remote if requested
if ($PushToRemote) {
    Write-Host "Pushing tags to remote..." -ForegroundColor Yellow
    git push origin $tagName
    
    # Commit the checkpoint file
    git add $checkpointFile
    git commit -m "Add checkpoint documentation for $tagName"
    git push origin HEAD
    
    Write-Host "Pushed checkpoint $tagName to remote" -ForegroundColor Green
}

Write-Host "Checkpoint creation completed successfully" -ForegroundColor Cyan
