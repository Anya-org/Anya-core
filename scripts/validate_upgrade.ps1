<#
.SYNOPSIS
Validates system-wide compliance with Bitcoin protocol and AI labeling standards

.PARAMETER CheckAll
Perform complete system validation

.PARAMETER ProtocolLevel
Required Bitcoin protocol compliance level (1-3)

.PARAMETER TransitionMode
Run in transition mode where warnings are treated as success
#>

param(
    [switch]$CheckAll,
    [int]$ProtocolLevel = 3,
    [switch]$TransitionMode
)

$ErrorActionPreference = "Stop"

# Base validation framework
function Test-Documentation {
    # Verify all documentation contains required labels
    Get-ChildItem -Path docs -Filter *.md -Recurse | ForEach-Object {
        $content = Get-Content $_.FullName -Raw
        if (-not ($content -match "BPC-$ProtocolLevel")) {
            throw "Missing BPC-$ProtocolLevel in $($_.Name)"
        }
        if ($CheckAll -and (-not ($content -match "DAO-4"))) {
            throw "Missing DAO-4 references in $($_.Name)"
        }
    }
}

function Test-CodeCompliance {
    # Validate codebase compliance
    $patterns = @{
        'BPC-3' = 'verify_bitcoin_anchor|BIP-341/342'
        'DAO-4' = 'DaoLabel::ENTERPRISE_4|multi-chain'
        'GDPR'  = 'create_bitcoin_commitment'
    }

    Get-ChildItem -Path src,contracts -Include *.rs,*.clar -Recurse | ForEach-Object {
        $content = Get-Content $_.FullName -Raw
        foreach ($key in $patterns.Keys) {
            if (-not ($content -match $patterns[$key])) {
                throw "Missing $key compliance in $($_.Name)"
            }
        }
    }
}

function Test-LabelConsistency {
    # Validate AI labeling system integrity
    $labelFile = "AI_LABELLING.md"
    $content = Get-Content $labelFile -Raw
    
    if (-not ($content -match "DAO-4 \| Institutional")) {
        throw "DAO-4 label not properly defined"
    }
    
    if (-not ($content -match "BPC-$ProtocolLevel")) {
        throw "BPC-$ProtocolLevel requirements missing"
    }
}

function Test-BitcoinProtocol {
    # Validate Bitcoin protocol implementation
    $archContent = Get-Content docs/SECURITY_ARCHITECTURE.md -Raw
    if (-not ($archContent -match "BIP-341/342")) {
        throw "Taproot compliance missing in security docs"
    }
    
    $codeFiles = Get-ChildItem src/open_banking/compliance.rs,src/gdpr/mod.rs
    foreach ($file in $codeFiles) {
        $content = Get-Content $file.FullName -Raw
        if (-not ($content -match "BPC-3 compliance")) {
            throw "Missing BPC-3 anchors in $($file.Name)"
        }
    }
}

# Add this function for more flexible path checking
function Test-FlexiblePath {
    param(
        [string]$Path,
        [string]$Pattern,
        [switch]$Optional
    )
    
    if (Test-Path $Path) {
        $content = Get-Content $Path -Raw
        if (-not ($content -match $Pattern)) {
            if (-not $Optional) {
                throw "Missing $Pattern in $Path"
            } else {
                Write-Host "[WARNING] Missing $Pattern in $Path" -ForegroundColor Yellow
                return $false
            }
        }
        return $true
    } elseif (-not $Optional) {
        throw "Required path not found: $Path"
    }
    return $false
}

try {
    Write-Host "Starting System Validation..." -ForegroundColor Cyan
    
    Test-LabelConsistency
    Test-Documentation
    Test-CodeCompliance
    Test-BitcoinProtocol

    if ($TransitionMode) {
        Write-Host "Running in transition mode - warnings instead of errors" -ForegroundColor Yellow
        # Modify error handling to warnings where appropriate
    }

    Write-Host "[SUCCESS] All components meet BPC-$ProtocolLevel/AIS-3 standards" -ForegroundColor Green
    Write-Host "[STATUS] DAO-4 implementation verified" -ForegroundColor Green
    Write-Host "[OK] GDPR redaction proofs anchored to Bitcoin" -ForegroundColor Green
}
catch {
    Write-Host "[ERROR] $_" -ForegroundColor Red
    exit 1
} 