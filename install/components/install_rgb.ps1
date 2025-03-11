$ErrorActionPreference = "Stop"

$rgb = @{
    Version = "0.9.0"
    Url = "https://github.com/RGB-WG/rgb-node/releases/download/v0.9.0/rgb-node-v0.9.0-win64.zip"
    InstallPath = "$env:PROGRAMFILES\RGB"
    DataPath = "$env:APPDATA\RGB"
}

# Create directories
New-Item -ItemType Directory -Force -Path $rgb.InstallPath | Out-Null
New-Item -ItemType Directory -Force -Path $rgb.DataPath | Out-Null

# Download and extract RGB Node
$downloadPath = "$env:TEMP\rgb.zip"
Invoke-WebRequest -Uri $rgb.Url -OutFile $downloadPath
Expand-Archive -Path $downloadPath -DestinationPath $rgb.InstallPath -Force

# Create basic configuration
@"
{
    "network": "bitcoin",
    "stash_endpoint": "127.0.0.1:3000",
    "contract_endpoints": ["127.0.0.1:3000"],
    "verbose": 3,
    "datadir": "$($rgb.DataPath -replace '\\', '/')",
    "electrum": {
        "url": "127.0.0.1:50001"
    },
    "bitcoin": {
        "network": "mainnet",
        "rpc_endpoint": "127.0.0.1:8332",
        "rpc_user": "anyacore",
        "rpc_password": "$(New-Guid)"
    }
}
"@ | Out-File "$($rgb.DataPath)\config.json"

# Add to PATH
$env:Path += ";$($rgb.InstallPath)"
[Environment]::SetEnvironmentVariable("Path", $env:Path, [EnvironmentVariableTarget]::User)

# Create startup script
@"
`$env:RGB_NODE_DATA_DIR = '$($rgb.DataPath)'
Start-Process -FilePath '$($rgb.InstallPath)\rgb-node.exe' -ArgumentList 'start' -NoNewWindow
"@ | Out-File "$($rgb.InstallPath)\start-rgb.ps1"
