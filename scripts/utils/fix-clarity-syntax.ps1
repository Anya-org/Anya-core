# Fix Clarity Syntax Script
Write-Host "================================================================" -ForegroundColor Cyan
Write-Host "--- Anya DAO - Clarity Contract Syntax Fixer                 ---" -ForegroundColor Cyan
Write-Host "================================================================" -ForegroundColor Cyan

# Find all Clarity contracts
$contractFiles = Get-ChildItem -Path "." -Include "*.clar" -Recurse

Write-Host "Found $($contractFiles.Count) Clarity contract files to process" -ForegroundColor Yellow

foreach ($file in $contractFiles) {
    Write-Host "Processing $($file.FullName)" -ForegroundColor Yellow
    
    # Read the file content
    $content = Get-Content -Path $file.FullName -Raw
    
    # Fix common syntax issues
    $fixedContent = $content
    
    # Fix missing semicolons after define statements
    $fixedContent = $fixedContent -replace '(\(define[^;]*?\))(\s*\()', '$1;$2'
    
    # Fix missing semicolons at end of expressions
    $fixedContent = $fixedContent -replace '(\)\s*)\r?\n(\s*\()', '$1;$2'
    
    # Fix missing semicolons before closing parentheses
    $fixedContent = $fixedContent -replace '([^;])\s*\)', '$1;)'
    
    # Save the fixed content
    if ($content -ne $fixedContent) {
        Set-Content -Path $file.FullName -Value $fixedContent
        Write-Host "  ✅ Fixed syntax issues in $($file.Name)" -ForegroundColor Green
    } else {
        Write-Host "  ✓ No issues found in $($file.Name)" -ForegroundColor Green
    }
}

Write-Host "`nSyntax fixing completed. All contracts have been processed." -ForegroundColor Green 