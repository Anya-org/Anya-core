$ErrorActionPreference = "Stop"

$lnd = @{
    Version = "0.17.0-beta"
    Url = "https://github.com/lightningnetwork/lnd/releases/download/v0.17.0-beta/lnd-windows-amd64-v0.17.0-beta.zip"
    InstallPath = "$env:PROGRAMFILES\Lnd"
    DataPath = "$env:APPDATA\Lnd"
}

# Create directories
New-Item -ItemType Directory -Force -Path $lnd.InstallPath | Out-Null
New-Item -ItemType Directory -Force -Path $lnd.DataPath | Out-Null

# Download and extract LND
$downloadPath = "$env:TEMP\lnd.zip"
Invoke-WebRequest -Uri $lnd.Url -OutFile $downloadPath
Expand-Archive -Path $downloadPath -DestinationPath $lnd.InstallPath -Force

# Create basic configuration
@"
[Application Options]
debuglevel=info
maxpendingchannels=5
listen=0.0.0.0:9735
restlisten=0.0.0.0:8080
rpclisten=0.0.0.0:10009

[Bitcoin]
bitcoin.active=1
bitcoin.mainnet=1
bitcoin.node=bitcoind

[Bitcoind]
bitcoind.rpcuser=anyacore
bitcoind.rpcpass=$(New-Guid)
bitcoind.zmqpubrawblock=tcp://127.0.0.1:28332
bitcoind.zmqpubrawtx=tcp://127.0.0.1:28333
"@ | Out-File "$($lnd.DataPath)\lnd.conf"

# Add to PATH
$env:Path += ";$($lnd.InstallPath)"
[Environment]::SetEnvironmentVariable("Path", $env:Path, [EnvironmentVariableTarget]::User)
