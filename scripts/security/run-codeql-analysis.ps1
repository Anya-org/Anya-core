# Run CodeQL Analysis for Bitcoin Components
# [AIR-2][AIS-3][BPC-3]

$ErrorActionPreference = "Stop"

# Detect CodeQL CLI
try {
    $codeqlVersion = & codeql --version
    Write-Host "Found CodeQL CLI: $codeqlVersion"
} catch {
    Write-Host "CodeQL CLI not found. Please install CodeQL CLI from https://github.com/github/codeql-cli-binaries/releases"
    exit 1
}

# Parameters
$repoRoot = $PSScriptRoot | Split-Path | Split-Path
$dbDir = Join-Path $repoRoot "codeql-dbs"
$resultsDir = Join-Path $repoRoot "reports"
$configFile = Join-Path $repoRoot ".github\codeql\codeql-config.yml"
$bitcoinDir = Join-Path $repoRoot "src\bitcoin"
$scriptsDir = Join-Path $repoRoot "scripts\bitcoin"

# Create necessary directories
if (-not (Test-Path $dbDir)) {
    New-Item -ItemType Directory -Force -Path $dbDir
}
if (-not (Test-Path $resultsDir)) {
    New-Item -ItemType Directory -Force -Path $resultsDir
}

# Create JavaScript database
Write-Host "Creating JavaScript CodeQL database..."
$jsDbDir = Join-Path $dbDir "js-db"
& codeql database create $jsDbDir --language=javascript --source-root=$repoRoot

# Create Rust database
Write-Host "Creating Rust CodeQL database..."
$rustDbDir = Join-Path $dbDir "rust-db"
& codeql database create $rustDbDir --language=rust --source-root=$repoRoot

# Analyze JavaScript database
Write-Host "Analyzing JavaScript codebase..."
$jsResultsFile = Join-Path $resultsDir "js-results.sarif"
& codeql database analyze $jsDbDir $configFile --format=sarif-latest --output=$jsResultsFile

# Analyze Rust database
Write-Host "Analyzing Rust codebase..."
$rustResultsFile = Join-Path $resultsDir "rust-results.sarif"
& codeql database analyze $rustDbDir $configFile --format=sarif-latest --output=$rustResultsFile

# Run Bitcoin-specific security checks
Write-Host "Running Bitcoin-specific security checks..."
$securityScriptsDir = Join-Path $repoRoot "scripts\security"
$bitcoinScriptsDir = Join-Path $repoRoot "scripts\bitcoin"

try {
    node "$securityScriptsDir\analyze-mcp-server.js" --file="$scriptsDir\mcp-server.js"
    node "$securityScriptsDir\crypto-validation.js"
    node "$bitcoinScriptsDir\validate-bip-compliance.js"
    
    Write-Host "✅ All security checks completed successfully."
} catch {
    Write-Host "❌ Security checks failed: $_"
    exit 1
}

Write-Host "CodeQL analysis complete. Reports are available in: $resultsDir" 