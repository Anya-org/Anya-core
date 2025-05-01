# Bitcoin Implementation Directory Structure Creation Script

# Create main directories
$mainDirs = @(
    "reorganized\bitcoin\core",
    "reorganized\bitcoin\layer2",
    "reorganized\bitcoin\protocol",
    "reorganized\bitcoin\testing",
    "reorganized\bitcoin\docs",
    "reorganized\bitcoin\ports",
    "reorganized\bitcoin\adapters",
    "reorganized\bitcoin\riscv",
    "reorganized\bitcoin\security"
)

foreach ($dir in $mainDirs) {
    if (-not (Test-Path $dir)) {
        New-Item -Path $dir -ItemType Directory -Force
        Write-Host "Created: $dir"
    } else {
        Write-Host "Already exists: $dir"
    }
}

# Create subdirectories
$subDirs = @{
    "reorganized\bitcoin\core" = @("consensus", "mempool", "network", "script")
    "reorganized\bitcoin\layer2" = @("framework", "bob", "lightning", "rgb", "rsk", "dlc", "taproot_assets")
    "reorganized\bitcoin\testing" = @("core", "layer2", "riscv", "integration")
    "reorganized\bitcoin\docs" = @("architecture", "standards", "layer2")
    "reorganized\bitcoin\adapters" = @("rpc", "storage", "protocols")
    "reorganized\bitcoin\riscv" = @("vm", "instructions", "contracts")
    "reorganized\bitcoin\ports" = @()
    "reorganized\bitcoin\security" = @("hsm", "crypto")
}

foreach ($parentDir in $subDirs.Keys) {
    foreach ($subDir in $subDirs[$parentDir]) {
        $path = Join-Path $parentDir $subDir
        if (-not (Test-Path $path)) {
            New-Item -Path $path -ItemType Directory -Force
            Write-Host "Created: $path"
        } else {
            Write-Host "Already exists: $path"
        }
    }
}

# Create port interface files
New-Item -Path "reorganized\bitcoin\ports\blockchain_port.rs" -ItemType File -Force
New-Item -Path "reorganized\bitcoin\ports\transaction_port.rs" -ItemType File -Force
New-Item -Path "reorganized\bitcoin\ports\layer2_port.rs" -ItemType File -Force

Write-Host "Directory structure created successfully." 