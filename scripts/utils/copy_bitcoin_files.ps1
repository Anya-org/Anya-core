# Bitcoin Implementation File Copying Script
# This script copies files from their original locations to the reorganized structure

# Set error action preference
$ErrorActionPreference = "Continue"

# Create a function to log messages
function Log-Message {
    param(
        [Parameter(Mandatory=$true)]
        [string]$Message,
        
        [Parameter(Mandatory=$false)]
        [string]$Type = "INFO"
    )
    
    Write-Host "[$Type] $Message"
}

# Create a function to safely copy files
function Safe-Copy {
    param(
        [Parameter(Mandatory=$true)]
        [string]$Source,
        
        [Parameter(Mandatory=$true)]
        [string]$Destination
    )
    
    if (Test-Path $Source) {
        try {
            $destDir = Split-Path -Parent $Destination
            if (-not (Test-Path $destDir)) {
                New-Item -Path $destDir -ItemType Directory -Force | Out-Null
            }
            
            Copy-Item -Path $Source -Destination $Destination -Force
            Log-Message "Copied: $Source -> $Destination"
            return $true
        }
        catch {
            Log-Message "Failed to copy: $Source -> $Destination. Error: $_" -Type "ERROR"
            return $false
        }
    }
    else {
        Log-Message "Source file not found: $Source" -Type "WARNING"
        return $false
    }
}

# Create a function to safely copy directories
function Safe-Copy-Directory {
    param(
        [Parameter(Mandatory=$true)]
        [string]$Source,
        
        [Parameter(Mandatory=$true)]
        [string]$Destination
    )
    
    if (Test-Path $Source) {
        try {
            if (-not (Test-Path $Destination)) {
                New-Item -Path $Destination -ItemType Directory -Force | Out-Null
            }
            
            Copy-Item -Path "$Source\*" -Destination $Destination -Recurse -Force
            Log-Message "Copied directory: $Source -> $Destination"
            return $true
        }
        catch {
            Log-Message "Failed to copy directory: $Source -> $Destination. Error: $_" -Type "ERROR"
            return $false
        }
    }
    else {
        Log-Message "Source directory not found: $Source" -Type "WARNING"
        return $false
    }
}

Log-Message "Starting Bitcoin implementation file copying process..."

# Copy Core Bitcoin files
Log-Message "Copying Core Bitcoin files..." -Type "SECTION"

# Core consensus files
Safe-Copy-Directory "anya-bitcoin\src\core\consensus" "reorganized\bitcoin\core\consensus"
Safe-Copy "src\bitcoin\bip340.rs" "reorganized\bitcoin\core\consensus\bip340.rs"
Safe-Copy "src\bitcoin\bip341.rs" "reorganized\bitcoin\core\consensus\bip341.rs"
Safe-Copy "src\bitcoin\validation.rs" "reorganized\bitcoin\core\consensus\validation.rs"
Safe-Copy "src\bitcoin\merkle.rs" "reorganized\bitcoin\core\consensus\merkle.rs"

# Core mempool files
Safe-Copy-Directory "anya-bitcoin\src\core\mempool" "reorganized\bitcoin\core\mempool"

# Core network files
Safe-Copy-Directory "anya-bitcoin\src\core\network" "reorganized\bitcoin\core\network"

# Core script files
Safe-Copy-Directory "anya-bitcoin\src\core\script" "reorganized\bitcoin\core\script"
Safe-Copy "src\bitcoin\taproot.rs" "reorganized\bitcoin\core\script\taproot.rs"

# Copy Layer 2 files
Log-Message "Copying Layer 2 files..." -Type "SECTION"

# Framework files
Safe-Copy-Directory "anya-bitcoin\src\layer2\framework" "reorganized\bitcoin\layer2\framework"

# BOB files
Safe-Copy-Directory "src\layer2\bob" "reorganized\bitcoin\layer2\bob"

