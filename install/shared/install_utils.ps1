function Test-AdminPrivileges {
    $identity = [Security.Principal.WindowsIdentity]::GetCurrent()
    $principal = New-Object Security.Principal.WindowsPrincipal($identity)
    return $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

function Install-Prerequisites {
    # Check for required PowerShell version
    if ($PSVersionTable.PSVersion.Major -lt 5) {
        throw "PowerShell 5.0 or later is required"
    }

    # Ensure TLS 1.2 is available
    [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
}

function Write-InstallLog {
    param(
        [string]$Message,
        [string]$LogPath = "$env:TEMP\anya_install.log"
    )
    
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    "$timestamp - $Message" | Out-File -Append $LogPath
}

function Remove-TempFiles {
    param([string]$Pattern = "*.zip")
    
    Get-ChildItem -Path $env:TEMP -Filter $Pattern | 
        Where-Object { $_.CreationTime -lt (Get-Date).AddHours(-1) } |
        Remove-Item -Force
}
