# [AIR-3][AIS-3][BPC-3] Commit Message Validation Script
# This script validates that commit messages follow the AI labeling format

param (
    [string]
)

# Valid labels and their levels
\ = @(
    # Core categories
    "AIR", "AIS", "AIT", "AIM", "AIP", "AIE",
    # Extended categories
    "BPC", "RES", "SCL", "PFM", "DAO", "DID", "W5C", "UXA"
)

\ = 3  # 0-3 scale

# Skip validation for merge commits
\ = Get-Content -Path \ -Raw
if (\ -match "^Merge ") {
    exit 0
}

# Check for AI labels in commit message
\ = '\[([A-Z]{2,5})-([0-9])\]'
\ = [regex]::Matches(\, \)

if (\.Count -eq 0) {
    Write-Host "Error: Commit message must include AI labels." -ForegroundColor Red
    Write-Host "Example: [AIR-3][AIS-3][BPC-3] Implement secure SPV verification" -ForegroundColor Yellow
    exit 1
}

\ = \False

foreach (\ in \) {
    if (\.Value -match '\[([A-Z]{2,5})-([0-9])\]') {
        \ = \[1]
        \ = [int]\[2]
        
        # Check if category is valid
        if (-not (\ -contains \)) {
            Write-Host "Error: Invalid category '\' in label '\'." -ForegroundColor Red
            \ = \True
        }
        
        # Check if level is valid
        if (\ -lt 0 -or \ -gt \) {
            Write-Host "Error: Invalid level '\' in label '\'. Level must be 0-\." -ForegroundColor Red
            \ = \True
        }
    }
}

if (\) {
    exit 1
}

exit 0
