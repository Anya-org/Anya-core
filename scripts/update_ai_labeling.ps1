#!/usr/bin/env pwsh
# Update AI Labeling Script [AIR-3][AIS-3][BPC-3]
# This script finds and updates deprecated AI labeling formats across Markdown files in the codebase

$CANONICAL_DOC = "docs/standards/AI_LABELING.md"
$DEPRECATED_DOCS = @("docs/AI_LABELING.md", "docs/API.md")

Write-Host "AI Label Standardization Tool v1.0.0" -ForegroundColor Cyan
Write-Host "Based on canonical documentation: $CANONICAL_DOC" -ForegroundColor Cyan
Write-Host ""

# Get all markdown files
Write-Host "Scanning for Markdown files..." -ForegroundColor Yellow
$mdFiles = Get-ChildItem -Path . -Filter "*.md" -Recurse -File

Write-Host "Found $($mdFiles.Count) Markdown files" -ForegroundColor Green
Write-Host ""

$replacementCount = 0
$fileCount = 0

# Regex patterns for finding old formats
$oldFormats = @(
    @{Pattern = "AIR-0\d{2}"; Replacement = "[AIR-3]"},
    @{Pattern = "AIS-0\d{2}"; Replacement = "[AIS-3]"},
    @{Pattern = "AIT-0\d{2}"; Replacement = "[AIT-3]"},
    @{Pattern = "AIM-0\d{2}"; Replacement = "[AIM-3]"},
    @{Pattern = "AIP-0\d{2}"; Replacement = "[AIP-3]"},
    @{Pattern = "AIE-0\d{2}"; Replacement = "[AIE-3]"},
    @{Pattern = "AIP-0\d{2}"; Replacement = "[AIP-3]"},
    @{Pattern = "BPC-0\d{2}"; Replacement = "[BPC-3]"},
    @{Pattern = "RES-0\d{2}"; Replacement = "[RES-3]"},
    # Legacy formats with parentheses
    @{Pattern = "\(AIR-0\d{2}\)"; Replacement = "[AIR-3]"},
    @{Pattern = "\(AIS-0\d{2}\)"; Replacement = "[AIS-3]"},
    @{Pattern = "\(AIT-0\d{2}\)"; Replacement = "[AIT-3]"},
    @{Pattern = "\(BPC-0\d{2}\)"; Replacement = "[BPC-3]"}
)

# Process each file
foreach ($file in $mdFiles) {
    # Skip the canonical document
    if ($file.FullName -eq $CANONICAL_DOC) {
        continue
    }
    
    $content = Get-Content -Path $file.FullName -Raw
    $originalContent = $content
    $modified = $false
    
    # Check if the file is one of the deprecated docs
    $isDeprecated = $DEPRECATED_DOCS -contains $file.FullName
    
    # Replace old formats
    foreach ($format in $oldFormats) {
        if ($content -match $format.Pattern) {
            $content = $content -replace $format.Pattern, $format.Replacement
            $modified = $true
            $replacementCount++
        }
    }
    
    # Update the file if modified
    if ($modified) {
        $fileCount++
        Set-Content -Path $file.FullName -Value $content
        Write-Host "Updated: $($file.FullName)" -ForegroundColor Green
    }
}

Write-Host ""
Write-Host "Summary:" -ForegroundColor Cyan
Write-Host "- Processed $($mdFiles.Count) Markdown files" -ForegroundColor White
Write-Host "- Updated $fileCount files" -ForegroundColor Green
Write-Host "- Made $replacementCount replacements" -ForegroundColor Green
Write-Host ""
Write-Host "Completed AI labeling standardization" -ForegroundColor Cyan 