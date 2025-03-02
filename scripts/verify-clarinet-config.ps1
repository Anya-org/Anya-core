# Verify and Fix Clarinet Configuration
# This script checks and fixes the Clarinet.toml configuration to ensure all contracts are properly linked

# Configuration
$clarinetTomlPath = "$PSScriptRoot\..\Clarinet.toml"
$contractsToCheck = @(
    @{
        Name = "governance-token"
        Path = "src/contracts/governance_token.clar"
    },
    @{
        Name = "dao"
        Path = "src/contracts/dao.clar"
    },
    @{
        Name = "dao-trait"
        Path = "dao/traits/dao-trait.clar"
    },
    @{
        Name = "dao-core"
        Path = "dao/core/dao-core.clar"
    }
)

Write-Host "Verifying Clarinet.toml configuration..." -ForegroundColor Cyan

# Check if Clarinet.toml exists
if (-not (Test-Path $clarinetTomlPath)) {
    Write-Host "Error: Clarinet.toml not found at $clarinetTomlPath" -ForegroundColor Red
    exit 1
}

# Read Clarinet.toml content
$clarinetToml = Get-Content $clarinetTomlPath -Raw

# Check if [contracts] section exists
if (-not ($clarinetToml -match "\[contracts\]")) {
    Write-Host "Error: [contracts] section not found in Clarinet.toml" -ForegroundColor Red
    exit 1
}

# Parse the [contracts] section
$contractsSection = [regex]::Match($clarinetToml, "\[contracts\](.*?)(\[|$)", [System.Text.RegularExpressions.RegexOptions]::Singleline).Groups[1].Value

# Check each contract
$missingContracts = @()
foreach ($contract in $contractsToCheck) {
    $contractName = $contract.Name
    $contractPath = $contract.Path
    
    # Check if contract exists in Clarinet.toml
    if (-not ($contractsSection -match "$contractName\s*=\s*\{path\s*=\s*[""']$([regex]::Escape($contractPath))[""']")) {
        $missingContracts += $contract
    }
    
    # Check if contract file exists
    $fullPath = Join-Path -Path $PSScriptRoot -ChildPath "..\$contractPath"
    if (-not (Test-Path $fullPath)) {
        Write-Host "Warning: Contract file not found: $fullPath" -ForegroundColor Yellow
        
        # Check if directory exists
        $directory = Split-Path -Path $fullPath -Parent
        if (-not (Test-Path $directory)) {
            Write-Host "Creating directory: $directory" -ForegroundColor Yellow
            New-Item -Path $directory -ItemType Directory -Force | Out-Null
        }
    }
}

# Fix missing contracts if needed
if ($missingContracts.Count -gt 0) {
    Write-Host "Found $($missingContracts.Count) missing contract(s) in Clarinet.toml" -ForegroundColor Yellow
    
    # Create backup
    $backupPath = "$clarinetTomlPath.bak"
    Copy-Item -Path $clarinetTomlPath -Destination $backupPath -Force
    Write-Host "Created backup at $backupPath" -ForegroundColor Green
    
    # Update Clarinet.toml
    $updatedContractsSection = "[contracts]`n"
    foreach ($contract in $contractsToCheck) {
        $updatedContractsSection += "$($contract.Name) = {path = `"$($contract.Path)`"}`n"
    }
    
    # Replace the contracts section
    $updatedToml = $clarinetToml -replace "\[contracts\](.*?)(\[|$)", "$updatedContractsSection`n[", [System.Text.RegularExpressions.RegexOptions]::Singleline
    
    # Write updated Clarinet.toml
    Set-Content -Path $clarinetTomlPath -Value $updatedToml
    Write-Host "Updated Clarinet.toml with missing contracts" -ForegroundColor Green
} else {
    Write-Host "All contracts are properly configured in Clarinet.toml" -ForegroundColor Green
}

# Verify contract files
foreach ($contract in $contractsToCheck) {
    $fullPath = Join-Path -Path $PSScriptRoot -ChildPath "..\$($contract.Path)"
    if (Test-Path $fullPath) {
        Write-Host "Contract file exists: $($contract.Path)" -ForegroundColor Green
    } else {
        Write-Host "Contract file missing: $($contract.Path)" -ForegroundColor Red
    }
}

Write-Host "Verification completed." -ForegroundColor Cyan 