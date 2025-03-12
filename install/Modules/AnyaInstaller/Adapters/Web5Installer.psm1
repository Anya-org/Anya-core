using module ../Ports/IWeb5Installer.psm1

class Web5Installer : IWeb5Installer {
    [bool]InstallWeb5Layer() {
        try {
            Write-Host "Installing Web5 layer..." -ForegroundColor Cyan
            
            # Create Web5 storage directory
            $web5Dir = "$env:ProgramData\AnyaCore\web5"
            if (-not (Test-Path $web5Dir)) {
                Write-Host "Creating Web5 storage directory: $web5Dir" -ForegroundColor Cyan
                New-Item -ItemType Directory -Path $web5Dir -Force | Out-Null
            }
            
            # Configure Web5 environment variables
            $env:WEB5_STORAGE_PATH = $web5Dir
            $env:WEB5_API_PORT = "7000"
            
            Write-Host "Web5 layer installed successfully" -ForegroundColor Green
            return $true
        }
        catch {
            Write-Host "Failed to install Web5 layer: $_" -ForegroundColor Red
            return $false
        }
    }
}

Export-ModuleMember -Class Web5Installer