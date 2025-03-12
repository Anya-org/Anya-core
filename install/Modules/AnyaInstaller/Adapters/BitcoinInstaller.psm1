using module ../Ports/IBitcoinInstaller.psm1

class BitcoinInstaller : IBitcoinInstaller {
    [bool]InstallBitcoinLayer([string]$Network) {
        try {
            if (-not ($Network -in @("mainnet", "testnet", "regtest"))) {
                throw "Invalid network: $Network. Must be 'mainnet', 'testnet', or 'regtest'"
            }
            Write-Host "Installing Bitcoin layer for network: $Network" -ForegroundColor Cyan
            $bitcoinDir = "$env:ProgramData\AnyaCore\bitcoin"
            if (-not (Test-Path $bitcoinDir)) {
                Write-Host "Creating Bitcoin data directory: $bitcoinDir" -ForegroundColor Cyan
                New-Item -ItemType Directory -Path $bitcoinDir -Force | Out-Null
            }
            $env:BITCOIN_NETWORK = $Network
            $env:BITCOIN_DATA_DIR = $bitcoinDir
            Write-Host "Bitcoin layer installed successfully" -ForegroundColor Green
            return $true
        }
        catch {
            Write-Host "Failed to install Bitcoin layer: $_" -ForegroundColor Red
            return $false
        }
    }
}

Export-ModuleMember -Class BitcoinInstaller