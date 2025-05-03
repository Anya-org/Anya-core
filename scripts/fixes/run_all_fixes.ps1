# Master script to execute all error fixes for Anya Core
# Ensures alignment with Bitcoin Core principles (decentralization, security, immutability, transparency)

param(
    [switch]$DryRun,
    [switch]$FixYaml = $true,
    [switch]$FixDocComments = $true,
    [switch]$FixBitcoinImports = $true,
    [switch]$FixValidationSyntax = $true,
    [switch]$SkipTests = $false
)

$scriptName = "Anya Core Error Fixing Suite"
$scriptVersion = "1.0.0"
$scriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$projectRoot = Split-Path -Parent (Split-Path -Parent $scriptRoot)

# Colors for logging
$infoColor = "Cyan"
$successColor = "Green"
$warningColor = "Yellow"
$errorColor = "Red"
$headerColor = "Magenta"

Write-Host "===============================================================" -ForegroundColor $headerColor
Write-Host "            $scriptName v$scriptVersion" -ForegroundColor $headerColor
Write-Host "===============================================================" -ForegroundColor $headerColor
Write-Host "Starting comprehensive error fixing aligned with Bitcoin Core principles..."
Write-Host "Project root: $projectRoot"
Write-Host "Run mode: $(if ($DryRun) { 'Dry run (no changes will be made)' } else { 'Live run (changes will be applied)' })" -ForegroundColor $(if ($DryRun) { $warningColor } else { $infoColor })
Write-Host ""

$ErrorActionPreference = "Continue" # Don't stop on errors

# Scripts to run and their descriptions
$fixScripts = @(
    @{
        Name = "YAML Workflow Fixes"
        Path = Join-Path $scriptRoot "fix_yaml_workflows.ps1"
        Description = "Fixes YAML syntax errors in GitHub workflow files"
        Enabled = $FixYaml
        BtcPrinciples = @("Integrity", "Transparency")
    },
    @{
        Name = "Documentation Structure Fixes"
        Path = Join-Path $scriptRoot "fix_doc_comments.ps1"
        Description = "Fixes documentation comment structure issues"
        Enabled = $FixDocComments
        BtcPrinciples = @("Clarity", "Transparency")
    },
    @{
        Name = "Bitcoin Import Fixes"
        Path = Join-Path $scriptRoot "fix_bitcoin_imports.ps1"
        Description = "Fixes Bitcoin import errors for proper Bitcoin Core alignment"
        Enabled = $FixBitcoinImports
        BtcPrinciples = @("Compatibility", "Security")
    },
    @{
        Name = "Validation Syntax Fixes"
        Path = Join-Path $scriptRoot "fix_validation_syntax.ps1"
        Description = "Fixes the unclosed delimiter issue in validation.rs"
        Enabled = $FixValidationSyntax
        BtcPrinciples = @("Integrity", "Security")
    }
)

# Track overall results
$results = @{
    Succeeded = 0
    Failed = 0
    Skipped = 0
}

foreach ($script in $fixScripts) {
    if (-not $script.Enabled) {
        Write-Host "SKIPPED: $($script.Name) - Disabled by parameter" -ForegroundColor $warningColor
        $results.Skipped++
        continue
    }
    
    if (-not (Test-Path $script.Path)) {
        Write-Host "ERROR: $($script.Name) - Script file not found at $($script.Path)" -ForegroundColor $errorColor
        $results.Failed++
        continue
    }
    
    # Display information about the script
    Write-Host "===============================================================" -ForegroundColor $infoColor
    Write-Host "Running: $($script.Name)" -ForegroundColor $infoColor
    Write-Host "Description: $($script.Description)" -ForegroundColor $infoColor
    Write-Host "Bitcoin Core Principles: $($script.BtcPrinciples -join ', ')" -ForegroundColor $infoColor
    Write-Host "---------------------------------------------------------------" -ForegroundColor $infoColor
    
    try {
        # Run the script with the appropriate dry run parameter
        $scriptParams = @{}
        if ($DryRun) {
            $scriptParams["DryRun"] = $true
        }
        
        & $script.Path @scriptParams
        
        # Check if the script was successful
        if ($LASTEXITCODE -eq 0 -or $null -eq $LASTEXITCODE) {
            Write-Host "SUCCESS: $($script.Name) completed successfully" -ForegroundColor $successColor
            $results.Succeeded++
        } else {
            Write-Host "ERROR: $($script.Name) failed with exit code $LASTEXITCODE" -ForegroundColor $errorColor
            $results.Failed++
        }
    } catch {
        Write-Host "ERROR: $($script.Name) threw an exception: $_" -ForegroundColor $errorColor
        $results.Failed++
    }
    
    Write-Host ""
}

