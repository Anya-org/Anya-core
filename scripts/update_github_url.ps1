# GitHub URL Update Script for Windows
# This script updates all references to the old GitHub URL with the new URL

param(
    [string]$OldUrl = "github.com/anya-org/anya-core",
    [string]$NewUrl = "github.com/anya-org/anya-core",
    [string]$RootDir = ".",
    [switch]$DryRun
)

# File extensions to check
$Extensions = @(
    "*.md", "*.html", "*.yaml", "*.yml", "*.toml", "*.json", 
    "*.sh", "*.ps1", "*.rs", "*.py", "*.js", "*.ts", "*.lock",
    "*.d", "*.html", "*.txt", "*.css"
)

# Function to update URLs in files
function Update-URLs {
    param (
        [string]$FilePath
    )
    
    $content = Get-Content -Path $FilePath -Raw
    $originalContent = $content
    
    # Replace URLs (handling various formats)
    $content = $content -replace "https://github.com/anya-org/anya-core", "https://github.com/anya-org/anya-core"
    $content = $content -replace "https://github.com/anya-org/anya-core-main", "https://github.com/anya-org/anya-core-main"
    $content = $content -replace "github.com/anya-org/anya-core", "github.com/anya-org/anya-core"
    $content = $content -replace "github.com/anya-org/anya-core-main", "github.com/anya-org/anya-core-main"
    $content = $content -replace "anya-org/anya-core", "anya-org/anya-core"
    $content = $content -replace "anya-org/anya-core-main", "anya-org/anya-core-main"
    
    # Check if content was modified
    if ($content -ne $originalContent) {
        if (-not $DryRun) {
            Set-Content -Path $FilePath -Value $content
            Write-Host "Updated: $FilePath" -ForegroundColor Green
        } else {
            Write-Host "Would update: $FilePath" -ForegroundColor Yellow
        }
        return $true
    }
    
    return $false
}

# Main execution
Write-Host "Starting GitHub URL update process..."
$updateCount = 0

# Process each file extension
foreach ($ext in $Extensions) {
    $files = Get-ChildItem -Path $RootDir -Recurse -File -Filter $ext
    
    foreach ($file in $files) {
        if (Update-URLs -FilePath $file.FullName) {
            $updateCount++
        }
    }
}

# Update .git submodules
$gitmodulesPath = Join-Path $RootDir ".gitmodules"
if (Test-Path $gitmodulesPath) {
    if (Update-URLs -FilePath $gitmodulesPath) {
        $updateCount++
    }
}

# Summary
Write-Host "`nUpdate complete."
if ($DryRun) {
    Write-Host "DRY RUN: Would have updated $updateCount files." -ForegroundColor Yellow
} else {
    Write-Host "Updated $updateCount files." -ForegroundColor Green
}

# Next steps
Write-Host "`nNext steps:"
Write-Host "1. Run 'grep -r \"anya-org/anya-core\" .' to find any remaining references" -ForegroundColor Yellow
Write-Host "2. Commit changes: 'git commit -am \"Update GitHub URLs from botshelomokoka to anya-org\"'" -ForegroundColor Yellow
Write-Host "3. Run tests to ensure everything still works" -ForegroundColor Yellow 
