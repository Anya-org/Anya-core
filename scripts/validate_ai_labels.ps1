#!/usr/bin/env pwsh
# AI Label Validation Script [AIR-3][AIS-3][BPC-3]
# This script validates AI labeling formats across code and docs in the codebaseva

param(
    [string]$file = "",
    [switch]$fix = $false,
    [switch]$verbose = $false
)

$CANONICAL_DOC = "docs/standards/AI_LABELING.md"

Write-Host "AI Label Validation Tool v1.0.0" -ForegroundColor Cyan
Write-Host "Based on canonical documentation: $CANONICAL_DOC" -ForegroundColor Cyan
Write-Host ""

# Valid label patterns
$validLabels = @(
    "AIR-[0-3]",
    "AIS-[0-3]", 
    "AIT-[0-3]", 
    "AIM-[0-3]", 
    "AIP-[0-3]", 
    "AIE-[0-3]", 
    "BPC-[0-3]", 
    "RES-[0-3]", 
    "SCL-[0-3]", 
    "PFM-[0-3]", 
    "DAO-[0-3]", 
    "DID-[0-3]", 
    "W5C-[0-3]", 
    "UXA-[0-3]"
)

$validLabelRegex = "\[(" + ($validLabels -join "|") + ")\]"
$legacyLabelRegex = "(AIR|AIS|AIT|AIM|AIP|AIE|BPC|RES)-\d{3}"

# Get files to check
$filesToCheck = @()
if ($file -ne "") {
    # Check a single file
    if (Test-Path $file) {
        $filesToCheck += Get-Item $file
    } else {
        Write-Host "File not found: $file" -ForegroundColor Red
        exit 1
    }
} else {
    # Get all relevant files
    $filePatterns = @("*.md", "*.rs", "*.js", "*.py", "*.ts", "*.cs", "*.c", "*.cpp", "*.h", "*.java", "*.go", "*.sh", "*.ps1", "*.toml", "*.yaml", "*.yml")
    
    foreach ($pattern in $filePatterns) {
        $filesToCheck += Get-ChildItem -Path . -Filter $pattern -Recurse -File
    }
}

Write-Host "Checking $($filesToCheck.Count) files for AI label compliance..." -ForegroundColor Yellow
Write-Host ""

$validCount = 0
$invalidCount = 0
$legacyCount = 0
$fixedCount = 0

foreach ($file in $filesToCheck) {
    $content = Get-Content -Path $file.FullName -Raw
    $hasInvalid = $false
    $hasLegacy = $false
    
    # Check for valid labels
    $validMatches = [regex]::Matches($content, $validLabelRegex)
    
    # Check for legacy labels
    $legacyMatches = [regex]::Matches($content, $legacyLabelRegex)
    
    if ($legacyMatches.Count -gt 0) {
        $hasLegacy = $true
        $legacyCount += $legacyMatches.Count
        
        if ($verbose) {
            Write-Host "Legacy labels in $($file.FullName):" -ForegroundColor Yellow
            foreach ($match in $legacyMatches) {
                Write-Host "  $($match.Value)" -ForegroundColor Yellow
            }
        }
        
        if ($fix) {
            $updatedContent = $content
            
            # Map from legacy to new format
            $updatedContent = $updatedContent -replace "AIR-\d{3}", "[AIR-3]"
            $updatedContent = $updatedContent -replace "AIS-\d{3}", "[AIS-3]"
            $updatedContent = $updatedContent -replace "AIT-\d{3}", "[AIT-3]"
            $updatedContent = $updatedContent -replace "AIM-\d{3}", "[AIM-3]"
            $updatedContent = $updatedContent -replace "AIP-\d{3}", "[AIP-3]"
            $updatedContent = $updatedContent -replace "AIE-\d{3}", "[AIE-3]"
            $updatedContent = $updatedContent -replace "BPC-\d{3}", "[BPC-3]"
            $updatedContent = $updatedContent -replace "RES-\d{3}", "[RES-3]"
            
            # Write updated content
            if ($content -ne $updatedContent) {
                Set-Content -Path $file.FullName -Value $updatedContent
                $fixedCount++
                Write-Host "Fixed legacy labels in $($file.FullName)" -ForegroundColor Green
            }
        }
    }
    
    # Update counters
    if ($validMatches.Count -gt 0 -and -not $hasLegacy) {
        $validCount++
        
        if ($verbose) {
            Write-Host "Valid labels in $($file.FullName):" -ForegroundColor Green
            foreach ($match in $validMatches) {
                Write-Host "  $($match.Value)" -ForegroundColor Green
            }
        }
    } elseif ($validMatches.Count -gt 0 -and $hasLegacy) {
        $invalidCount++
        Write-Host "Mixed label formats in $($file.FullName)" -ForegroundColor Yellow
    } elseif ($hasLegacy) {
        $invalidCount++
    }
}

