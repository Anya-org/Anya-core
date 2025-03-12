# LoggingModule.ps1
# Logging module for Anya Core
# Following Hexagonal Architecture principles for Bitcoin Development Framework

# Global variables for logging
$script:LogLevel = "Info"
$script:LogFile = $null
$script:LogLevels = @{
    "Debug" = 0
    "Info" = 1
    "Warning" = 2
    "Error" = 3
    "Critical" = 4
}

function Initialize-Logging {
    [CmdletBinding()]
    param (
        [Parameter(Mandatory = $false)]
        [ValidateSet("Debug", "Info", "Warning", "Error", "Critical")]
        [string]$LogLevel = "Info",
        
        [Parameter(Mandatory = $false)]
        [string]$LogFilePath
    )
    
    $script:LogLevel = $LogLevel
    
    # Create log directory if it doesn't exist
    $CurrentDir = Split-Path -Parent $MyInvocation.MyCommand.Path
    $LogDir = Join-Path -Path (Split-Path -Parent $CurrentDir) -ChildPath "Logs"
    
    if (-not (Test-Path $LogDir)) {
        New-Item -ItemType Directory -Path $LogDir | Out-Null
    }
    
    # If no log file path is specified, create one with timestamp
    if (-not $LogFilePath) {
        $Timestamp = Get-Date -Format "yyyyMMdd-HHmmss"
        $LogFilePath = Join-Path -Path $LogDir -ChildPath "AnyaCore-$Timestamp.log"
    }
    
    $script:LogFile = $LogFilePath
    
    # Write initial log entry
    $InitialLogMessage = "Logging initialized at level: $LogLevel"
    Write-Log $InitialLogMessage -Level Info
    
    return $script:LogFile
}

function Write-Log {
    [CmdletBinding()]
    param (
        [Parameter(Mandatory = $true, Position = 0)]
        [string]$Message,
        
        [Parameter(Mandatory = $false)]
        [ValidateSet("Debug", "Info", "Warning", "Error", "Critical")]
        [string]$Level = "Info",
        
        [Parameter(Mandatory = $false)]
        [switch]$NoConsole
    )
    
    # Check if logging is initialized
    if (-not $script:LogFile) {
        Initialize-Logging
    }
    
    # Only log if the message level is >= the current log level
    if ($script:LogLevels[$Level] -ge $script:LogLevels[$script:LogLevel]) {
        $Timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
        $LogEntry = "[$Timestamp] [$Level] $Message"
        
        # Write to log file
        Add-Content -Path $script:LogFile -Value $LogEntry
        
        # Write to console if not suppressed
        if (-not $NoConsole) {
            $ConsoleColor = switch ($Level) {
                "Debug" { "Gray" }
                "Info" { "White" }
                "Warning" { "Yellow" }
                "Error" { "Red" }
                "Critical" { "DarkRed" }
                default { "White" }
            }
            
            Write-Host $LogEntry -ForegroundColor $ConsoleColor
        }
    }
}

function Get-LogFilePath {
    [CmdletBinding()]
    param()
    
    # Check if logging is initialized
    if (-not $script:LogFile) {
        Initialize-Logging
    }
    
    return $script:LogFile
}

function Set-LogLevel {
    [CmdletBinding()]
    param (
        [Parameter(Mandatory = $true)]
        [ValidateSet("Debug", "Info", "Warning", "Error", "Critical")]
        [string]$Level
    )
    
    $script:LogLevel = $Level
    Write-Log "Log level changed to: $Level" -Level Info
}

# Specialized logging functions for Bitcoin Development Framework
function Write-SecurityLog {
    [CmdletBinding()]
    param (
        [Parameter(Mandatory = $true)]
        [string]$Message,
        
        [Parameter(Mandatory = $false)]
        [string]$Component = "General",
        
        [Parameter(Mandatory = $false)]
        [ValidateSet("Info", "Warning", "Error", "Critical")]
        [string]$Level = "Info"
    )
    
    # Add security audit trail information
    $AuditMessage = "[Security][$Component] $Message"
    Write-Log $AuditMessage -Level $Level
    
    # If critical, also log to a separate security audit file
    if ($Level -eq "Critical") {
        $CurrentDir = Split-Path -Parent $MyInvocation.MyCommand.Path
        $AuditDir = Join-Path -Path (Split-Path -Parent $CurrentDir) -ChildPath "Security"
        
        if (-not (Test-Path $AuditDir)) {
            New-Item -ItemType Directory -Path $AuditDir | Out-Null
        }
        
        $Timestamp = Get-Date -Format "yyyyMMdd"
        $AuditFile = Join-Path -Path $AuditDir -ChildPath "SecurityAudit-$Timestamp.log"
        
        $AuditEntry = "[$((Get-Date).ToString("o"))] [CRITICAL] $Message"
        Add-Content -Path $AuditFile -Value $AuditEntry
    }
}

function Write-BipComplianceLog {
    [CmdletBinding()]
    param (
        [Parameter(Mandatory = $true)]
        [int]$BipNumber,
        
        [Parameter(Mandatory = $true)]
        [string]$Message,
        
        [Parameter(Mandatory = $false)]
        [ValidateSet("Compliant", "Non-Compliant", "Partial", "Testing")]
        [string]$Status = "Testing"
    )
    
    $BipMessage = "[BIP-$BipNumber][$Status] $Message"
    Write-Log $BipMessage -Level Info
}

# Export the functions to be used by other modules
Export-ModuleMember -Function Initialize-Logging, Write-Log, Get-LogFilePath, Set-LogLevel, 
                              Write-SecurityLog, Write-BipComplianceLog 