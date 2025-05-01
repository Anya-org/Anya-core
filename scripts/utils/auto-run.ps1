# Auto-run script for anya-bitcoin
# This script watches for changes and automatically rebuilds the project

Write-Host "Starting file watcher for auto-run..." -ForegroundColor Green

$watcher = New-Object System.IO.FileSystemWatcher
$watcher.Path = "$(Get-Location)"
$watcher.IncludeSubdirectories = $true
$watcher.EnableRaisingEvents = $true
$watcher.NotifyFilter = [System.IO.NotifyFilters]::LastWrite -bor [System.IO.NotifyFilters]::FileName

# Define what happens when a file is changed
$action = {
    $path = $Event.SourceEventArgs.FullPath
    $changeType = $Event.SourceEventArgs.ChangeType
    $timeStamp = $Event.TimeGenerated
    
    # Only rebuild if it's a .rs file that changed
    if ($path -like "*.rs") {
        Write-Host "`nFile $path was $changeType at $timeStamp" -ForegroundColor Yellow
        Write-Host "Starting build..." -ForegroundColor Cyan
        cargo check -p anya-bitcoin --lib --no-default-features
    }
}

# Register the event handlers
Register-ObjectEvent -InputObject $watcher -EventName Changed -Action $action | Out-Null
Register-ObjectEvent -InputObject $watcher -EventName Created -Action $action | Out-Null

Write-Host "Watcher started. Press Ctrl+C to stop."
while ($true) { Start-Sleep -Seconds 1 }
