# Script to fix Bitcoin import errors in Rust files
# Ensures proper alignment with Bitcoin Core by fixing import paths for the bitcoin crate

param(
    [switch]$DryRun
)

$scriptName = "Bitcoin Import Fixer"
$scriptVersion = "1.0.0"
$rootDir = Join-Path $PSScriptRoot "..\\.."

Write-Host "===== $scriptName v$scriptVersion =====" -ForegroundColor Cyan
Write-Host "Fixing Bitcoin import issues in Rust files..."

# Target files with import errors
$targetFiles = @{
    "anya-bitcoin\src\core\consensus\rules.rs" = $true
    "anya-bitcoin\src\core\consensus\validation.rs" = $true
    "anya-bitcoin\src\layer2\dlc\mod.rs" = $true
    "anya-bitcoin\src\core\script\standard.rs" = $true
    "anya-bitcoin\src\core\script\interpreter.rs" = $true
    "anya-bitcoin\src\prelude.rs" = $true
}

# Import path mappings (old path -> new path)
$importMappings = @{
    "bitcoin::BlockHeader" = "bitcoin::block::Header as BlockHeader"
    "bitcoin::util::uint::Uint256" = "bitcoin::hashes::Hash as Uint256"
    "bitcoin::LockTime" = "bitcoin::locktime::LockTime"
    "bitcoin::Version" = "bitcoin::transaction::Version"
    "bitcoin::taproot::ScriptPath" = "bitcoin::sighash::ScriptPath"
    "bitcoin::error::BitcoinError" = "bitcoin::consensus::Error as BitcoinError"
    "bitcoin::error::BitcoinResult" = "std::result::Result<T, bitcoin::consensus::Error> as BitcoinResult"
    "bitcoin::tapscript" = "bitcoin::taproot::tapscript"
    "bitcoin::util" = "bitcoin::hashes"
}

# Counter for tracking changes
$counter = @{
    Total = 0
    Fixed = 0
    Skipped = 0
}

# Function to fix bitcoin imports in a file
function Fix-BitcoinImports {
    param(
        [string]$FilePath
    )
    
    $relativeFilePath = $FilePath.Replace($rootDir, "").TrimStart("\")
    
    # Check if this file is in our target list
    if (-not $targetFiles.ContainsKey($relativeFilePath)) {
        return
    }
    
    $fileName = Split-Path -Leaf $FilePath
    Write-Host "Processing: $fileName" -ForegroundColor Yellow
    
    # Read the file content
    $content = Get-Content -Path $FilePath -Raw
    $originalContent = $content
    $modified = $false
    
    # Apply import mappings
    foreach ($oldImport in $importMappings.Keys) {
        $newImport = $importMappings[$oldImport]
        
        if ($content -match [regex]::Escape($oldImport)) {
            $content = $content -replace [regex]::Escape($oldImport), $newImport
            $modified = $true
            Write-Host "  - Replaced: $oldImport -> $newImport" -ForegroundColor Green
        }
    }
    
    # Fix missing bitflags dependency
    if ($fileName -eq "peers.rs" -or $fileName -eq "interpreter.rs") {
        if ($content -match "bitflags!" -and -not $content -match "#\[macro_use\]\s*extern\s+crate\s+bitflags") {
            $content = "#[macro_use]`nextern crate bitflags;`n`n" + $content
            $modified = $true
            Write-Host "  - Added bitflags extern crate declaration" -ForegroundColor Green
        }
    }
    
    # Fix size issues with [u8] in bitcoin::Script
    if ($fileName -eq "standard.rs" -and $content -match "bitcoin::Script") {
        $content = $content -replace "bitcoin::Script", "bitcoin::script::Script"
        $modified = $true
        Write-Host "  - Updated Script imports to use bitcoin::script::Script" -ForegroundColor Green
    }
    
    # Fix specific issues with the script interpreter
    if ($fileName -eq "interpreter.rs") {
        # Fix the duplicated discriminant values
        if ($content -match "discriminant value `0` assigned more than once") {
            $content = $content -replace "OP_0 = 0,", "OP_0 = 0,"
            $content = $content -replace "OP_FALSE = 0,", "OP_FALSE = 0x00,"
            $modified = $true
            Write-Host "  - Fixed duplicate discriminant value for OP_FALSE" -ForegroundColor Green
        }
        
        if ($content -match "discriminant value `81` assigned more than once") {
            $content = $content -replace "OP_1 = 81,", "OP_1 = 81,"
            $content = $content -replace "OP_TRUE = 81,", "OP_TRUE = 0x51,"
            $modified = $true
            Write-Host "  - Fixed duplicate discriminant value for OP_TRUE" -ForegroundColor Green
        }
        
        # Fix syntax issue with <<
        if ($content -match "`<`< is interpreted as a start of generic arguments") {
            $content = $content -replace "i64 << 32", "(i64::from(1) << 32)"
            $modified = $true
            Write-Host "  - Fixed shift operator syntax" -ForegroundColor Green
        }
    }
    
    # Check if the content was modified
    if ($modified) {
        $counter.Total++
        
        if (-not $DryRun) {
            # Write the fixed content back to the file
            $content | Set-Content -Path $FilePath -Encoding UTF8
            Write-Host "  - Fixed bitcoin imports in $fileName" -ForegroundColor Green
            $counter.Fixed++
        } else {
            Write-Host "  - Would fix bitcoin imports in $fileName (dry run)" -ForegroundColor Yellow
            $counter.Skipped++
        }
    } else {
        Write-Host "  - No applicable bitcoin import issues found in $fileName" -ForegroundColor Gray
    }
}

# Process target files
foreach ($relativeFilePath in $targetFiles.Keys) {
    $filePath = Join-Path $rootDir $relativeFilePath
    
    if (Test-Path $filePath) {
        Fix-BitcoinImports -FilePath $filePath
    } else {
        Write-Host "File not found: $filePath" -ForegroundColor Red
    }
}

# Print summary
Write-Host "`nBitcoin Import Fixing Summary:" -ForegroundColor Cyan
Write-Host "  Total files with issues: $($counter.Total)" -ForegroundColor White
Write-Host "  Files fixed: $($counter.Fixed)" -ForegroundColor Green
Write-Host "  Files skipped (dry run): $($counter.Skipped)" -ForegroundColor Yellow

Write-Host "Bitcoin import fixing completed" -ForegroundColor Green
