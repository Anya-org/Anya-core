Write-Host "=== Anya Core Installer Dashboard ===" -ForegroundColor Cyan
Write-Host "1. Run Standalone Installation"
Write-Host "2. Run Node Installation (Networked)"
Write-Host "3. Run Cluster Installation (Networked)"
Write-Host "4. Run System Checks"
Write-Host "5. Re-run Virtual Environment Setup"
Write-Host "6. Exit"
$choice = Read-Host "Enter your selection (1-6)"
switch ($choice) {
    "1" {
         Write-Host "Running Standalone Installation..." -ForegroundColor Cyan
         $result = Install-AnyaCore -DeploymentType "Standalone"
         Write-Host "Installation Result: $result" -ForegroundColor Green
    }
    "2" {
         Write-Host "Running Node Installation..." -ForegroundColor Cyan
         $result = Install-AnyaCore -DeploymentType "Node" -IsNetworked $true
         Write-Host "Installation Result: $result" -ForegroundColor Green
    }
    "3" {
         Write-Host "Running Cluster Installation..." -ForegroundColor Cyan
         $result = Install-AnyaCore -DeploymentType "Cluster" -IsNetworked $true
         Write-Host "Installation Result: $result" -ForegroundColor Green
    }
    "4" {
         Write-Host "Running System Checks..." -ForegroundColor Cyan
         $checks = Test-SystemChecks
         Write-Host "System Checks Details:" -ForegroundColor Yellow
         Write-Output $checks
    }
    "5" {
         Write-Host "Re-running Virtual Environment Setup..." -ForegroundColor Cyan
         $venvManager = (Import-Module -Name "./Modules/AnyaInstaller/Adapters/VenvManager.psm1" -PassThru)
         if ($venvManager.InitializeVenv("$env:ProgramData\AnyaCore\venv", "3.9", "21.3.1")) {
             Write-Host "Virtual environment re-initialized successfully." -ForegroundColor Green
         }
         else {
             Write-Host "Failed to re-initialize virtual environment." -ForegroundColor Red
         }
    }
    "6" {
         Write-Host "Exiting installer dashboard." -ForegroundColor Cyan
    }
    default {
         Write-Host "Invalid selection. Exiting." -ForegroundColor Red
    }
}
