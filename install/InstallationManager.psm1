#Requires -Version 5.0
#Requires -RunAsAdministrator

using module ./Models/InstallationConfig.psm1
using module ./Models/TestConfig.psm1

# Import shared modules
Import-Module "$PSScriptRoot\shared\DashboardUI.ps1"
Import-Module "$PSScriptRoot\shared\SystemChecks.ps1"

class InstallationManager {
    [string]$Version = "3.1.0"
    [InstallationConfig]$Config
    [TestConfig]$TestConfig
    [string]$LogPath
    [string]$VenvPath
    hidden [bool]$InstallationSuccessful = $false

    InstallationManager() {
        $this.LogPath = "$env:TEMP\anya_install.log"
        $this.VenvPath = "$PSScriptRoot\venv"
        $this.Config = [InstallationConfig]::new()
        $this.TestConfig = [TestConfig]::new()
        $this.InitializeEnvironment()
    }

    [void]InitializeEnvironment() {
        if (-not $this.InitializeVenv()) {
            throw "Failed to initialize virtual environment"
        }
        Write-InstallLog "Environment initialized successfully"
    }

    [void]Start() {
        try {
            while ($true) {
                $this.ShowMainMenu()
                $choice = Read-Host "`nSelect option"
                
                switch ($choice) {
                    "1" { $this.ShowInstallationMenu() }
                    "2" { $this.ShowTestMenu() }
                    "3" { $this.ShowConfigurationMenu() }
                    "4" { 
                        $this.Cleanup()
                        exit 0 
                    }
                }
            }
        }
        finally {
            if (-not $this.InstallationSuccessful) {
                $this.Cleanup()
            }
        }
    }

    [void]ShowMainMenu() {
        Clear-Host
        Write-DashboardBlock "Anya Core Platform Manager" @(
            "1. Installation Management",
            "2. Test Management",
            "3. Configuration",
            "4. Exit"
        )
    }

    [void]ShowInstallationMenu() {
        while ($true) {
            Write-DashboardBlock "Installation Management" @(
                "1. Change Deployment Type (Current: $($this.Config.DeploymentType))",
                "2. Toggle Network Mode (Current: $($this.Config.IsNetworked))",
                "3. View System Requirements",
                "4. Start Installation",
                "5. Back to Main Menu"
            )
            
            switch (Read-Host "`nSelect option") {
                "1" { $this.Config.SetDeploymentType() }
                "2" { $this.Config.ToggleNetworkMode() }
                "3" { $this.ShowSystemRequirements() }
                "4" { 
                    $this.InstallationSuccessful = $this.RunInstallation()
                    if ($this.InstallationSuccessful) { return }
                }
                "5" { return }
            }
        }
    }

    [void]ShowTestMenu() {
        while ($true) {
            Write-DashboardBlock "Test Management" @(
                "1. Run All Tests",
                "2. Select Test Categories",
                "3. View Test Results",
                "4. Back to Main Menu"
            )
            
            switch (Read-Host "`nSelect option") {
                "1" { $this.TestConfig.RunAllTests() }
                "2" { $this.TestConfig.SelectAndRunTests() }
                "3" { $this.TestConfig.ShowTestResults() }
                "4" { return }
            }
        }
    }

    [void]Cleanup() {
        if ($env:VIRTUAL_ENV) {
            deactivate
        }
        if ((Test-Path $this.VenvPath) -and -not $this.InstallationSuccessful) {
            Write-Host "Cleaning up installation..." -ForegroundColor Yellow
            Remove-Item -Path $this.VenvPath -Recurse -Force
        }
    }

    # ...existing private methods...
}

# Export the module
Export-ModuleMember -Function * -Variable *
