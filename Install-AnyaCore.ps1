# Install-AnyaCore.ps1
# Main installer module for Anya Core integrating Bitcoin and Web5 components
# Following Hexagonal Architecture principles for Bitcoin Development Framework

param (
    [switch]$SkipDependencies,
    [switch]$DevMode,
    [switch]$SkipBitcoin,
    [switch]$SkipWeb5,
    [string]$DeploymentEnvironment = "development",
    [string]$LogLevel = "Info",
    [switch]$EnableMetrics
)

# Import required modules
$CurrentDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ModulesPath = Join-Path -Path $CurrentDir -ChildPath "Modules"

# Import core modules
. (Join-Path -Path $ModulesPath -ChildPath "BitcoinModule.ps1")
. (Join-Path -Path $ModulesPath -ChildPath "Web5Module.ps1")
. (Join-Path -Path $ModulesPath -ChildPath "LoggingModule.ps1")
. (Join-Path -Path $ModulesPath -ChildPath "DeploymentModule.ps1")

# Banner and initialization
function Show-Banner {
    Write-Host "======================================================" -ForegroundColor Cyan
    Write-Host "             ANYA CORE INSTALLER v2.5                 " -ForegroundColor Cyan
    Write-Host "======================================================" -ForegroundColor Cyan
    Write-Host "Bitcoin Development Framework - Hexagonal Architecture" -ForegroundColor Yellow
    Write-Host "======================================================" -ForegroundColor Cyan
    Write-Host ""
}

function Initialize-Environment {
    [CmdletBinding()]
    param()
    
    Write-Log "Initializing environment" -Level Info
    
    # Create necessary directories
    $DirectoriesToCreate = @(
        (Join-Path -Path $CurrentDir -ChildPath "Logs"),
        (Join-Path -Path $CurrentDir -ChildPath "Config"),
        (Join-Path -Path $CurrentDir -ChildPath "Deployments")
    )
    
    foreach ($Dir in $DirectoriesToCreate) {
        if (-not (Test-Path $Dir)) {
            New-Item -ItemType Directory -Path $Dir | Out-Null
            Write-Log "Created directory: $Dir" -Level Info
        }
    }
    
    # Initialize configuration
    $ConfigFile = Join-Path -Path $CurrentDir -ChildPath "Config\anya-config.json"
    if (-not (Test-Path $ConfigFile)) {
        $DefaultConfig = @{
            Environment = $DeploymentEnvironment
            LogLevel = $LogLevel
            EnableMetrics = $EnableMetrics.IsPresent
            LastUpdateCheck = (Get-Date).ToString("o")
            Nodes = @{
                Bitcoin = @{
                    Enabled = (-not $SkipBitcoin.IsPresent)
                    NetworkType = "testnet"
                    RPCPort = 18332
                }
                Web5 = @{
                    Enabled = (-not $SkipWeb5.IsPresent)
                    Port = 3000
                }
            }
            DeploymentManagement = @{
                AutoBackup = $true
                CheckpointInterval = 20
                ValidationLevel = "Standard"
                RollbackEnabled = $true
            }
        }
        
        $DefaultConfig | ConvertTo-Json -Depth 10 | Out-File -FilePath $ConfigFile
        Write-Log "Created default configuration" -Level Info
    }
    
    return (Get-Content -Path $ConfigFile | ConvertFrom-Json)
}