Write-Host ""
Write-Host "Validation Summary:" -ForegroundColor Cyan
Write-Host "- Checked $($filesToCheck.Count) files" -ForegroundColor White
Write-Host "- Found $validCount files with valid labels" -ForegroundColor Green
Write-Host "- Found $invalidCount files with invalid or mixed labels" -ForegroundColor Yellow
Write-Host "- Found $legacyCount legacy label instances" -ForegroundColor Yellow

if ($fix) {
    Write-Host "- Fixed labels in $fixedCount files" -ForegroundColor Green
}

Write-Host ""
if ($invalidCount -gt 0) {
    Write-Host "Validation failed: $invalidCount files with non-compliant labels" -ForegroundColor Red
    Write-Host "Run with -fix to automatically update legacy formats" -ForegroundColor Yellow
    exit 1
} else {
    Write-Host "Validation successful: All files use compliant label formats" -ForegroundColor Green
    exit 0
}

<#
.SYNOPSIS
    Validates AI labeling references across the codebase
.DESCRIPTION
    This script scans the codebase for references to outdated AI labeling files
    and provides warnings about any files that need to be updated to reference
    the canonical AI_LABELING.md file in docs/standards directory.
.NOTES
    Version:        1.0
    Author:         Anya Core Team
    Creation Date:  2025-03-20
#>

# Configuration
$canonicalPathRelative = "docs/standards/AI_LABELING.md"
$canonicalPathAbsolute = "$PSScriptRoot/../$canonicalPathRelative"
$oldReferences = @(
    "docs/standards/AI_LABELING.md",
    "docs/docs/standards/AI_LABELING.md",
    "docs/standards/AI_LABELING.md",
    "docs/standards/AI_LABELING.md",
    "docs/docs/standards/AI_LABELING.md"
)

# Check if canonical file exists
if (-not (Test-Path $canonicalPathAbsolute)) {
    Write-Error "Canonical AI labeling file not found at: $canonicalPathRelative"
    exit 1
}

# Counters for reporting
$totalFiles = 0
$filesWithOldReferences = 0
$totalOldReferences = 0

Write-Host "`n📋 AI Labeling Reference Validator [AIR-3][AIS-3][BPC-3]`n" -ForegroundColor Cyan
Write-Host "Scanning repository for outdated AI labeling references...`n"

# Get all markdown, HTML, and code files
$filesToScan = Get-ChildItem -Path "$PSScriptRoot/.." -Recurse -File -Include "*.md","*.html","*.js","*.ts","*.jsx","*.tsx","*.py","*.java","*.cs","*.go","*.rs","*.sh","*.ps1","*.sql" | 
                Where-Object { -not $_.FullName.Contains("node_modules") -and -not $_.FullName.Contains(".git") }

# Scan each file for old references
foreach ($file in $filesToScan) {
    $totalFiles++
    $fileContent = Get-Content -Path $file.FullName -Raw
    $foundOldReferences = @()
    
    foreach ($oldRef in $oldReferences) {
        if ($fileContent -match [regex]::Escape($oldRef)) {
            $foundOldReferences += $oldRef
            $totalOldReferences++
        }
    }
    
    if ($foundOldReferences.Count -gt 0) {
        $filesWithOldReferences++
        $relativePath = $file.FullName.Replace("$PSScriptRoot\..\", "").Replace("\", "/")
        
        Write-Host "⚠️ File contains outdated AI labeling references: $relativePath" -ForegroundColor Yellow
        Write-Host "   Found references to: $($foundOldReferences -join ', ')" -ForegroundColor Yellow
        Write-Host "   Update to use canonical path: $canonicalPathRelative" -ForegroundColor Yellow
        Write-Host ""
    }
}

# Print summary
Write-Host "📊 Summary:" -ForegroundColor Cyan
Write-Host "   Total files scanned: $totalFiles" -ForegroundColor White
Write-Host "   Files with outdated references: $filesWithOldReferences" -ForegroundColor $(if ($filesWithOldReferences -gt 0) { "Yellow" } else { "Green" })
Write-Host "   Total outdated references found: $totalOldReferences" -ForegroundColor $(if ($totalOldReferences -gt 0) { "Yellow" } else { "Green" })
Write-Host ""

if ($totalOldReferences -eq 0) {
    Write-Host "✅ No outdated AI labeling references found!" -ForegroundColor Green
    exit 0
} else {
    Write-Host "⚠️ Please update all references to use the canonical AI labeling path: $canonicalPathRelative" -ForegroundColor Yellow
    exit 0  # Non-zero exit code would be appropriate for CI/CD pipelines
} 