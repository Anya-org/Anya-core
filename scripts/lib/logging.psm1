function Write-Log {
    param(
        [Parameter(Mandatory=$true)]
        [string]$Message,
        
        [Parameter(Mandatory=$false)]
        [ValidateSet('Info', 'Warning', 'Error', 'Success')]
        [string]$Level = 'Info'
    )
    
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    $logMessage = "[$timestamp] [$Level] $Message"
    
    switch ($Level) {
        'Info' { 
            Write-Host $logMessage -ForegroundColor White 
        }
        'Warning' { 
            Write-Host $logMessage -ForegroundColor Yellow 
        }
        'Error' { 
            Write-Host $logMessage -ForegroundColor Red 
        }
        'Success' { 
            Write-Host $logMessage -ForegroundColor Green 
        }
    }
    
    # Also write to log file
    $logFile = Join-Path $PSScriptRoot "../../logs/alignment.log"
    $logDir = Split-Path $logFile -Parent
    
    if (-not (Test-Path $logDir)) {
        New-Item -ItemType Directory -Path $logDir | Out-Null
    }
    
    Add-Content -Path $logFile -Value $logMessage
}

# Export functions
Export-ModuleMember -Function Write-Log 