# Lightning files
Safe-Copy-Directory "src\layer2\lightning" "reorganized\bitcoin\layer2\lightning"
Safe-Copy "src\bitcoin\lightning.rs" "reorganized\bitcoin\layer2\lightning\bitcoin_lightning.rs"

# RGB files
Safe-Copy-Directory "src\layer2\rgb" "reorganized\bitcoin\layer2\rgb"
if (Test-Path "src\bitcoin\layer2\rgb") {
    Safe-Copy-Directory "src\bitcoin\layer2\rgb" "reorganized\bitcoin\layer2\rgb"
}

# RSK files
Safe-Copy-Directory "src\layer2\rsk" "reorganized\bitcoin\layer2\rsk"

# DLC files
if (Test-Path "src\bitcoin\dlc") {
    Safe-Copy-Directory "src\bitcoin\dlc" "reorganized\bitcoin\layer2\dlc"
}

# Taproot Assets files
if (Test-Path "src\layer2\taproot_assets") {
    Safe-Copy-Directory "src\layer2\taproot_assets" "reorganized\bitcoin\layer2\taproot_assets"
}

# Copy Protocol files
Log-Message "Copying Protocol files..." -Type "SECTION"
Safe-Copy "src\bitcoin\protocol.rs" "reorganized\bitcoin\protocol\core_protocol.rs"
if (Test-Path "anya-bitcoin\src\protocol") {
    Safe-Copy-Directory "anya-bitcoin\src\protocol" "reorganized\bitcoin\protocol"
}

# Copy Testing files
Log-Message "Copying Testing files..." -Type "SECTION"

# Core tests
if (Test-Path "src\bitcoin\tests") {
    Safe-Copy-Directory "src\bitcoin\tests" "reorganized\bitcoin\testing\core"
}

# Layer 2 tests
Safe-Copy "tests\bitcoin\validation_test.rs" "reorganized\bitcoin\testing\layer2\validation_test.rs"

# RISC-V tests
Safe-Copy "tests\bitcoin\riscv_tests.rs" "reorganized\bitcoin\testing\riscv\riscv_tests.rs"
Safe-Copy "tests\bitcoin\riscv_vm_tests.rs" "reorganized\bitcoin\testing\riscv\riscv_vm_tests.rs"

# Integration tests
Safe-Copy "tests\bitcoin\cross_layer_tests.rs" "reorganized\bitcoin\testing\integration\cross_layer_tests.rs"
Safe-Copy "tests\bitcoin\layer3_tests.rs" "reorganized\bitcoin\testing\integration\layer3_tests.rs"
Safe-Copy "tests\bitcoin\vm_layer_tests.rs" "reorganized\bitcoin\testing\integration\vm_layer_tests.rs"

# Copy Documentation files
Log-Message "Copying Documentation files..." -Type "SECTION"

# Architecture documentation
Safe-Copy "docs\HEXAGONAL.md" "reorganized\bitcoin\docs\architecture\HEXAGONAL.md"

# Layer 2 documentation
Safe-Copy "docs\bitcoin\LAYER2_SUPPORT.md" "reorganized\bitcoin\docs\layer2\OVERVIEW.md"

# Copy any remaining Bitcoin documentation
if (Test-Path "docs\bitcoin") {
    Safe-Copy-Directory "docs\bitcoin" "reorganized\bitcoin\docs"
}

# Generate Port Interfaces
Log-Message "Generating Port Interfaces..." -Type "SECTION"

$blockchainPortContent = @"
// Generated blockchain_port.rs
// Hexagonal Architecture - Primary Port

use async_trait::async_trait;
use crate::core::consensus::{Block, BlockHash, Transaction, Proof};
use crate::error::Result;

#[async_trait]
pub trait BlockchainPort {
    async fn broadcast_transaction(&self, tx: &Transaction) -> Result<BroadcastResult>;
    async fn get_block(&self, hash: BlockHash) -> Result<Block>;
    async fn verify_proof(&self, proof: &Proof) -> Result<VerificationResult>;
}

