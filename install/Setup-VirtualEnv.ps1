# Create and configure virtual environment for Anya Core installation

$venvPath = "$PSScriptRoot\venv"
$venvScriptsPath = "$venvPath\Scripts"

# Create virtual environment directory if it doesn't exist
if (-not (Test-Path $venvPath)) {
    Write-Host "Creating virtual environment..." -ForegroundColor Cyan
    New-Item -ItemType Directory -Path $venvPath -Force | Out-Null
    
    # Copy necessary PowerShell files
    Copy-Item "$PSHOME\powershell.exe" "$venvPath"
    Copy-Item "$PSHOME\pwsh.dll" "$venvPath"
    
    # Create activation script
    @"
# Virtual environment activation script
`$env:VIRTUAL_ENV = "$venvPath"
`$env:PATH = "`$venvPath;`$venvScriptsPath;`$env:PATH"
`$env:POWERSHELL_DISTRIBUTION_CHANNEL = 'VENV'
`$env:PSExecutionPolicyPreference = 'Bypass'

function global:deactivate {
    # Reset environment variables
    `$env:PATH = `$env:PATH -replace [regex]::Escape("`$venvPath;`$venvScriptsPath;"), ''
    Remove-Item env:VIRTUAL_ENV -ErrorAction SilentlyContinue
    Remove-Item env:POWERSHELL_DISTRIBUTION_CHANNEL -ErrorAction SilentlyContinue
    Remove-Item env:PSExecutionPolicyPreference -ErrorAction SilentlyContinue
    
    # Remove function
    Remove-Item function:deactivate -ErrorAction SilentlyContinue
}

Write-Host "Virtual environment activated at `$venvPath" -ForegroundColor Green
Write-Host "Type 'deactivate' to exit the virtual environment" -ForegroundColor Yellow
"@ | Out-File -FilePath "$venvScriptsPath\Activate.ps1" -Encoding UTF8

    Write-Host "Virtual environment created successfully" -ForegroundColor Green
}

# Launch new PowerShell session in virtual environment
Write-Host "Launching virtual environment..." -ForegroundColor Cyan
Start-Process powershell -ArgumentList "-NoExit", "-Command", "cd '$PSScriptRoot'; . '$venvScriptsPath\Activate.ps1'; .\Launch-Installer.ps1"
