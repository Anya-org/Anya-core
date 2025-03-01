# Validation script for RGB and Web5 components
Write-Host "=== Anya Core Component Validation ===" -ForegroundColor Cyan

# Define paths to check
$baseDir = "c:\Users\bmokoka\Downloads\OPSource\anya-core"
$srcDir = Join-Path $baseDir "src"
$depsDir = Join-Path $baseDir "dependencies"
$testsDir = Join-Path $baseDir "tests"

Write-Host "`nChecking core components..." -ForegroundColor Yellow

# Check Bitcoin components
$bitcoinDir = Join-Path $srcDir "bitcoin"
$bitcoinModules = @(
    "mod.rs",
    "wallet.rs",
    "rgb",
    "taproot",
    "transaction.rs"
)

Write-Host "`n1. Bitcoin Components:" -ForegroundColor Green
foreach ($module in $bitcoinModules) {
    $path = Join-Path $bitcoinDir $module
    $exists = Test-Path $path
    $status = if ($exists) { "✅ Found" } else { "❌ Missing" }
    Write-Host "   - $module : $status"
}

# Check RGB components
$rgbDir = Join-Path $bitcoinDir "rgb"
$rgbModules = @(
    "mod.rs",
    "asset.rs",
    "metadata.rs"
)

Write-Host "`n2. RGB Components:" -ForegroundColor Green
foreach ($module in $rgbModules) {
    $path = Join-Path $rgbDir $module
    $exists = Test-Path $path
    $status = if ($exists) { "✅ Found" } else { "❌ Missing" }
    Write-Host "   - $module : $status"
}

# Check Web5 components
$web5Dir = Join-Path $srcDir "web5"
$web5Modules = @(
    "mod.rs",
    "did.rs",
    "credential.rs",
    "dwn",
    "anchoring.rs"
)

Write-Host "`n3. Web5 Components:" -ForegroundColor Green
foreach ($module in $web5Modules) {
    $path = Join-Path $web5Dir $module
    $exists = Test-Path $path
    $status = if ($exists) { "✅ Found" } else { "❌ Missing" }
    Write-Host "   - $module : $status"
}

# Check Test files
$testFiles = @(
    "rgb_asset_test.rs",
    "web5_anchoring_test.rs"
)

Write-Host "`n4. Test Components:" -ForegroundColor Green
foreach ($testFile in $testFiles) {
    $path = Join-Path $testsDir $testFile
    $exists = Test-Path $path
    $status = if ($exists) { "✅ Found" } else { "❌ Missing" }
    Write-Host "   - $testFile : $status"
}

# Check Dependencies
$depsFiles = @(
    "anya-bitcoin",
    "anya-extensions"
)

Write-Host "`n5. Dependencies:" -ForegroundColor Green
foreach ($depFile in $depsFiles) {
    $path = Join-Path $depsDir $depFile
    $exists = Test-Path $path
    $status = if ($exists) { "✅ Found" } else { "❌ Missing" }
    Write-Host "   - $depFile : $status"
}

# Check for configuration files
$configFiles = @(
    "Cargo.toml",
    ".env",
    "README.md"
)

Write-Host "`n6. Configuration Files:" -ForegroundColor Green
foreach ($configFile in $configFiles) {
    $path = Join-Path $baseDir $configFile
    $exists = Test-Path $path
    $status = if ($exists) { "✅ Found" } else { "❌ Missing" }
    Write-Host "   - $configFile : $status"
}

Write-Host "`n=== Validation Summary ===" -ForegroundColor Cyan
Write-Host "The key components for RGB asset transfer and Web5 with Bitcoin anchoring"
Write-Host "have been checked. Please review any missing components and implement them."
Write-Host "Components marked with ✅ are available and ready for use."
Write-Host "`nAny missing components should be implemented according to the design specifications."