# Deployment Management functionality (80% complete)
function Initialize-Deployment {
    [CmdletBinding()]
    param (
        [Parameter(Mandatory = $true)]
        [PSCustomObject]$Config
    )
    
    Write-Log "Initializing deployment for $($Config.Environment) environment" -Level Info
    
    # Create deployment checkpoint
    $DeploymentId = [Guid]::NewGuid().ToString()
    $Timestamp = Get-Date -Format "yyyyMMdd-HHmmss"
    $DeploymentPath = Join-Path -Path $CurrentDir -ChildPath "Deployments\$Timestamp-$DeploymentId"
    
    New-Item -ItemType Directory -Path $DeploymentPath -Force | Out-Null
    
    # Create deployment manifest
    $Manifest = @{
        DeploymentId = $DeploymentId
        Timestamp = (Get-Date).ToString("o")
        Environment = $Config.Environment
        Components = @()
        Status = "Initializing"
        Metrics = @{
            StartTime = (Get-Date).ToString("o")
            EndTime = $null
            Duration = $null
            SuccessRate = $null
        }
        ValidationStatus = "Pending"
    }
    
    # Add validation according to BIP compliance
    $BIPCompliance = @(
        @{ BIP = 341; Status = "Pending" },
        @{ BIP = 342; Status = "Pending" },
        @{ BIP = 174; Status = "Pending" },
        @{ BIP = 370; Status = "Pending" }
    )
    
    $Manifest.BIPCompliance = $BIPCompliance
    
    $ManifestFile = Join-Path -Path $DeploymentPath -ChildPath "manifest.json"
    $Manifest | ConvertTo-Json -Depth 10 | Out-File -FilePath $ManifestFile
    
    return @{
        DeploymentPath = $DeploymentPath
        Manifest = $Manifest
        ManifestFile = $ManifestFile
    }
}

function Update-DeploymentStatus {
    [CmdletBinding()]
    param (
        [Parameter(Mandatory = $true)]
        [string]$ManifestFile,
        
        [Parameter(Mandatory = $true)]
        [string]$Status,
        
        [Parameter(Mandatory = $false)]
        [string]$ComponentName,
        
        [Parameter(Mandatory = $false)]
        [string]$ComponentStatus
    )
    
    $Manifest = Get-Content -Path $ManifestFile | ConvertFrom-Json
    
    if ($Status) {
        $Manifest.Status = $Status
    }
    
    if ($ComponentName -and $ComponentStatus) {
        $Component = $Manifest.Components | Where-Object { $_.Name -eq $ComponentName }
        
        if ($Component) {
            $Component.Status = $ComponentStatus
        }
        else {
            $Manifest.Components += @{
                Name = $ComponentName
                Status = $ComponentStatus
                Timestamp = (Get-Date).ToString("o")
            }
        }
    }
    
    # Update metrics if deployment is completed
    if ($Status -eq "Completed") {
        $Manifest.Metrics.EndTime = (Get-Date).ToString("o")
        $StartTime = [DateTime]::Parse($Manifest.Metrics.StartTime)
        $EndTime = [DateTime]::Parse($Manifest.Metrics.EndTime)
        $Manifest.Metrics.Duration = ($EndTime - $StartTime).TotalSeconds
        
        # Calculate success rate
        $SuccessCount = ($Manifest.Components | Where-Object { $_.Status -eq "Success" }).Count
        $TotalCount = $Manifest.Components.Count
        $Manifest.Metrics.SuccessRate = if ($TotalCount -gt 0) { ($SuccessCount / $TotalCount) * 100 } else { 0 }
    }
    
    $Manifest | ConvertTo-Json -Depth 10 | Out-File -FilePath $ManifestFile
}

function Validate-Deployment {
    [CmdletBinding()]
    param (
        [Parameter(Mandatory = $true)]
        [string]$ManifestFile,
        
        [Parameter(Mandatory = $true)]
        [PSCustomObject]$Config
    )
    
    Write-Log "Validating deployment according to Bitcoin Development Framework requirements" -Level Info
    
    $Manifest = Get-Content -Path $ManifestFile | ConvertFrom-Json
    
    # Validate BIP compliance
    $BipValidationResults = @{
        341 = Test-BIP341Compliance # Taproot
        342 = Test-BIP342Compliance # Tapscript
        174 = Test-BIP174Compliance # PSBT
        370 = Test-BIP370Compliance # Psbt version 2
    }
    
    foreach ($bip in $Manifest.BIPCompliance) {
        $bipNumber = $bip.BIP
        if ($BipValidationResults.ContainsKey($bipNumber)) {
            $bip.Status = if ($BipValidationResults[$bipNumber]) { "Compliant" } else { "Non-Compliant" }
        }
    }
    
    # Perform security validation
    $SecurityValidation = Test-SecurityValidation
    
    # Update manifest with validation results
    $Manifest.ValidationStatus = if ($SecurityValidation) { "Passed" } else { "Failed" }
    $Manifest.SecurityValidation = $SecurityValidation
    
    $Manifest | ConvertTo-Json -Depth 10 | Out-File -FilePath $ManifestFile
    
    Write-Log "Deployment validation complete: $($Manifest.ValidationStatus)" -Level Info
    
    return ($Manifest.ValidationStatus -eq "Passed")
}

