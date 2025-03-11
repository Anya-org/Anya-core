$ErrorActionPreference = "Stop"

$bitcoinCore = @{
    Version = "24.0.1"
    Url = "https://bitcoin.org/bin/bitcoin-core-24.0.1/bitcoin-24.0.1-win64.zip"
    InstallPath = "$env:PROGRAMFILES\Bitcoin"
    DataPath = "$env:APPDATA\Bitcoin"
}

# Create directories
New-Item -ItemType Directory -Force -Path $bitcoinCore.InstallPath | Out-Null
New-Item -ItemType Directory -Force -Path $bitcoinCore.DataPath | Out-Null

# Download and extract Bitcoin Core
$downloadPath = "$env:TEMP\bitcoin.zip"
Invoke-WebRequest -Uri $bitcoinCore.Url -OutFile $downloadPath
Expand-Archive -Path $downloadPath -DestinationPath $bitcoinCore.InstallPath -Force

# Create basic configuration
@"
server=1
rpcuser=anyacore
rpcpassword=$(New-Guid)
txindex=1
"@ | Out-File "$($bitcoinCore.DataPath)\bitcoin.conf"

# Add to PATH
$env:Path += ";$($bitcoinCore.InstallPath)\bin"
[Environment]::SetEnvironmentVariable("Path", $env:Path, [EnvironmentVariableTarget]::User)
