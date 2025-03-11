class InstallationConfig {
    [string]$DeploymentType = 'Standalone'
    [bool]$IsNetworked = $false
    [hashtable]$Components

    InstallationConfig() {
        $this.LoadComponents()
    }

    [void]LoadComponents() {
        $this.Components = @{
            Core = @(
                @{
                    Name = "Bitcoin Core"
                    Required = $true
                    Script = "$PSScriptRoot\..\components\install_bitcoin.ps1"
                    Version = "24.0.1"
                    Dependencies = @()
                }
            )
            Node = @(
                @{
                    Name = "Lightning Network Daemon"
                    Required = $false
                    Script = "$PSScriptRoot\..\components\install_lnd.ps1"
                    Version = "0.17.0-beta"
                    Dependencies = @("Bitcoin Core")
                }
            )
            Cluster = @(
                @{
                    Name = "RGB Node"
                    Required = $false
                    Script = "$PSScriptRoot\..\components\install_rgb.ps1"
                    Version = "0.9.0"
                    Dependencies = @("Bitcoin Core")
                }
            )
        }
    }

    # ...existing methods...
}

Export-ModuleMember -Function *
