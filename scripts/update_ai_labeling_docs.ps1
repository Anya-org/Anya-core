# [AIR-3][AIS-3][BPC-3] AI Labeling Documentation Update Script
# This script adds deprecation notices to existing AI labeling documentation files

# Set script version
$VERSION = "1.0.0"
$CANONICAL_DOC = "docs/standards/AI_LABELING.md"

Write-Host "AI Labeling Documentation Update Script v$VERSION" -ForegroundColor Green
Write-Host "This script adds deprecation notices to existing AI labeling files" -ForegroundColor Green
Write-Host ""

# Files to update with deprecation notices
$filesToUpdate = @(
    "docs/AI_labelling.md",
    "docs/AI_LABELING.md",
    "docs/AI_LABELING_IMPLEMENTATION.md",
    "docs/LABELLING_SYSTEM.md",
    "AI_LABELLING.md",
    "AI_LABELING_IMPLEMENTATION.md"
)

# Deprecation notice to add
$deprecationNotice = @"
<!-- markdownlint-disable MD013 line-length -->

> **⚠️ DEPRECATION NOTICE ⚠️**
> 
> This document is deprecated and will be removed in future versions.
> Please use the canonical AI labeling documentation at [$CANONICAL_DOC]($CANONICAL_DOC).
> The canonical document standardizes all AI labeling formats and requirements.

<!-- Original content below this line -->

"@

# Function to add deprecation notice to a file
function Add-DeprecationNotice {
    param (
        [string]$filePath
    )
    
    if (Test-Path $filePath) {
        $content = Get-Content -Path $filePath -Raw
        
        # Check if the deprecation notice already exists
        if ($content -match "DEPRECATION NOTICE") {
            Write-Host "Deprecation notice already exists in $filePath" -ForegroundColor Yellow
            return $false
        }
        
        # Add deprecation notice at the top of the file
        $updatedContent = $deprecationNotice + $content
        Set-Content -Path $filePath -Value $updatedContent
        
        Write-Host "✓ Added deprecation notice to $filePath" -ForegroundColor Green
        return $true
    }
    else {
        Write-Host "✗ File not found: $filePath" -ForegroundColor Red
        return $false
    }
}

$updatedCount = 0

# Update each file with a deprecation notice
foreach ($file in $filesToUpdate) {
    if (Add-DeprecationNotice -filePath $file) {
        $updatedCount++
    }
}

Write-Host ""
Write-Host "Summary:" -ForegroundColor Cyan
Write-Host "--------" -ForegroundColor Cyan
Write-Host "Files updated: $updatedCount / $($filesToUpdate.Count)" -ForegroundColor White

# If all files were updated successfully, suggest running the validation script
if ($updatedCount -eq $filesToUpdate.Count) {
    Write-Host ""
    Write-Host "All files updated successfully!" -ForegroundColor Green
    Write-Host "Next steps:" -ForegroundColor Cyan
    Write-Host "1. Run the validation script: ./scripts/validate_ai_labels.ps1" -ForegroundColor White
    Write-Host "2. Install Git hooks: ./scripts/install_hooks.ps1" -ForegroundColor White
    Write-Host "3. Consider updating code files with the new standardized format" -ForegroundColor White
}
else {
    Write-Host ""
    Write-Host "Some files were not updated. Please check the messages above." -ForegroundColor Yellow
}

# Check if the canonical document exists and is valid
if (Test-Path $CANONICAL_DOC) {
    Write-Host ""
    Write-Host "The canonical document exists at $CANONICAL_DOC" -ForegroundColor Green
}
else {
    Write-Host ""
    Write-Host "⚠️ The canonical document does not exist at $CANONICAL_DOC" -ForegroundColor Red
    Write-Host "Please create it before running this script again." -ForegroundColor Red
} 