pub struct BroadcastResult {
    pub transaction_id: String,
    pub status: BroadcastStatus,
}

pub enum BroadcastStatus {
    Accepted,
    Rejected(String),
    Pending,
}

pub struct VerificationResult {
    pub is_valid: bool,
    pub reason: Option<String>,
}
"@

$transactionPortContent = @"
// Generated transaction_port.rs
// Hexagonal Architecture - Primary Port

use async_trait::async_trait;
use crate::core::consensus::{Transaction, TransactionId, ValidationResult};
use crate::error::Result;

#[async_trait]
pub trait TransactionPort {
    async fn submit_transaction(&self, tx: Transaction) -> Result<TransactionId>;
    async fn get_transaction(&self, id: TransactionId) -> Result<Transaction>;
    async fn validate_transaction(&self, tx: &Transaction) -> Result<ValidationResult>;
}
"@

$layer2PortContent = @"
// Generated layer2_port.rs
// Hexagonal Architecture - Secondary Port

use async_trait::async_trait;
use crate::error::Result;

#[async_trait]
pub trait ProtocolPort {
    async fn submit_protocol_tx(&self, tx: ProtocolTransaction) -> Result<TransactionId>;
    async fn verify_protocol_state(&self, state: &ProtocolState) -> Result<VerificationResult>;
    async fn sync_protocol_state(&self) -> Result<SyncResult>;
}

#[async_trait]
pub trait AssetPort {
    async fn issue_asset(&self, params: AssetParams) -> Result<AssetId>;
    async fn transfer_asset(&self, transfer: AssetTransfer) -> Result<TransferResult>;
    async fn get_asset_state(&self, asset_id: AssetId) -> Result<AssetState>;
}

// Protocol-specific types
pub struct ProtocolTransaction;
pub struct ProtocolState;
pub struct TransactionId;
pub struct VerificationResult;
pub struct SyncResult;
pub struct AssetParams;
pub struct AssetId;
pub struct AssetTransfer;
pub struct TransferResult;
pub struct AssetState;
"@

Set-Content -Path "reorganized\bitcoin\ports\blockchain_port.rs" -Value $blockchainPortContent
Set-Content -Path "reorganized\bitcoin\ports\transaction_port.rs" -Value $transactionPortContent
Set-Content -Path "reorganized\bitcoin\ports\layer2_port.rs" -Value $layer2PortContent

Log-Message "Port interfaces generated."

# Create main library entry point
$libRsContent = @"
//! Bitcoin Core Implementation
//! Following hexagonal architecture principles and official Bitcoin Improvement Proposals (BIPs)

use std::{sync::Arc, path::PathBuf};

// Re-export core modules
pub mod core;
pub mod layer2;
pub mod ports;
pub mod adapters;
pub mod protocol;
pub mod riscv;
pub mod security;

// Configuration
#[derive(Debug, Clone)]
pub struct Config {
    pub network: bitcoin::Network,
    pub datadir: PathBuf,
    pub max_peers: u32,      // Default: 125
    pub min_peers: u32,      // Default: 8
}

impl Default for Config {
    fn default() -> Self {
        Self {
            network: bitcoin::Network::Bitcoin,
            datadir: PathBuf::from("~/.bitcoin"),
            max_peers: 125,
            min_peers: 8,
        }
    }
}

// Main node implementation
pub struct BitcoinNode {
    config: Config,
    consensus: core::consensus::Validator,
    mempool: core::mempool::Mempool,
    network: core::network::P2P,
    /// Layer 2 protocol registry
    layer2_registry: Option<Arc<layer2::framework::Layer2Registry>>,
}

impl BitcoinNode {
    pub fn new(config: Config) -> Result<Self, bitcoin::Error> {
        Ok(Self {
            consensus: core::consensus::Validator::new(&config)?,
            mempool: core::mempool::Mempool::new(&config)?,
            network: core::network::P2P::new(&config)?,
            config,
            layer2_registry: None,
        })
    }

