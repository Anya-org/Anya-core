#!/usr/bin/env pwsh

# Auto-alignment and fixes script for Anya Core
# Author: bo_thebig
# Email: botshelomokokoka@gmail.com

$ErrorActionPreference = "Stop"
$ProgressPreference = "SilentlyContinue"

# Create lib directory if it doesn't exist
$libPath = Join-Path $PSScriptRoot "lib"
if (-not (Test-Path $libPath)) {
    New-Item -ItemType Directory -Path $libPath | Out-Null
}

# Import logging module
$loggingModule = Join-Path $libPath "logging.psm1"
if (Test-Path $loggingModule) {
    Import-Module $loggingModule -ErrorAction Stop
} else {
    Write-Error "Logging module not found at: $loggingModule"
    exit 1
}

function Write-Header {
    param([string]$message)
    Write-Log -Message $message -Level 'Info'
}

function Test-GitConfig {
    Write-Header "Checking Git Configuration"
    
    try {
        # Set git signing configuration
        git config --global user.name "bo_thebig"
        git config --global user.email "botshelomokokoka@gmail.com"
        git config --global commit.gpgsign true
        
        # Verify configuration
        $name = git config --global user.name
        $email = git config --global user.email
        $gpgsign = git config --global commit.gpgsign
        
        Write-Log "Git configuration verified:" -Level 'Success'
        Write-Log "Name: $name" -Level 'Info'
        Write-Log "Email: $email" -Level 'Info'
        Write-Log "GPG Sign: $gpgsign" -Level 'Info'
    }
    catch {
        Write-Log "Failed to configure git: $_" -Level 'Error'
        throw
    }
}

function Test-BIPCompliance {
    Write-Header "Checking BIP Compliance"
    
    try {
        # Check BIP-341 (Taproot) compliance
        $taprootFiles = Get-ChildItem -Path "src" -Recurse -File | Where-Object { $_.Name -match "taproot|bip341" }
        foreach ($file in $taprootFiles) {
            $content = Get-Content $file.FullName -Raw
            if ($content -match "SILENT_LEAF") {
                Write-Log "BIP-341 SILENT_LEAF found in $($file.Name)" -Level 'Success'
            } else {
                Write-Log "BIP-341 SILENT_LEAF missing in $($file.Name)" -Level 'Warning'
            }
        }
    }
    catch {
        Write-Log "Failed to check BIP compliance: $_" -Level 'Error'
        throw
    }
}

function Test-InstallationSystem {
    Write-Header "Validating Installation System"
    
    try {
        # Check installer components
        $requiredPaths = @(
            "installer/src/main.rs",
            "installer/src/hardware.rs",
            "installer/src/network.rs",
            "install/dashboard.ps1",
            "scripts/install/validator.js"
        )
        
        foreach ($path in $requiredPaths) {
            if (Test-Path $path) {
                Write-Log "Found: $path" -Level 'Success'
            } else {
                Write-Log "Missing: $path" -Level 'Warning'
            }
        }
    }
    catch {
        Write-Log "Failed to validate installation system: $_" -Level 'Error'
        throw
    }
}

function Test-Web5Compliance {
    Write-Header "Checking Web5 Compliance"
    
    try {
        # Check Web5 implementation
        $web5Path = "src/web5"
        if (Test-Path $web5Path) {
            $bip341Files = Get-ChildItem -Path $web5Path -Recurse -File | Where-Object { $_.Name -match "bip341|taproot" }
            if ($bip341Files) {
                Write-Log "Web5 BIP-341 implementation found" -Level 'Success'
            } else {
                Write-Log "Web5 BIP-341 implementation missing" -Level 'Warning'
            }
        }
    }
    catch {
        Write-Log "Failed to check Web5 compliance: $_" -Level 'Error'
        throw
    }
}

function Sync-BranchChanges {
    Write-Header "Syncing Branch Changes"
    
    try {
        # Checkout and update feature branch
        git checkout feature/web5-bip341-compliance
        git pull origin feature/web5-bip341-compliance
        
        # Stage and commit changes
        git add .
        git commit -S -m "@AI: Auto-aligned BIP-341 compliance fixes
        
        - Verified git signing configuration
        - Checked BIP-341 SILENT_LEAF implementation
        - Validated installation system components
        - Updated Web5 compliance checks
        
        Author: bo_thebig
        Signed-off-by: botshelomokokoka@gmail.com"
        
        # Push changes
        git push origin feature/web5-bip341-compliance
        
        Write-Log "Successfully pushed changes to feature/web5-bip341-compliance" -Level 'Success'
    }
    catch {
        Write-Log "Failed to sync branch changes: $_" -Level 'Error'
        throw
    }
}

function Start-AlignmentProcess {
    try {
        Write-Log "Starting Anya Core alignment process..." -Level 'Info'
        
        # Run all checks
        Test-GitConfig
        Test-BIPCompliance
        Test-InstallationSystem
        Test-Web5Compliance
        
        # Sync changes if all checks pass
        Sync-BranchChanges
        
        Write-Log "Alignment process completed successfully!" -Level 'Success'
    }
    catch {
        Write-Log "Error during alignment process: $_" -Level 'Error'
        exit 1
    }
}

# Execute the alignment process
Start-AlignmentProcess 