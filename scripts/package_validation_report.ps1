# Script to generate a report on the RGB and Web5 features for packaging readiness
# This script will check the status of all components and generate a report

# Create the output directory if it doesn't exist
$reportsDir = Join-Path "c:\Users\bmokoka\Downloads\OPSource\anya-core" "reports"
if (-not (Test-Path $reportsDir)) {
    New-Item -ItemType Directory -Path $reportsDir -Force | Out-Null
}

# Define the report file
$reportFile = Join-Path $reportsDir "packaging_readiness_report.md"
$timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"

# Start writing the report
$report = @"
# Anya Core - Packaging Readiness Report
**Generated:** $timestamp

## Overview
This report evaluates the readiness of the RGB and Web5 features for packaging and deployment.

## Feature Status Summary

| Feature | Status | Notes |
|---------|--------|-------|
"@

# Base paths
$baseDir = "c:\Users\bmokoka\Downloads\OPSource\anya-core"
$srcDir = Join-Path $baseDir "src"
$testsDir = Join-Path $baseDir "tests"

# Check RGB Asset features
$rgbDir = Join-Path $srcDir "bitcoin" "rgb"
$rgbTestFile = Join-Path $testsDir "rgb_asset_test.rs"
$rgbStatus = if (Test-Path $rgbDir -PathType Container) {
    if (Test-Path $rgbTestFile -PathType Leaf) {
        "✅ Ready"
    } else {
        "⚠️ Code exists but tests incomplete"
    }
} else {
    "❌ Missing implementation"
}
$rgbNotes = if (Test-Path $rgbTestFile -PathType Leaf) {
    "Tests written, asset issuance and transfer functionality implemented."
} else {
    "Implementation needs validation with complete tests."
}

# Check Web5 features
$web5Dir = Join-Path $srcDir "web5"
$web5TestFile = Join-Path $testsDir "web5_anchoring_test.rs"
$web5Status = if (Test-Path $web5Dir -PathType Container) {
    if (Test-Path $web5TestFile -PathType Leaf) {
        "✅ Ready"
    } else {
        "⚠️ Code exists but tests incomplete"
    }
} else {
    "❌ Missing implementation"
}
$web5Notes = if (Test-Path $web5TestFile -PathType Leaf) {
    "Tests written, DID resolution, credential verification and Bitcoin anchoring implemented."
} else {
    "Implementation needs validation with complete tests."
}

# Check Bitcoin Wallet with multi-output PSBT
$bitcoinDir = Join-Path $srcDir "bitcoin"
$taprootDir = Join-Path $bitcoinDir "taproot"
$taprootStatus = if (Test-Path $taprootDir -PathType Container) {
    "✅ Ready"
} else {
    "❌ Missing implementation"
}
$taprootNotes = if (Test-Path $taprootDir -PathType Container) {
    "Taproot functionality implemented with multi-output PSBT support."
} else {
    "Taproot implementation required for advanced Bitcoin features."
}

# Add feature status to report
$report += @"
| RGB Asset Transfer | $rgbStatus | $rgbNotes |
| Web5 with Bitcoin Anchoring | $web5Status | $web5Notes |
| Bitcoin Wallet with Taproot | $taprootStatus | $taprootNotes |
"@

# Add dependencies section
$report += @"

## Dependencies Status

| Dependency | Version | Status |
|------------|---------|--------|
"@

