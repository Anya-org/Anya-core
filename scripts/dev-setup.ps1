# Development environment setup script
param (
    [switch]$SkipDependencies,
    [switch]$SkipTests
)

# Install core dependencies
if (-not $SkipDependencies) {
    Write-Host "Installing Rust dependencies..."
    rustup update stable
    rustup component add rustfmt clippy
    
    Write-Host "Installing Node.js dependencies..."
    npm install -g yarn
    
    Write-Host "Installing Python dependencies..."
    pip install -r requirements.txt
}

# Set up development directories
Write-Host "Setting up development structure..."
$directories = @(
    "src/core",
    "src/mobile",
    "src/bitcoin",
    "packages/dash33",
    "docs/api",
    "docs/research"
)

foreach ($dir in $directories) {
    New-Item -ItemType Directory -Force -Path $dir
}

# Initialize git hooks
Write-Host "Setting up git hooks..."
Copy-Item scripts/hooks/* .git/hooks/ -Force

# Run initial tests
if (-not $SkipTests) {
    Write-Host "Running initial tests..."
    cargo test --workspace
    cd packages/dash33 && yarn test && cd ../..
}

Write-Host "Development environment setup complete!"
