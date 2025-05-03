# Script to fix YAML syntax errors in GitHub workflow files
# Specifically targeting the 'on' directive format issues

param(
    [switch]$DryRun
)

$scriptName = "GitHub Workflow YAML Fixer"
$scriptVersion = "1.0.0"
$workflowDir = Join-Path $PSScriptRoot "..\..\\.github\\workflows"

Write-Host "===== $scriptName v$scriptVersion =====" -ForegroundColor Cyan
Write-Host "Fixing YAML syntax in GitHub workflow files..."

# Get all workflow files
$workflowFiles = Get-ChildItem -Path $workflowDir -Filter "*.yml" -ErrorAction SilentlyContinue

if ($workflowFiles.Count -eq 0) {
    Write-Host "No workflow files found in $workflowDir" -ForegroundColor Yellow
    exit 0
}

Write-Host "Found $($workflowFiles.Count) workflow files to process" -ForegroundColor Cyan

# Process each workflow file
foreach ($file in $workflowFiles) {
    Write-Host "Processing: $($file.Name)" -ForegroundColor Yellow
    
    # Read the file content
    $content = Get-Content -Path $file.FullName -Raw
    
    # Check if there's an issue with the 'on' directive
    if ($content -match "(?m)^on:") {
        Write-Host "  - File already has correct 'on:' format" -ForegroundColor Green
        continue
    }
    
    # This targets the common error where the 'on' directive is not properly formatted
    if ($content -match "(?m)^on\s") {
        $fixedContent = $content -replace "(?m)^on\s", "on:`n  "
        
        if ($DryRun) {
            Write-Host "  - Would fix 'on' directive syntax (dry run)" -ForegroundColor Yellow
        } else {
            # Write the fixed content back to the file
            $fixedContent | Set-Content -Path $file.FullName -Encoding UTF8
            Write-Host "  - Fixed 'on' directive syntax" -ForegroundColor Green
        }
    } else {
        Write-Host "  - No 'on' directive syntax issue found or unexpected format" -ForegroundColor Yellow
    }
}

Write-Host "YAML syntax fixing completed" -ForegroundColor Green
