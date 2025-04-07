# fix_core_package_refs.ps1
# Script to fix references to anya-core-core in Cargo.toml files

Write-Host "Starting package reference fix..."

$cargoFiles = Get-ChildItem -Path "." -Filter "Cargo.toml" -Recurse
$fixCount = 0

foreach ($file in $cargoFiles) {
    $content = Get-Content $file.FullName -Raw
    
    # Skip if this is the root Cargo.toml
    if ($file.DirectoryName -eq (Get-Location).Path) {
        Write-Host "Skipping root Cargo.toml" -ForegroundColor Yellow
        continue
    }
    
    if ($content -match "anya-core-core") {
        $newContent = $content -replace "anya-core-core", "anya-core-lib"
        Set-Content -Path $file.FullName -Value $newContent
        Write-Host "Updated $($file.FullName)" -ForegroundColor Green
        $fixCount++
    }
    else {
        Write-Host "No changes needed for $($file.FullName)" -ForegroundColor DarkGray
    }
}

Write-Host "Fixed $fixCount Cargo.toml files" -ForegroundColor Cyan
Write-Host "Run 'cargo check' to verify the changes." 