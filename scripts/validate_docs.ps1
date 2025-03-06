# Documentation Validation Script
# This script validates documentation quality and identifies issues

param(
    [switch]$ReportOnly,
    [string]$RootDir = ".",
    [switch]$FixIssues
)

# Configuration
$Config = @{
    RequiredFiles = @(
        "README.md",
        "API.md",
        "CONTRIBUTING.md",
        "SECURITY.md",
        "CHANGELOG.md",
        "docs/INDEX.md",
        "AI_LABELLING.md"
    )
    
    RequiredDirs = @(
        "docs/api",
        "docs/architecture",
        "docs/development",
        "docs/guides",
        "docs/security"
    )
    
    LinkPatterns = @(
        '(?<!!)\[([^\]]+)\]\(([^)]+)\)',  # Markdown links [text](url)
        'href=["'']([^"'']+)["'']',       # HTML links href="url"
        'url\s*:\s*["'']([^"'']+)["'']'   # URL in YAML/JSON
    )
}

# Initialize counters
$stats = @{
    FilesChecked = 0
    BrokenLinks = 0
    MissingFiles = 0
    StyleViolations = 0
    SpellingErrors = 0
    FixedIssues = 0
}

# Function to check if a file exists
function Test-RequiredFiles {
    $missing = @()
    
    foreach ($file in $Config.RequiredFiles) {
        $filePath = Join-Path $RootDir $file
        if (-not (Test-Path $filePath)) {
            $missing += $file
            $stats.MissingFiles++
        }
    }
    
    foreach ($dir in $Config.RequiredDirs) {
        $dirPath = Join-Path $RootDir $dir
        if (-not (Test-Path $dirPath)) {
            $missing += $dir
            $stats.MissingFiles++
        }
    }
    
    return $missing
}

# Function to check for broken links
function Test-Links {
    param (
        [string]$FilePath,
        [string]$Content
    )
    
    $brokenLinks = @()
    
    foreach ($pattern in $Config.LinkPatterns) {
        if ($Content -match $pattern) {
            $matches = [regex]::Matches($Content, $pattern)
            
            foreach ($match in $matches) {
                $url = $match.Groups[1].Value
                
                # Skip external URLs and anchors
                if ($url -match "^(http|https|mailto|tel):" -or $url.StartsWith("#")) {
                    continue
                }
                
                # Handle relative paths
                $targetPath = if ($url.StartsWith("/")) {
                    Join-Path $RootDir ($url.Substring(1))
                } else {
                    Join-Path (Split-Path $FilePath -Parent) $url
                }
                
                # Check if the file exists
                if (-not (Test-Path $targetPath)) {
                    $lineContent = Get-Content -Path $FilePath | Select-String -Pattern ([regex]::Escape($match.Value))
                    $lineNumber = if ($lineContent) { $lineContent.LineNumber } else { "Unknown" }
                    
                    $brokenLinks += @{
                        Source = $FilePath
                        Target = $url
                        Line = $lineNumber
                    }
                    $stats.BrokenLinks++
                }
            }
        }
    }
    
    return $brokenLinks
}

# Function to check for style violations
function Test-Style {
    param (
        [string]$FilePath
    )
    
    $violations = @()
    
    # Basic style checks
    $content = Get-Content $FilePath -Raw
    
    # Check for trailing whitespace
    if ($content -match "\s+$") {
        $violations += "Trailing whitespace"
    }
    
    # Check for inconsistent heading structure
    $headings = [regex]::Matches($content, "^(#+)\s+(.+)$", "Multiline")
    $prevLevel = 0
    
    foreach ($heading in $headings) {
        $level = $heading.Groups[1].Value.Length
        
        if ($prevLevel -gt 0 -and $level - $prevLevel -gt 1) {
            $violations += "Inconsistent heading structure: $($heading.Groups[2].Value)"
        }
        
        $prevLevel = $level
    }
    
    # Check for very long lines
    $lines = Get-Content $FilePath
    foreach ($line in $lines) {
        if ($line.Length -gt 120) {
            $violations += "Line too long: $($line.Substring(0, 50))..."
        }
    }
    
    $stats.StyleViolations += $violations.Count
    
    return $violations
}

