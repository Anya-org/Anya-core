# Simple validation report script
$reportsDir = Join-Path "c:\Users\bmokoka\Downloads\OPSource\anya-core" "reports"
if (-not (Test-Path $reportsDir)) {
    New-Item -ItemType Directory -Path $reportsDir -Force | Out-Null
}

$reportFile = Join-Path $reportsDir "packaging_validation.md"
$timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"

# Base paths
$baseDir = "c:\Users\bmokoka\Downloads\OPSource\anya-core"
$srcDir = Join-Path $baseDir "src"
$testsDir = Join-Path $baseDir "tests"

# Check for RGB test file
$rgbTestFile = Join-Path $testsDir "rgb_asset_test.rs"
$rgbTestExists = Test-Path $rgbTestFile -PathType Leaf

# Check for Web5 test file
$web5TestFile = Join-Path $testsDir "web5_anchoring_test.rs"
$web5TestExists = Test-Path $web5TestFile -PathType Leaf

# Check for Taproot directory
$bitcoinDir = Join-Path $srcDir "bitcoin"
$taprootDir = Join-Path $bitcoinDir "taproot"
$taprootExists = Test-Path $taprootDir -PathType Container

# Check for Basic cargo.toml
$cargoFile = Join-Path $baseDir "Cargo.toml"
$cargoExists = Test-Path $cargoFile -PathType Leaf

# Create report content
$report = @"
# Anya Core - Packaging Validation Report
**Generated:** $timestamp

## Feature Status

| Feature | Status |
|---------|--------|
| RGB Asset Test | $(if ($rgbTestExists) { "✅ Available" } else { "❌ Missing" }) |
| Web5 Anchoring Test | $(if ($web5TestExists) { "✅ Available" } else { "❌ Missing" }) |
| Taproot Implementation | $(if ($taprootExists) { "✅ Available" } else { "❌ Missing" }) |
| Cargo Configuration | $(if ($cargoExists) { "✅ Available" } else { "❌ Missing" }) |

## Packaging Readiness

"@

# Add overall status
if ($rgbTestExists -and $web5TestExists -and $taprootExists -and $cargoExists) {
    $report += @"
**Status: ✅ READY FOR PACKAGING**

All core components for RGB asset transfer and Web5 with Bitcoin anchoring are available. The project is ready for packaging and deployment.

**Next Steps:**
1. Run final integration tests
2. Package the project
3. Deploy to production
"@
} else {
    $report += @"
**Status: ❌ NOT READY FOR PACKAGING**

Some core components are missing:
"@
    
    if (-not $rgbTestExists) {
        $report += "- RGB asset test is missing\n"
    }
    if (-not $web5TestExists) {
        $report += "- Web5 anchoring test is missing\n"
    }
    if (-not $taprootExists) {
        $report += "- Taproot implementation is missing\n"
    }
    if (-not $cargoExists) {
        $report += "- Cargo configuration is missing\n"
    }
    
    $report += @"

Complete the missing components before proceeding with packaging.
"@
}

# Write report to file
$report | Out-File -FilePath $reportFile -Encoding utf8

Write-Host "Report generated: $reportFile"
Write-Host ""
Write-Host "=== Packaging Readiness Summary ==="
Write-Host ""
Write-Host "RGB Asset Test: $(if ($rgbTestExists) { "✅ Available" } else { "❌ Missing" })"
Write-Host "Web5 Anchoring Test: $(if ($web5TestExists) { "✅ Available" } else { "❌ Missing" })"
Write-Host "Taproot Implementation: $(if ($taprootExists) { "✅ Available" } else { "❌ Missing" })"
Write-Host "Cargo Configuration: $(if ($cargoExists) { "✅ Available" } else { "❌ Missing" })"
Write-Host ""

if ($rgbTestExists -and $web5TestExists -and $taprootExists -and $cargoExists) {
    Write-Host "Overall Status: ✅ READY FOR PACKAGING" -ForegroundColor Green
} else {
    Write-Host "Overall Status: ❌ NOT READY FOR PACKAGING" -ForegroundColor Red
}
