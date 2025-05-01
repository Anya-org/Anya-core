# [AIR-3][AIS-3][BPC-3][AIT-3] BIP Health Check PowerShell Script
# Runs the BIP health checker and creates a report

param(
    [switch]$Help,
    [switch]$VerboseOutput,
    [string]$Format = "markdown",
    [string]$Output,
    [string]$Mode = "report"
)

# Script directory and project root
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = (Get-Item $ScriptDir).Parent.Parent.FullName

# Create reports directory if it doesn't exist
$ReportsDir = Join-Path -Path $ProjectRoot -ChildPath "reports\bip"
if (-not (Test-Path $ReportsDir)) {
    New-Item -ItemType Directory -Path $ReportsDir -Force | Out-Null
}

# Default timestamp format
$Timestamp = Get-Date -Format "yyyyMMddHHmmss"

# Default output path if not specified
if (-not $Output) {
    $Output = Join-Path -Path $ReportsDir -ChildPath "bip-health-$Timestamp.md"
}

function Print-Usage {
    Write-Host "Usage: .\check-health.ps1 [options]"
    Write-Host "Options:"
    Write-Host "  -VerboseOutput     Enable verbose output"
    Write-Host "  -Format FORMAT     Output format (markdown, json, text)"
    Write-Host "  -Output FILE       Output file path"
    Write-Host "  -Mode MODE         Mode (check, report, monitor)"
    Write-Host "  -Help              Print this help message"
}

if ($Help) {
    Print-Usage
    exit 0
}

# Prepare verbose flag
$VerboseFlag = if ($VerboseOutput) { "--verbose" } else { "" }

# Build the binary if needed
Write-Host "Building BIP health checker..."
Push-Location $ProjectRoot
cargo build --bin bip_health
Pop-Location

# Run the health checker
Write-Host "Running BIP health check in $Mode mode..."

$Binary = Join-Path -Path $ProjectRoot -ChildPath "target\debug\bip_health.exe"

switch ($Mode.ToLower()) {
    "check" {
        & $Binary $VerboseFlag check --format $Format
    }
    "report" {
        & $Binary $VerboseFlag report --format $Format --output $Output
        Write-Host "Report saved to: $Output"
    }
    "monitor" {
        & $Binary $VerboseFlag monitor --output-dir $ReportsDir
    }
    default {
        Write-Host "Invalid mode: $Mode"
        Print-Usage
        exit 1
    }
}

Write-Host "BIP health check completed successfully!" 