# Function to fix common issues
function Fix-CommonIssues {
    param (
        [string]$FilePath
    )
    
    $content = Get-Content $FilePath -Raw
    $original = $content
    
    # Fix trailing whitespace
    $content = $content -replace "\s+$", ""
    
    # Fix common URL issues
    $content = $content -replace "github.com/botshelomokoka/anya-core", "github.com/anya-org/anya-core"
    
    if ($content -ne $original) {
        Set-Content -Path $FilePath -Value $content
        $stats.FixedIssues++
        return $true
    }
    
    return $false
}

# Main execution
Write-Host "Starting documentation validation..." -ForegroundColor Cyan

# Check required files
$missingFiles = Test-RequiredFiles
if ($missingFiles.Count -gt 0) {
    Write-Host "`nMissing required files/directories:" -ForegroundColor Red
    foreach ($file in $missingFiles) {
        Write-Host "  - $file" -ForegroundColor Red
    }
} else {
    Write-Host "All required files and directories present." -ForegroundColor Green
}

# Process Markdown files
$mdFiles = Get-ChildItem -Path $RootDir -Recurse -Include "*.md"
$brokenLinks = @()
$styleViolations = @{}

foreach ($file in $mdFiles) {
    $stats.FilesChecked++
    $content = Get-Content $file.FullName -Raw
    
    # Check links
    $fileLinks = Test-Links -FilePath $file.FullName -Content $content
    if ($fileLinks.Count -gt 0) {
        $brokenLinks += $fileLinks
    }
    
    # Check style
    $fileViolations = Test-Style -FilePath $file.FullName
    if ($fileViolations.Count -gt 0) {
        $styleViolations[$file.FullName] = $fileViolations
    }
    
    # Fix issues if requested
    if ($FixIssues) {
        if (Fix-CommonIssues -FilePath $file.FullName) {
            Write-Host "Fixed issues in: $($file.FullName)" -ForegroundColor Yellow
        }
    }
}

# Output broken links
if ($brokenLinks.Count -gt 0) {
    Write-Host "`nBroken links found:" -ForegroundColor Red
    foreach ($link in $brokenLinks) {
        Write-Host "  - $($link.Source):$($link.Line) -> $($link.Target)" -ForegroundColor Red
    }
} else {
    Write-Host "No broken links found." -ForegroundColor Green
}

# Output style violations
if ($styleViolations.Count -gt 0) {
    Write-Host "`nStyle violations found:" -ForegroundColor Red
    foreach ($file in $styleViolations.Keys) {
        Write-Host "  File: $file" -ForegroundColor Red
        foreach ($violation in $styleViolations[$file]) {
            Write-Host "    - $violation" -ForegroundColor Red
        }
    }
} else {
    Write-Host "No style violations found." -ForegroundColor Green
}

# Output summary
Write-Host "`nValidation Summary:" -ForegroundColor Cyan
Write-Host "  Files checked: $($stats.FilesChecked)" -ForegroundColor Cyan
Write-Host "  Broken links: $($stats.BrokenLinks)" -ForegroundColor $(if ($stats.BrokenLinks -gt 0) { "Red" } else { "Green" })
Write-Host "  Missing files: $($stats.MissingFiles)" -ForegroundColor $(if ($stats.MissingFiles -gt 0) { "Red" } else { "Green" })
Write-Host "  Style violations: $($stats.StyleViolations)" -ForegroundColor $(if ($stats.StyleViolations -gt 0) { "Red" } else { "Green" })

if ($FixIssues) {
    Write-Host "  Issues fixed: $($stats.FixedIssues)" -ForegroundColor Yellow
}

# Exit with appropriate code for CI/CD
if (-not $ReportOnly -and ($stats.BrokenLinks -gt 0 -or $stats.MissingFiles -gt 0 -or $stats.StyleViolations -gt 0)) {
    exit 1
} else {
    exit 0
} 
