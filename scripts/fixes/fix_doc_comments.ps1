# Script to fix documentation comment structure issues
# Addresses the issue of inner doc comments (//!) appearing in incorrect positions

param(
    [switch]$DryRun
)

$scriptName = "Documentation Comments Fixer"
$scriptVersion = "1.0.0"
$rootDir = Join-Path $PSScriptRoot "..\\.."

Write-Host "===== $scriptName v$scriptVersion =====" -ForegroundColor Cyan
Write-Host "Fixing documentation comment structure..."

# Directories to process
$sourceDirs = @(
    (Join-Path $rootDir "anya-bitcoin\src\core\network"),
    (Join-Path $rootDir "anya-bitcoin\src\core\mempool"),
    (Join-Path $rootDir "anya-bitcoin\src\layer2\bob"),
    (Join-Path $rootDir "anya-bitcoin\src\layer2\rgb"),
    (Join-Path $rootDir "anya-bitcoin\src\layer2\rsk")
)

# Counter for tracking changes
$counter = @{
    Total = 0
    Fixed = 0
    Skipped = 0
}

# Function to fix doc comments in a file
function Fix-DocComments {
    param(
        [string]$FilePath
    )
    
    $fileName = Split-Path -Leaf $FilePath
    Write-Host "Processing: $fileName" -ForegroundColor Yellow
    
    # Read the file content
    $content = Get-Content -Path $FilePath -Raw
    $originalContent = $content
    
    # Fix inner doc comments that appear after items
    # Pattern: matches inner doc comments (//!) that don't appear at the start of the file or after a blank line
    $pattern = "(?<!\A|\n\s*\n)(?<=\n)(\s*)(\/\/\!.*?\n)"
    
    # Replace inner doc comments with outer doc comments (///)
    $fixedContent = [regex]::Replace($content, $pattern, {
        param($match)
        $indent = $match.Groups[1].Value
        $comment = $match.Groups[2].Value
        # Replace //! with /// but maintain the rest of the line and indentation
        $fixedComment = $indent + $comment.Replace("//!", "///")
        return $fixedComment
    })
    
    # Move all remaining inner doc comments to the top of the file, before any code
    if ($fixedContent -match "(?m)^\s*\/\/\!") {
        # Extract all inner doc comments
        $innerComments = [regex]::Matches($fixedContent, "(?m)^\s*\/\/\!.*?$") | ForEach-Object { $_.Value }
        
        # Remove the inner comments from their current positions
        $contentWithoutInnerComments = [regex]::Replace($fixedContent, "(?m)^\s*\/\/\!.*?$", "")
        
        # Clean up any resulting double blank lines
        $contentWithoutInnerComments = [regex]::Replace($contentWithoutInnerComments, "\n\s*\n\s*\n", "`n`n")
        
        # Add all inner comments at the top of the file
        $innerCommentsText = $innerComments -join "`n"
        $fixedContent = $innerCommentsText + "`n`n" + $contentWithoutInnerComments.TrimStart()
    }
    
    # Check if the content was modified
    if ($fixedContent -ne $originalContent) {
        $counter.Total++
        
        if (-not $DryRun) {
            # Write the fixed content back to the file
            $fixedContent | Set-Content -Path $FilePath -Encoding UTF8
            Write-Host "  - Fixed doc comments" -ForegroundColor Green
            $counter.Fixed++
        } else {
            Write-Host "  - Would fix doc comments (dry run)" -ForegroundColor Yellow
            $counter.Skipped++
        }
    } else {
        Write-Host "  - No doc comment issues found" -ForegroundColor Gray
    }
}

# Process all Rust files in the specified directories
foreach ($dir in $sourceDirs) {
    if (Test-Path $dir) {
        $files = Get-ChildItem -Path $dir -Filter "*.rs" -Recurse
        
        foreach ($file in $files) {
            Fix-DocComments -FilePath $file.FullName
        }
    } else {
        Write-Host "Directory not found: $dir" -ForegroundColor Red
    }
}

# Print summary
Write-Host "`nDoc Comments Fixing Summary:" -ForegroundColor Cyan
Write-Host "  Total files with issues: $($counter.Total)" -ForegroundColor White
Write-Host "  Files fixed: $($counter.Fixed)" -ForegroundColor Green
Write-Host "  Files skipped (dry run): $($counter.Skipped)" -ForegroundColor Yellow

Write-Host "Documentation comment fixing completed" -ForegroundColor Green
