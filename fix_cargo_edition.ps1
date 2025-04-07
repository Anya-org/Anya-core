# fix_cargo_edition.ps1
# Script to fix edition inheritance in Cargo.toml files

Write-Host "Starting Cargo.toml fix..."

$cargoFiles = Get-ChildItem -Path "." -Filter "Cargo.toml" -Recurse
$editionFixCount = 0
$workspaceFixCount = 0

foreach ($file in $cargoFiles) {
    $content = Get-Content $file.FullName -Raw
    $modified = $false
    
    # Skip if this is the root Cargo.toml
    if ($file.DirectoryName -eq (Get-Location).Path) {
        Write-Host "Skipping root Cargo.toml" -ForegroundColor Yellow
        continue
    }
    
    # Fix edition inheritance
    if ($content -match "edition\.workspace = true") {
        $content = $content -replace "edition\.workspace = true", "edition = `"2021`""
        $editionFixCount++
        $modified = $true
    }
    
    # Handle conflicting workspace sections in non-root files
    if ($content -match "\[workspace\]") {
        $content = $content -replace "(\[workspace\][\s\S]*?)((\[.*?\])|\Z)", "# REMOVED CONFLICTING WORKSPACE SECTION`n`$2"
        $workspaceFixCount++
        $modified = $true
    }
    
    if ($modified) {
        Set-Content -Path $file.FullName -Value $content
        Write-Host "Updated $($file.FullName)" -ForegroundColor Green
    }
    else {
        Write-Host "No changes needed for $($file.FullName)" -ForegroundColor DarkGray
    }
}

Write-Host "Fixed $editionFixCount edition inheritance issues" -ForegroundColor Cyan
Write-Host "Fixed $workspaceFixCount conflicting workspace sections" -ForegroundColor Cyan
Write-Host "Run 'cargo check' to verify the changes." 