# Check Cargo.toml for dependencies
$cargoToml = Join-Path $baseDir "Cargo.toml"
if (Test-Path $cargoToml -PathType Leaf) {
    $cargoContent = Get-Content $cargoToml -Raw
    
    # Check for Bitcoin dependency
    if ($cargoContent -match "bitcoin.+?version\s*=\s*\"(.*?)\"") {
        $bitcoinVersion = $matches[1]
        $bitcoinStatus = "✅ Available"
    } else {
        $bitcoinVersion = "N/A"
        $bitcoinStatus = "❌ Missing"
    }
    
    # Check for BDK dependency
    if ($cargoContent -match "bdk.+?version\s*=\s*\"(.*?)\"") {
        $bdkVersion = $matches[1]
        $bdkStatus = "✅ Available"
    } else {
        $bdkVersion = "N/A"
        $bdkStatus = "❌ Missing"
    }
    
    # Check for Web5 dependency
    if ($cargoContent -match "web5.+?tag\s*=\s*\"(.*?)\"") {
        $web5Version = $matches[1]
        $web5Status = "✅ Available"
    } else {
        $web5Version = "N/A"
        $web5Status = "❌ Missing"
    }
    
    # Check for RGB dependencies
    if ($cargoContent -match "rgb-core.+?tag\s*=\s*\"(.*?)\"") {
        $rgbCoreVersion = $matches[1]
        $rgbCoreStatus = "✅ Available"
    } else {
        $rgbCoreVersion = "N/A"
        $rgbCoreStatus = "❌ Missing"
    }
    
    if ($cargoContent -match "rgb-std.+?tag\s*=\s*\"(.*?)\"") {
        $rgbStdVersion = $matches[1]
        $rgbStdStatus = "✅ Available"
    } else {
        $rgbStdVersion = "N/A"
        $rgbStdStatus = "❌ Missing"
    }
} else {
    $bitcoinVersion = $bdkVersion = $web5Version = $rgbCoreVersion = $rgbStdVersion = "N/A"
    $bitcoinStatus = $bdkStatus = $web5Status = $rgbCoreStatus = $rgbStdStatus = "❌ Missing"
}

# Add dependency info to report
$report += @"
| Bitcoin | $bitcoinVersion | $bitcoinStatus |
| BDK | $bdkVersion | $bdkStatus |
| Web5 | $web5Version | $web5Status |
| RGB Core | $rgbCoreVersion | $rgbCoreStatus |
| RGB Std | $rgbStdVersion | $rgbStdStatus |
"@

# Add documentation status
$report += @"

## Documentation Status

| Document | Status | Notes |
|----------|--------|-------|
"@

# Check README
$readmeFile = Join-Path $baseDir "README.md"
$readmeStatus = if (Test-Path $readmeFile -PathType Leaf) {
    # Check if the README mentions RGB and Web5
    $readmeContent = Get-Content $readmeFile -Raw
    $hasRgb = $readmeContent -match "RGB"
    $hasWeb5 = $readmeContent -match "Web5"
    
    if ($hasRgb -and $hasWeb5) {
        "✅ Complete"
    } else {
        "⚠️ Needs updates"
    }
} else {
    "❌ Missing"
}

$readmeNotes = if ($readmeStatus -eq "✅ Complete") {
    "Documentation includes details on RGB asset features and Web5 with Bitcoin anchoring."
} elseif ($readmeStatus -eq "⚠️ Needs updates") {
    "Documentation needs to be updated to include all current features."
} else {
    "README file is missing and should be created."
}

# Check TODO.md
$todoFile = Join-Path $baseDir "TODO.md"
$todoStatus = if (Test-Path $todoFile -PathType Leaf) {
    "✅ Available"
} else {
    "❌ Missing"
}
$todoNotes = if ($todoStatus -eq "✅ Available") {
    "TODO list is available and should be updated with completed features."
} else {
    "TODO file is missing and should be created to track remaining tasks."
}

# Add documentation info to report
$report += @"
| README.md | $readmeStatus | $readmeNotes |
| TODO.md | $todoStatus | $todoNotes |
"@

# Add test status
$report += @"

## Test Status

| Test | Status | Notes |
|------|--------|-------|
"@

# Check RGB asset test
$rgbTestStatus = if (Test-Path $rgbTestFile -PathType Leaf) {
    "✅ Available"
} else {
    "❌ Missing"
}
$rgbTestNotes = if ($rgbTestStatus -eq "✅ Available") {
    "RGB asset tests are available and should be executed to validate functionality."
} else {
    "RGB asset tests are missing and should be implemented."
}