# Run tests if not skipped
if (-not $SkipTests) {
    Write-Host "===============================================================" -ForegroundColor $headerColor
    Write-Host "            Running Verification Tests" -ForegroundColor $headerColor
    Write-Host "===============================================================" -ForegroundColor $headerColor
    
    $testScriptPath = Join-Path $projectRoot "unified-test-system.ps1"
    if (Test-Path $testScriptPath) {
        Write-Host "Running unified test system to verify fixes..." -ForegroundColor $infoColor
        
        try {
            & $testScriptPath -TargetModule "BitcoinCore" -SkipLongRunning
            
            if ($LASTEXITCODE -eq 0 -or $null -eq $LASTEXITCODE) {
                Write-Host "SUCCESS: Verification tests completed successfully" -ForegroundColor $successColor
            } else {
                Write-Host "WARNING: Verification tests completed with issues (exit code: $LASTEXITCODE)" -ForegroundColor $warningColor
                Write-Host "         Some errors may require manual intervention" -ForegroundColor $warningColor
            }
        } catch {
            Write-Host "ERROR: Verification tests failed with exception: $_" -ForegroundColor $errorColor
        }
    } else {
        Write-Host "WARNING: Unified test system not found at $testScriptPath" -ForegroundColor $warningColor
        Write-Host "         Skipping verification tests" -ForegroundColor $warningColor
    }
}

# Print summary
Write-Host "===============================================================" -ForegroundColor $headerColor
Write-Host "                       Summary" -ForegroundColor $headerColor
Write-Host "===============================================================" -ForegroundColor $headerColor
Write-Host "Total scripts: $($fixScripts.Count)" -ForegroundColor $infoColor
Write-Host "Succeeded:    $($results.Succeeded)" -ForegroundColor $successColor
Write-Host "Failed:       $($results.Failed)" -ForegroundColor $(if ($results.Failed -gt 0) { $errorColor } else { $infoColor })
Write-Host "Skipped:      $($results.Skipped)" -ForegroundColor $(if ($results.Skipped -gt 0) { $warningColor } else { $infoColor })

if ($DryRun) {
    Write-Host "`nNote: This was a dry run. No actual changes were made." -ForegroundColor $warningColor
    Write-Host "      To apply the fixes, run this script without the -DryRun parameter." -ForegroundColor $warningColor
}

if ($results.Failed -gt 0) {
    Write-Host "`nSome fixes failed. See the logs above for details." -ForegroundColor $errorColor
    Write-Host "Manual intervention may be required for those issues." -ForegroundColor $errorColor
    
    # List likely issues that may need manual fixing
    Write-Host "`nCommon issues that may need manual fixing:" -ForegroundColor $warningColor
    Write-Host "1. Layer2 protocol imports in lightning, rgb, and taproot_assets modules" -ForegroundColor $warningColor
    Write-Host "2. RSK error handling for PoisonError conversions" -ForegroundColor $warningColor
    Write-Host "3. Bitflags dependency issues in network and script modules" -ForegroundColor $warningColor
    Write-Host "4. Type size issues with [u8] in script implementations" -ForegroundColor $warningColor
}

# Return the overall success/failure
if ($results.Failed -gt 0) {
    exit 1
} else {
    exit 0
}
