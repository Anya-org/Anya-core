<#!
Creates required cache directories on the Windows host F: drive for the Anya Core dev container.
Run this in a Windows PowerShell (not inside the container) BEFORE rebuilding the dev container.
#>

param(
    [int]$MinFreeGB = 10
)

$paths = @(
    'F:\anya-rust-target',
    'F:\anya-cargo-registry',
    'F:\anya-cargo-git',
    'F:\anya-sccache'
)

Write-Host "[Anya Core] Ensuring cache directories exist on F: ..." -ForegroundColor Cyan

foreach ($p in $paths) {
    if (-not (Test-Path $p)) {
        Write-Host "Creating $p" -ForegroundColor Yellow
        New-Item -ItemType Directory -Path $p | Out-Null
    } else {
        Write-Host "Exists: $p" -ForegroundColor DarkGreen
    }
}

try {
    $drive = Get-PSDrive -Name F -ErrorAction Stop
    $freeGB = [math]::Round($drive.Free / 1GB,2)
    Write-Host "Free space on F: ${freeGB} GB" -ForegroundColor Cyan
    if ($freeGB -lt $MinFreeGB) {
        Write-Warning "F: drive has less than $MinFreeGB GB free. Consider pruning or enlarging before heavy builds."
    }
} catch {
    Write-Warning "Could not query F: drive. Ensure it is mounted/accessible."
}

Write-Host "Done." -ForegroundColor Green
