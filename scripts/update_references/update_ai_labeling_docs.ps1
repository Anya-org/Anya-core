<#
.SYNOPSIS
    Updates references to AI labeling documentation across the codebase
.DESCRIPTION
    This script finds and updates references to deprecated AI labeling 
    documentation files, replacing them with references to the canonical 
    AI_LABELING.md file in the docs/standards directory.
.NOTES
    Version:        1.0
    Author:         Anya Core Team
    Creation Date:  2025-03-20
#>

# Configuration
$canonicalPathRelative = "docs/standards/AI_LABELING.md"
$canonicalPathAbsolute = "$PSScriptRoot/../../$canonicalPathRelative"
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
$filesUpdated = 0
$totalReplacementsCount = 0

Write-Host "`n🔄 AI Labeling Reference Updater [AIR-3][AIS-3][BPC-3]`n" -ForegroundColor Cyan
Write-Host "Scanning repository for outdated AI labeling references...`n"

# Get all markdown, HTML, and code files
$filesToScan = Get-ChildItem -Path "$PSScriptRoot/../.." -Recurse -File -Include "*.md","*.html","*.js","*.ts","*.jsx","*.tsx","*.py","*.java","*.cs","*.go","*.rs","*.sh","*.ps1","*.sql" | 
                Where-Object { -not $_.FullName.Contains("node_modules") -and -not $_.FullName.Contains(".git") }

# Scan and update each file for old references
foreach ($file in $filesToScan) {
    $totalFiles++
    $fileContent = Get-Content -Path $file.FullName -Raw
    $originalContent = $fileContent
    $replacementsInFile = 0
    
    # Process URL links first - special format
    foreach ($oldRef in $oldReferences) {
        # Match patterns like [text](oldRef) and replace with [text](canonicalPathRelative)
        $pattern = "\[([^\]]+)\]\($oldRef\)"
        $replacement = "[$1]($canonicalPathRelative)"
        $newContent = [regex]::Replace($fileContent, $pattern, $replacement)
        
        if ($newContent -ne $fileContent) {
            $count = ([regex]::Matches($fileContent, $pattern)).Count
            $replacementsInFile += $count
            $fileContent = $newContent
        }
        
        # Also handle HTML links
        $htmlPattern = "href=`"$oldRef`""
        $htmlReplacement = "href=`"$canonicalPathRelative`""
        $newContent = $fileContent -replace $htmlPattern, $htmlReplacement
        
        if ($newContent -ne $fileContent) {
            $count = ([regex]::Matches($fileContent, $htmlPattern)).Count
            $replacementsInFile += $count
            $fileContent = $newContent
        }
    }
    
    # Direct textual references
    foreach ($oldRef in $oldReferences) {
        # Simple text replacements
        $newContent = $fileContent -replace [regex]::Escape($oldRef), $canonicalPathRelative
        
        if ($newContent -ne $fileContent) {
            # Count occurrences of oldRef in fileContent
            $count = ([regex]::Matches($fileContent, [regex]::Escape($oldRef))).Count
            $replacementsInFile += $count
            $fileContent = $newContent
        }
    }
    
    # Update file if content changed
    if ($fileContent -ne $originalContent) {
        $relativePath = $file.FullName.Replace("$PSScriptRoot\..\..\", "").Replace("\", "/")
        
        Write-Host "🔄 Updating references in file: $relativePath" -ForegroundColor Green
        Write-Host "   Replacements made: $replacementsInFile" -ForegroundColor Green
        
        # Write the updated content back to the file
        Set-Content -Path $file.FullName -Value $fileContent -NoNewline
        
        $filesUpdated++
        $totalReplacementsCount += $replacementsInFile
    }
}

# Print summary
Write-Host "`n📊 Summary:" -ForegroundColor Cyan
Write-Host "   Total files scanned: $totalFiles" -ForegroundColor White
Write-Host "   Files updated: $filesUpdated" -ForegroundColor $(if ($filesUpdated -gt 0) { "Green" } else { "White" })
Write-Host "   Total replacements: $totalReplacementsCount" -ForegroundColor $(if ($totalReplacementsCount -gt 0) { "Green" } else { "White" })
Write-Host ""

if ($filesUpdated -eq 0) {
    Write-Host "✅ No files needed updating!" -ForegroundColor Green
} else {
    Write-Host "✅ Files successfully updated to use the canonical AI labeling path: $canonicalPathRelative" -ForegroundColor Green
}

exit 0 