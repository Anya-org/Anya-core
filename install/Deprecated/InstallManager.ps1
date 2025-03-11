#Requires -Version 5.0
#Requires -RunAsAdministrator

# Import shared modules
Import-Module "$PSScriptRoot\shared\DashboardUI.ps1"
Import-Module "$PSScriptRoot\shared\SystemChecks.ps1"

Write-Warning @"
[DEPRECATED] InstallManager.ps1 is deprecated and will be removed in future versions.
Please use the new installation system:
    Import-Module "$PSScriptRoot\Modules\AnyaInstaller"
    $installer = New-AnyaInstaller
    $installer.Start()
"@

Start-Sleep -Seconds 5
& "$PSScriptRoot\Modules\AnyaInstaller\Start-AnyaInstaller.ps1"
exit

class InstallManager {
    [string]$Version = "3.1.0"
    [string]$LogPath
    [string]$VenvPath
    [hashtable]$Components
    [string]$DeploymentType
    [bool]$IsNetworked
    
    InstallManager() {
        $this.LogPath = "$env:TEMP\anya_install.log"
        $this.VenvPath = "$PSScriptRoot\venv"
        $this.LoadComponents()
        $this.DeploymentType = 'Standalone'
        $this.IsNetworked = $false
    }

    [bool]InitializeVenv() {
        try {
            if (-not (Test-Path $this.VenvPath)) {
                Write-Host "Creating virtual environment..." -ForegroundColor Cyan
                New-Item -ItemType Directory -Path $this.VenvPath -Force | Out-Null
                New-Item -ItemType Directory -Path "$($this.VenvPath)\Scripts" -Force | Out-Null
                
                # Copy PowerShell files
                Copy-Item "$PSHOME\powershell.exe" $this.VenvPath
                Copy-Item "$PSHOME\pwsh.dll" $this.VenvPath
                
                # Create activation script
                $activateScript = @"
# Virtual environment activation script
`$env:VIRTUAL_ENV = "$($this.VenvPath)"
`$env:PATH = "`$($this.VenvPath);`$($this.VenvPath)\Scripts;`$env:PATH"
`$env:POWERSHELL_DISTRIBUTION_CHANNEL = 'VENV'
`$env:PSExecutionPolicyPreference = 'Bypass'

function global:deactivate {
    `$env:PATH = `$env:PATH -replace [regex]::Escape("`$($this.VenvPath);`$($this.VenvPath)\Scripts;"), ''
    Remove-Item env:VIRTUAL_ENV -ErrorAction SilentlyContinue
    Remove-Item env:POWERSHELL_DISTRIBUTION_CHANNEL -ErrorAction SilentlyContinue
    Remove-Item env:PSExecutionPolicyPreference -ErrorAction SilentlyContinue
    Remove-Item function:deactivate -ErrorAction SilentlyContinue
}

Write-Host "Virtual environment activated at `$($this.VenvPath)" -ForegroundColor Green
"@ 
                $activateScript | Out-File -FilePath "$($this.VenvPath)\Scripts\Activate.ps1" -Encoding UTF8
            }

            if (-not $env:VIRTUAL_ENV) {
                . "$($this.VenvPath)\Scripts\Activate.ps1"
            }
            return $true
        }
        catch {
            Write-Host "Failed to initialize virtual environment: $_" -ForegroundColor Red
            return $false
        }
    }

    [void]CleanupVenv() {
        if ($env:VIRTUAL_ENV) {
            deactivate
        }
        if ((Test-Path $this.VenvPath) -and -not $this.InstallationSuccessful) {
            Write-Host "Cleaning up virtual environment..." -ForegroundColor Yellow
            Remove-Item -Path $this.VenvPath -Recurse -Force
        }
    }

    [bool]$InstallationSuccessful = $false

    [void]ShowInstallationMenu() {
        if (-not $this.InitializeVenv()) {
            return
        }

        try {
            while ($true) {
                Write-DashboardBlock "Anya Core Installation" @(
                    "1. Change Deployment Type (Current: $($this.DeploymentType))",
                    "2. Toggle Network Mode (Current: $($this.IsNetworked))",
                    "3. View System Requirements",
                    "4. Start Installation",
                    "5. Run Tests",
                    "6. Exit"
                )
                
                $choice = Read-Host "`nSelect option"
                switch ($choice) {
                    "1" { $this.SetDeploymentType() }
                    "2" { $this.IsNetworked = -not $this.IsNetworked }
                    "3" { $this.ShowSystemRequirements() }
                    "4" { 
                        $this.InstallationSuccessful = $this.StartInstallation()
                        if ($this.InstallationSuccessful) { 
                            return 
                        }
                    }
                    "5" { $this.RunTests() }
                    "6" { 
                        $this.CleanupVenv()
                        exit 0 
                    }
                }
            }
        }
        finally {
            $this.CleanupVenv()
        }
    }
    
    [void]SetDeploymentType() {
        Write-DashboardBlock "Select Deployment Type" @(
            "1. Standalone (Single Node)",
            "2. Network Node",
            "3. Cluster Node"
        )
        
        $choice = Read-Host "Select type"
        switch ($choice) {
            "1" { $this.DeploymentType = "Standalone" }
            "2" { $this.DeploymentType = "Node" }
            "3" { $this.DeploymentType = "Cluster" }
        }
    }
    
    [bool]StartInstallation() {
        if (-not $this.ValidateSystem()) {
            return $false
        }
        
        # Get components for deployment type
        $components = @()
        $components += $this.Components.Core
        if ($this.DeploymentType -in @('Node','Cluster')) {
            $components += $this.Components.Node
        }
        if ($this.DeploymentType -eq 'Cluster') {
            $components += $this.Components.Cluster
        }
        
        # Install components
        $total = $components.Count
        $current = 0
        
        foreach ($component in $components) {
            $current++
            $percent = [math]::Round(($current / $total) * 100)
            Show-InstallProgress "Installing $($component.Name)..." $percent
            
            if (-not (Install-Component @component)) {
                if ($component.Required) {
                    throw "Failed to install required component: $($component.Name)"
                }
            }
        }
        
        return $true
    }
    
    [void]RunTests() {
        # Import and run master test script
        & "$PSScriptRoot\..\tests\master_test.ps1"
    }
}

# Main execution
try {
    $installer = [InstallManager]::new()
    $installer.ShowInstallationMenu()
} catch {
    Write-Host "`nInstallation failed: $_" -ForegroundColor Red
    Write-InstallLog "Installation failed: $_"
    if ($installer) {
        $installer.CleanupVenv()
    }
    exit 1
}