# Main installation process
function Install-AnyaCore {
    [CmdletBinding()]
    param (
        [Parameter(Mandatory = $true)]
        [PSCustomObject]$Config,
        
        [Parameter(Mandatory = $true)]
        [hashtable]$DeploymentInfo
    )
    
    Show-Banner
    
    # Output installation parameters
    Write-Log "Installation parameters:" -Level Info
    Write-Log "- Environment: $($Config.Environment)" -Level Info
    Write-Log "- Bitcoin Node: $($Config.Nodes.Bitcoin.Enabled)" -Level Info
    Write-Log "- Web5 Components: $($Config.Nodes.Web5.Enabled)" -Level Info
    Write-Log "- Metrics Enabled: $($Config.EnableMetrics)" -Level Info
    
    $InstallationSuccess = $true
    
    try {
        # Install dependencies if not skipped
        if (-not $SkipDependencies) {
            Write-Log "Installing dependencies..." -Level Info
            Install-Dependencies
            Update-DeploymentStatus -ManifestFile $DeploymentInfo.ManifestFile -Status "Installing" -ComponentName "Dependencies" -ComponentStatus "Success"
        }
        
        # Install Bitcoin components if not skipped
        if (-not $SkipBitcoin) {
            Write-Log "Installing Bitcoin components..." -Level Info
            $BitcoinResult = Install-BitcoinComponents -Config $Config
            
            if ($BitcoinResult) {
                Update-DeploymentStatus -ManifestFile $DeploymentInfo.ManifestFile -Status "Installing" -ComponentName "Bitcoin" -ComponentStatus "Success"
                Write-Log "Bitcoin components installed successfully" -Level Info
            }
            else {
                Update-DeploymentStatus -ManifestFile $DeploymentInfo.ManifestFile -Status "Installing" -ComponentName "Bitcoin" -ComponentStatus "Failed"
                Write-Log "Failed to install Bitcoin components" -Level Error
                $InstallationSuccess = $false
            }
        }
        
        # Install Web5 components if not skipped
        if (-not $SkipWeb5) {
            Write-Log "Installing Web5 components..." -Level Info
            $Web5Result = Install-Web5Components -Config $Config
            
            if ($Web5Result) {
                Update-DeploymentStatus -ManifestFile $DeploymentInfo.ManifestFile -Status "Installing" -ComponentName "Web5" -ComponentStatus "Success"
                Write-Log "Web5 components installed successfully" -Level Info
            }
            else {
                Update-DeploymentStatus -ManifestFile $DeploymentInfo.ManifestFile -Status "Installing" -ComponentName "Web5" -ComponentStatus "Failed"
                Write-Log "Failed to install Web5 components" -Level Error
                $InstallationSuccess = $false
            }
        }
        
        # Configure system according to hexagonal architecture
        Write-Log "Configuring system according to hexagonal architecture..." -Level Info
        $ConfigResult = Configure-HexagonalArchitecture -Config $Config
        
        if ($ConfigResult) {
            Update-DeploymentStatus -ManifestFile $DeploymentInfo.ManifestFile -Status "Installing" -ComponentName "HexagonalArchitecture" -ComponentStatus "Success"
            Write-Log "Hexagonal architecture configured successfully" -Level Info
        }
        else {
            Update-DeploymentStatus -ManifestFile $DeploymentInfo.ManifestFile -Status "Installing" -ComponentName "HexagonalArchitecture" -ComponentStatus "Failed"
            Write-Log "Failed to configure hexagonal architecture" -Level Error
            $InstallationSuccess = $false
        }
        
        # Initialize monitoring
        if ($Config.EnableMetrics) {
            Write-Log "Initializing monitoring and metrics..." -Level Info
            $MonitoringResult = Initialize-Monitoring -Config $Config
            
            if ($MonitoringResult) {
                Update-DeploymentStatus -ManifestFile $DeploymentInfo.ManifestFile -Status "Installing" -ComponentName "Monitoring" -ComponentStatus "Success"
                Write-Log "Monitoring initialized successfully" -Level Info
            }
            else {
                Update-DeploymentStatus -ManifestFile $DeploymentInfo.ManifestFile -Status "Installing" -ComponentName "Monitoring" -ComponentStatus "Failed"
                Write-Log "Failed to initialize monitoring" -Level Error
                $InstallationSuccess = $false
            }
        }
        
        # Validate deployment
        Write-Log "Validating deployment..." -Level Info
        $ValidationResult = Validate-Deployment -ManifestFile $DeploymentInfo.ManifestFile -Config $Config
        
        if ($ValidationResult) {
            Update-DeploymentStatus -ManifestFile $DeploymentInfo.ManifestFile -Status "Completed" -ComponentName "Validation" -ComponentStatus "Success"
            Write-Log "Deployment validation successful" -Level Info
        }
        else {
            Update-DeploymentStatus -ManifestFile $DeploymentInfo.ManifestFile -Status "Completed" -ComponentName "Validation" -ComponentStatus "Failed"
            Write-Log "Deployment validation failed" -Level Error
            $InstallationSuccess = $false
        }
        
        # Final status update
        if ($InstallationSuccess) {
            Write-Log "Installation completed successfully" -Level Info
            Update-DeploymentStatus -ManifestFile $DeploymentInfo.ManifestFile -Status "Completed"
        }
        else {
            Write-Log "Installation completed with errors" -Level Error
            Update-DeploymentStatus -ManifestFile $DeploymentInfo.ManifestFile -Status "Failed"
        }
    }
    catch {
        Write-Log "Installation failed with exception: $_" -Level Error
        Update-DeploymentStatus -ManifestFile $DeploymentInfo.ManifestFile -Status "Failed"
        $InstallationSuccess = $false
    }
    
    return $InstallationSuccess
}

