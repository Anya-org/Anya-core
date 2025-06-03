#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Migrates legacy AI labeling formats to the standardized format.

.DESCRIPTION
    This script scans the codebase for level-4 AI labels and converts them to level-3
    as per the standardized AI labeling system.

.PARAMETER Path
    The path to scan for files. Defaults to the repository root.

.PARAMETER ExcludePaths
    Paths to exclude from scanning.

.PARAMETER Extension
    File extensions to include in scanning. Defaults to common code and documentation files.

.EXAMPLE
    ./migrate_ai_labels.ps1

.EXAMPLE
    ./migrate_ai_labels.ps1 -Path src/bitcoin -Extension "*.rs"
#>

param (
    [string]$Path = ".",
    [string[]]$ExcludePaths = @("node_modules", ".git", "target", "dist", "build"),
    [string[]]$Extension = @("*.rs", "*.js", "*.ts", "*.jsx", "*.tsx", "*.py", "*.md", "*.txt")
)

# Configuration
$MigrationRules = @(
    @{ Pattern = '\[DAO-4\]'; Replacement = '[DAO-3]' },
    @{ Pattern = '\[AIR-4\]'; Replacement = '[AIR-3]' },
    @{ Pattern = '\[AIS-4\]'; Replacement = '[AIS-3]' },
    @{ Pattern = '\[AIT-4\]'; Replacement = '[AIT-3]' },
    @{ Pattern = '\[AIM-4\]'; Replacement = '[AIM-3]' },
    @{ Pattern = '\[AIP-4\]'; Replacement = '[AIP-3]' },
    @{ Pattern = '\[AIE-4\]'; Replacement = '[AIE-3]' },
    @{ Pattern = '\[BPC-4\]'; Replacement = '[BPC-3]' },
    @{ Pattern = '\[RES-4\]'; Replacement = '[RES-3]' },
    @{ Pattern = '\[SCL-4\]'; Replacement = '[SCL-3]' },
    @{ Pattern = '\[PFM-4\]'; Replacement = '[PFM-3]' },
    @{ Pattern = '\[DID-4\]'; Replacement = '[DID-3]' },
    @{ Pattern = '\[W5C-4\]'; Replacement = '[W5C-3]' },
    @{ Pattern = '\[UXA-4\]'; Replacement = '[UXA-3]' }
)

# Progress tracking
$totalFiles = 0
$modifiedFiles = 0
$totalReplacements = 0

function Get-FilesToProcess {
    $excludePattern = ($ExcludePaths -join '|')
    if ($excludePattern) {
        $excludePattern = "($excludePattern)"
    }
    else {
        $excludePattern = "^$"
    }

    $files = @()
    foreach ($ext in $Extension) {
        $extFiles = Get-ChildItem -Path $Path -Recurse -File -Filter $ext | 
            Where-Object { $_.FullName -notmatch $excludePattern }
        $files += $extFiles
    }
    return $files
}

function Update-AILabels {
    param (
        [System.IO.FileInfo]$File
    )

    $content = Get-Content -Path $File.FullName -Raw
    $originalContent = $content
    $replacementsInFile = 0

    foreach ($rule in $MigrationRules) {
        $matches = [regex]::Matches($content, $rule.Pattern)
        if ($matches.Count -gt 0) {
            $content = $content -replace $rule.Pattern, $rule.Replacement
            $replacementsInFile += $matches.Count
            $script:totalReplacements += $matches.Count
        }
    }

    if ($content -ne $originalContent) {
        Set-Content -Path $File.FullName -Value $content -NoNewline
        $script:modifiedFiles++
        Write-Host "✓ Updated $($File.FullName): $replacementsInFile replacements" -ForegroundColor Green
    }

    return $replacementsInFile
}

# Main execution
Write-Host "AI Label Migration Tool v1.0.0" -ForegroundColor Cyan
Write-Host "Scanning for files..." -ForegroundColor Cyan

$files = Get-FilesToProcess
$totalFiles = $files.Count

Write-Host "Found $totalFiles files to process" -ForegroundColor Cyan

foreach ($file in $files) {
    Update-AILabels -File $file
}

# Summary
Write-Host "`nMigration Summary:" -ForegroundColor Cyan
Write-Host "-----------------" -ForegroundColor Cyan
Write-Host "Files processed: $totalFiles" -ForegroundColor White
Write-Host "Files modified: $modifiedFiles" -ForegroundColor Green
Write-Host "Total replacements: $totalReplacements" -ForegroundColor Green

if ($totalReplacements -gt 0) {
    Write-Host "`nRun the validation script to confirm successful migration:" -ForegroundColor Yellow
    Write-Host "  ./scripts/validate_ai_labels.ps1" -ForegroundColor Yellow
} 