    pub fn start(&mut self) -> Result<(), bitcoin::Error> {
        // Initialize Layer 2 factory and registry
        let factory = Arc::new(layer2::framework::Layer2Factory::new());
        let registry = Arc::new(layer2::framework::Layer2Registry::new(factory));
        self.layer2_registry = Some(registry);
        
        Ok(())
    }
    
    /// Get Layer 2 protocol registry
    pub fn layer2_registry(&self) -> Option<Arc<layer2::framework::Layer2Registry>> {
        self.layer2_registry.clone()
    }
}
"@

Set-Content -Path "reorganized\bitcoin\lib.rs" -Value $libRsContent
Log-Message "Main library entry point created: lib.rs"

# Generate baseline impl files for core modules
$coreModRs = @"
//! Core Bitcoin implementation

pub mod consensus;
pub mod mempool;
pub mod network;
pub mod script;
"@

Set-Content -Path "reorganized\bitcoin\core\mod.rs" -Value $coreModRs
Log-Message "Core module file created: core/mod.rs"

$layer2ModRs = @"
//! Layer 2 implementations

pub mod framework;
pub mod bob;
pub mod lightning;
pub mod rgb;
pub mod rsk;
pub mod dlc;
pub mod taproot_assets;
"@

Set-Content -Path "reorganized\bitcoin\layer2\mod.rs" -Value $layer2ModRs
Log-Message "Layer 2 module file created: layer2/mod.rs"

# Check for missing directories and validate the structure
Log-Message "Validation step: Checking for missing directories..." -Type "SECTION"

$requiredDirs = @(
    "reorganized\bitcoin\core\consensus",
    "reorganized\bitcoin\core\mempool",
    "reorganized\bitcoin\core\network",
    "reorganized\bitcoin\core\script",
    "reorganized\bitcoin\layer2\framework",
    "reorganized\bitcoin\layer2\bob",
    "reorganized\bitcoin\layer2\lightning",
    "reorganized\bitcoin\layer2\rgb",
    "reorganized\bitcoin\layer2\rsk",
    "reorganized\bitcoin\layer2\dlc",
    "reorganized\bitcoin\layer2\taproot_assets",
    "reorganized\bitcoin\protocol",
    "reorganized\bitcoin\testing\core",
    "reorganized\bitcoin\testing\layer2",
    "reorganized\bitcoin\testing\riscv",
    "reorganized\bitcoin\testing\integration",
    "reorganized\bitcoin\docs\architecture",
    "reorganized\bitcoin\docs\standards",
    "reorganized\bitcoin\docs\layer2",
    "reorganized\bitcoin\ports",
    "reorganized\bitcoin\adapters\rpc",
    "reorganized\bitcoin\adapters\storage",
    "reorganized\bitcoin\adapters\protocols",
    "reorganized\bitcoin\riscv\vm",
    "reorganized\bitcoin\riscv\instructions",
    "reorganized\bitcoin\riscv\contracts",
    "reorganized\bitcoin\security\hsm",
    "reorganized\bitcoin\security\crypto"
)

$missingDirs = @()
foreach ($dir in $requiredDirs) {
    if (-not (Test-Path $dir)) {
        $missingDirs += $dir
        Log-Message "Missing directory: $dir" -Type "WARNING"
    }
}

if ($missingDirs.Count -gt 0) {
    Log-Message "Found $($missingDirs.Count) missing directories. Creating them now..." -Type "WARNING"
    foreach ($dir in $missingDirs) {
        New-Item -Path $dir -ItemType Directory -Force | Out-Null
        Log-Message "Created missing directory: $dir"
    }
}
else {
    Log-Message "All required directories are present."
}

Log-Message "Bitcoin implementation file copying complete!" -Type "SUCCESS" 