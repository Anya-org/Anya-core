Describe "BitcoinInstaller" {
    Context "InstallBitcoinLayer" {
        It "Should install Bitcoin layer for mainnet" {
            $bitcoinInstaller = [BitcoinInstaller]::new()
            $bitcoinInstaller.InstallBitcoinLayer("mainnet") | Should -Be $true
        }

        It "Should throw for invalid network" {
            $bitcoinInstaller = [BitcoinInstaller]::new()
            { $bitcoinInstaller.InstallBitcoinLayer("invalid") } | Should -Throw
        }
    }
}