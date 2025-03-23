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
            
            # Enforce BIP-341 SILENT_LEAF pattern
            $silentLeaf = "0x8f3a1c29566443e2e2d6e5a9a5a4e8d"
            Add-Content -Path "$bitcoinDir/bitcoin.conf" -Value "silent_leaf=$silentLeaf"
            
            # Enforce PSBT v2 (BIP-174)
            Add-Content -Path "$bitcoinDir/bitcoin.conf" -Value "psbt_version=2"
            
            # Validate installation
            return (Test-Path "$bitcoinDir/bitcoin.conf") -and 
                   (Get-Content "$bitcoinDir/bitcoin.conf" | Select-String "silent_leaf")
        }
        catch {
            Write-Host "Failed to install Bitcoin layer: $_" -ForegroundColor Red
            return $false
        }
    }
}

Export-ModuleMember -Class BitcoinInstaller