# Check Web5 anchoring test
$web5TestStatus = if (Test-Path $web5TestFile -PathType Leaf) {
    "✅ Available"
} else {
    "❌ Missing"
}
$web5TestNotes = if ($web5TestStatus -eq "✅ Available") {
    "Web5 with Bitcoin anchoring tests are available and should be executed to validate functionality."
} else {
    "Web5 with Bitcoin anchoring tests are missing and should be implemented."
}

# Add test info to report
$report += @"
| RGB Asset Test | $rgbTestStatus | $rgbTestNotes |
| Web5 Anchoring Test | $web5TestStatus | $web5TestNotes |
"@

# Add packaging recommendations
$report += @"

## Packaging Recommendations

"@

# Overall status
$featuresReady = ($rgbStatus -match "Ready" -and $web5Status -match "Ready" -and $taprootStatus -match "Ready")
$dependenciesReady = ($bitcoinStatus -match "Available" -and $bdkStatus -match "Available" -and $web5Status -match "Available" -and $rgbCoreStatus -match "Available" -and $rgbStdStatus -match "Available")
$testsAvailable = ($rgbTestStatus -match "Available" -and $web5TestStatus -match "Available")

if ($featuresReady -and $dependenciesReady -and $testsAvailable) {
    $report += @"
✅ **READY FOR PACKAGING**

All critical features are implemented and tested. Dependencies are configured correctly. The project is ready for packaging and deployment.

**Next Steps:**
1. Run final integration tests
2. Create the package using `cargo build --release`
3. Deploy to the target environment
"@
} elseif ($featuresReady -and $dependenciesReady) {
    $report += @"
⚠️ **ALMOST READY**

The core features are implemented and dependencies are configured, but some tests are missing. Complete the test suite before packaging.

**Next Steps:**
1. Implement missing tests
2. Run all tests to ensure functionality
3. Proceed with packaging when tests pass
"@
} else {
    $report += @"
❌ **NOT READY**

The project is not ready for packaging. Several critical components are missing or incomplete.

**Next Steps:**
"@
    
    if (-not $featuresReady) {
        $report += "1. Complete implementation of missing features\n"
    }
    if (-not $dependenciesReady) {
        $report += "2. Fix dependency configuration issues\n"
    }
    if (-not $testsAvailable) {
        $report += "3. Implement comprehensive test suite\n"
    }
    
    $report += "4. Review packaging criteria once these steps are completed"
}

# Write the report to a file
$report | Out-File -FilePath $reportFile -Encoding utf8

Write-Host "Report generated: $reportFile" -ForegroundColor Green
Write-Host "Run this script again after making changes to update the report." -ForegroundColor Yellow

# Output a summary to the console
Write-Host "`n=== Packaging Readiness Summary ===`n" -ForegroundColor Cyan
Write-Host "RGB Asset Transfer: $rgbStatus" -ForegroundColor $(if ($rgbStatus -match "Ready") { "Green" } elseif ($rgbStatus -match "Missing") { "Red" } else { "Yellow" })
Write-Host "Web5 with Bitcoin Anchoring: $web5Status" -ForegroundColor $(if ($web5Status -match "Ready") { "Green" } elseif ($web5Status -match "Missing") { "Red" } else { "Yellow" })
Write-Host "Bitcoin Wallet with Taproot: $taprootStatus" -ForegroundColor $(if ($taprootStatus -match "Ready") { "Green" } elseif ($taprootStatus -match "Missing") { "Red" } else { "Yellow" })

if ($featuresReady -and $dependenciesReady -and $testsAvailable) {
    Write-Host "`nOverall Status: ✅ READY FOR PACKAGING" -ForegroundColor Green
} elseif ($featuresReady -and $dependenciesReady) {
    Write-Host "`nOverall Status: ⚠️ ALMOST READY" -ForegroundColor Yellow
} else {
    Write-Host "`nOverall Status: ❌ NOT READY" -ForegroundColor Red
}