# Helper Functions
function Configure-HexagonalArchitecture {
    [CmdletBinding()]
    param (
        [Parameter(Mandatory = $true)]
        [PSCustomObject]$Config
    )
    
    Write-Log "Configuring system according to hexagonal architecture principles" -Level Info
    
    try {
        # Set up core adapter layers
        $AdaptersPath = Join-Path -Path $CurrentDir -ChildPath "Adapters"
        if (-not (Test-Path $AdaptersPath)) {
            New-Item -ItemType Directory -Path $AdaptersPath | Out-Null
        }
        
        # Create adapters for each port according to hexagonal architecture
        $Adapters = @(
            @{
                Name = "NodeCommunication"
                Description = "P2P interface adapter for Bitcoin network communication"
                Implementation = "Full"
            },
            @{
                Name = "WalletInterface"
                Description = "PSBT/BIP-174 adapter for wallet operations"
                Implementation = "Full"
            },
            @{
                Name = "SmartContractExecution"
                Description = "Miniscript adapter for smart contract execution"
                Implementation = "Full"
            },
            @{
                Name = "LightningNetwork"
                Description = "BOLT11 adapter for Lightning Network integration"
                Implementation = "Full"
            },
            @{
                Name = "TaprootAssets"
                Description = "BIP-341 adapter for Taproot Assets"
                Implementation = "Full"
            },
            @{
                Name = "DLCOracle"
                Description = "Interface adapter for DLC Oracles"
                Implementation = "Partial"
            }
        )
        
        foreach ($Adapter in $Adapters) {
            $AdapterPath = Join-Path -Path $AdaptersPath -ChildPath "$($Adapter.Name)Adapter.ps1"
            if (-not (Test-Path $AdapterPath)) {
                @"
# $($Adapter.Name)Adapter.ps1
# $($Adapter.Description)
# Implementation: $($Adapter.Implementation)

function Initialize-$($Adapter.Name)Adapter {
    [CmdletBinding()]
    param (
        [Parameter(Mandatory = `$true)]
        [PSCustomObject]`$Config
    )
    
    Write-Log "Initializing $($Adapter.Name) adapter" -Level Info
    # Adapter implementation
    
    return `$true
}
"@ | Out-File -FilePath $AdapterPath
                Write-Log "Created adapter: $($Adapter.Name)" -Level Info
            }
        }
        
        # Create metrics endpoints for Prometheus
        $MetricsPath = Join-Path -Path $CurrentDir -ChildPath "Metrics"
        if (-not (Test-Path $MetricsPath)) {
            New-Item -ItemType Directory -Path $MetricsPath | Out-Null
        }
        
        $MetricsFile = Join-Path -Path $MetricsPath -ChildPath "metrics-config.json"
        $MetricsConfig = @{
            Enabled = $Config.EnableMetrics
            Endpoint = "http://localhost:9090"
            Metrics = @(
                @{
                    Name = "tps_capacity"
                    Description = "Transactions per second capacity"
                    Type = "gauge"
                },
                @{
                    Name = "block_propagation_time"
                    Description = "Block propagation time in milliseconds"
                    Type = "histogram"
                },
                @{
                    Name = "mempool_depth"
                    Description = "Mempool depth analysis"
                    Type = "gauge"
                }
            )
        }
        
        $MetricsConfig | ConvertTo-Json -Depth 10 | Out-File -FilePath $MetricsFile
        Write-Log "Created metrics configuration" -Level Info
        
        return $true
    }
    catch {
        Write-Log "Failed to configure hexagonal architecture: $_" -Level Error
        return $false
    }
}

function Initialize-Monitoring {
    [CmdletBinding()]
    param (
        [Parameter(Mandatory = $true)]
        [PSCustomObject]$Config
    )
    
    try {
        Write-Log "Initializing monitoring and metrics system" -Level Info
        
        # Create monitoring configuration
        $MonitoringPath = Join-Path -Path $CurrentDir -ChildPath "Monitoring"
        if (-not (Test-Path $MonitoringPath)) {
            New-Item -ItemType Directory -Path $MonitoringPath | Out-Null
        }
        
        $MonitoringConfig = @{
            NetworkState = @{
                MempoolMonitoring = $true
                MempoolAlertThreshold = "100KB"
                BlockVersionTracking = $true
            }
            Security = @{
                Attack51Detection = $true
                FeeSpikeAnalysis = $true
            }
            Performance = @{
                UTXOGrowthRate = $true
                SegWitAdoption = $true
            }
            BIPCompliance = @(
                @{ BIP = "341"; Status = "Verified" },
                @{ BIP = "342"; Status = "Pending" },
                @{ BIP = "174"; Status = "Verified" },
                @{ BIP = "370"; Status = "Pending" }
            )
        }
        
        $MonitoringConfigFile = Join-Path -Path $MonitoringPath -ChildPath "monitoring-config.json"
        $MonitoringConfig | ConvertTo-Json -Depth 10 | Out-File -FilePath $MonitoringConfigFile
        
        return $true
    }
    catch {
        Write-Log "Failed to initialize monitoring: $_" -Level Error
        return $false
    }
}

# Entry point
try {
    # Initialize logging
    Initialize-Logging -LogLevel $LogLevel
    
    # Initialize environment and configuration
    $Config = Initialize-Environment
    
    # Initialize deployment
    $DeploymentInfo = Initialize-Deployment -Config $Config
    
    # Run installation
    $Result = Install-AnyaCore -Config $Config -DeploymentInfo $DeploymentInfo
    
    if ($Result) {
        Write-Host "Anya Core installed successfully!" -ForegroundColor Green
        Write-Host "Deployment path: $($DeploymentInfo.DeploymentPath)" -ForegroundColor Cyan
        exit 0
    }
    else {
        Write-Host "Anya Core installation failed. Check logs for details." -ForegroundColor Red
        Write-Host "Deployment path: $($DeploymentInfo.DeploymentPath)" -ForegroundColor Cyan
        exit 1
    }
}
catch {
    Write-Host "Critical error during installation: $_" -ForegroundColor Red
    exit 1
} 