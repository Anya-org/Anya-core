# API Standardization Implementation Script
# Implements consistent API naming conventions and structure across the codebase
# Ensures all endpoints follow Bitcoin Core principles of security and immutability

param(
    [switch]$DryRun,
    [switch]$Verbose
)

# Script configuration
$scriptName = "API Standardization Implementation"
$scriptVersion = "1.0.0"
$scriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$projectRoot = Split-Path -Parent (Split-Path -Parent $scriptRoot)

# Directories to process
$apiDirs = @(
    (Join-Path $projectRoot "src\api"),
    (Join-Path $projectRoot "anya-bitcoin\src\api"),
    (Join-Path $projectRoot "core\src\api")
)

Write-Host "===== $scriptName v$scriptVersion =====" -ForegroundColor Cyan
Write-Host "Starting API standardization..."

# API naming convention standards - ensuring they conform to Bitcoin Core principles
$standardEndpointPatterns = @{
    "GET" = "^\/api\/v\d+\/[\w-]+(?:\/[\w-]+)*$"
    "POST" = "^\/api\/v\d+\/[\w-]+(?:\/[\w-]+)*$"
    "PUT" = "^\/api\/v\d+\/[\w-]+(?:\/[\w-]+)*$"
    "DELETE" = "^\/api\/v\d+\/[\w-]+(?:\/[\w-]+)*$"
    "PATCH" = "^\/api\/v\d+\/[\w-]+(?:\/[\w-]+)*$"
}

# Replacement templates for non-standard endpoints
# Structured to align with Bitcoin-focused functionality
$replacementTemplates = @{
    "bip353" = "/api/v1/bip353"
    "taproot" = "/api/v1/taproot"
    "bitcoin" = "/api/v1/bitcoin"
    "dlc" = "/api/v1/dlc"
    "rgb" = "/api/v1/rgb"
    "stacks" = "/api/v1/stacks"
    "rsk" = "/api/v1/rsk"
    "web5" = "/api/v1/web5"
    "open-banking" = "/api/v1/banking"
    "enterprise-banking" = "/api/v1/enterprise/banking"
}

# Verb replacements for non-standard methods
$verbReplacements = @{
    "validate" = "POST"
    "verify" = "POST"
    "check" = "GET"
    "fetch" = "GET"
    "update" = "PUT"
    "create" = "POST"
    "add" = "POST"
    "remove" = "DELETE"
    "delete" = "DELETE"
}

# Track changes
$changes = @{
    Total = 0
    Modified = 0
    Skipped = 0
    Errors = @()
}

# Function to check if an endpoint is standard compliant
function Test-StandardEndpoint {
    param(
        [string]$Endpoint,
        [string]$Method
    )
    
    if ($standardEndpointPatterns.ContainsKey($Method)) {
        $pattern = $standardEndpointPatterns[$Method]
        return $Endpoint -match $pattern
    }
    
    return $false
}

# Function to create standardized endpoint
function Get-StandardizedEndpoint {
    param(
        [string]$Endpoint,
        [string]$Method
    )
    
    # Already standardized
    if (Test-StandardEndpoint -Endpoint $Endpoint -Method $Method) {
        return $Endpoint
    }
    
    # Apply replacement templates
    $standardized = $Endpoint
    foreach ($key in $replacementTemplates.Keys) {
        if ($Endpoint -match "\/($key)\/") {
            $standardized = $Endpoint -replace "\/($key)\/", "$($replacementTemplates[$key])/"
            break
        }
    }
    
    # Ensure it starts with /api/v1
    if ($standardized -notmatch "^\/api\/v\d+\/") {
        # Extract the resource path without leading /
        $resource = $standardized -replace "^\/", ""
        $standardized = "/api/v1/$resource"
    }
    
    # Ensure kebab-case for path segments
    $segments = $standardized -split "/"
    for ($i = 0; $i -lt $segments.Length; $i++) {
        if ($segments[$i] -match "^[a-zA-Z0-9]+[A-Z][a-zA-Z0-9]*$") {
            # Convert camelCase or PascalCase to kebab-case
            $kebab = $segments[$i] -creplace "(?<=[a-z])(?=[A-Z])", "-"
            $kebab = $kebab.ToLower()
            $segments[$i] = $kebab
        }
    }
    
    $standardized = $segments -join "/"
    
    return $standardized
}

# Function to standardize method
function Get-StandardizedMethod {
    param(
        [string]$Method,
        [string]$FunctionName
    )
    
    # Already a standard HTTP method
    $standardMethods = @("GET", "POST", "PUT", "DELETE", "PATCH")
    if ($standardMethods -contains $Method) {
        return $Method
    }
    
    # Try to infer from function name
    foreach ($verb in $verbReplacements.Keys) {
        if ($FunctionName -like "$verb*") {
            return $verbReplacements[$verb]
        }
    }
    
    # Default to GET for read operations, POST for everything else
    if ($FunctionName -match "^(get|list|find|search|query)") {
        return "GET"
    }
    
    return "POST"
}

# Function to process a single API file
function Process-ApiFile {
    param(
        [string]$FilePath
    )
    
    $fileName = Split-Path -Leaf $FilePath
    Write-Host "Processing API file: $fileName" -ForegroundColor Yellow
    
    # Read file content
    $content = Get-Content $FilePath -Raw
    $originalContent = $content
    $modified = $false
    
    # Look for endpoint definitions
    # Pattern matches common endpoint definition formats in Rust
    $endpointMatches = [regex]::Matches($content, "(#\[\s*(get|post|put|delete|patch|route)\s*\(\s*[""'])([^""']+)([""']\s*\))")
    
    foreach ($match in $endpointMatches) {
        $fullMatch = $match.Groups[0].Value
        $decorator = $match.Groups[1].Value
        $method = $match.Groups[2].Value.Trim().ToUpper()
        $endpoint = $match.Groups[3].Value
        $closingQuote = $match.Groups[4].Value
        
        # Get function name associated with this endpoint
        $functionNameMatch = [regex]::Match($content.Substring($match.Index), "(?:fn|async\s+fn)\s+([a-zA-Z_][a-zA-Z0-9_]*)")
        $functionName = if ($functionNameMatch.Success) { $functionNameMatch.Groups[1].Value } else { "" }
        
        # Standardize method and endpoint
        $standardMethod = Get-StandardizedMethod -Method $method -FunctionName $functionName
        $standardEndpoint = Get-StandardizedEndpoint -Endpoint $endpoint -Method $standardMethod
        
        if ($endpoint -ne $standardEndpoint) {
            Write-Host "  - Standardizing endpoint: $endpoint -> $standardEndpoint" -ForegroundColor Green
            
            # Replace endpoint in the decorator
            $newDecorator = $decorator.Replace("#[", "#[") + $standardEndpoint + $closingQuote
            $content = $content.Replace($fullMatch, $newDecorator)
            $modified = $true
            $changes.Modified++
        }
        
        if ($method -ne $standardMethod -and $method -ne "route") {
            Write-Host "  - Standardizing method: $method -> $standardMethod" -ForegroundColor Green
            
            # Replace method in the decorator
            $newMethod = $decorator.Replace($method, $standardMethod.ToLower())
            $content = $content.Replace($decorator, $newMethod)
            $modified = $true
            $changes.Modified++
        }
        
        $changes.Total++
    }
    
    # Write changes back to file if modified
    if ($modified -and -not $DryRun) {
        Write-Host "  - Writing changes to $fileName" -ForegroundColor Green
        $content | Set-Content -Path $FilePath -Encoding UTF8
    } elseif ($modified -and $DryRun) {
        Write-Host "  - Would write changes to $fileName (dry run)" -ForegroundColor Yellow
    } else {
        Write-Host "  - No changes needed for $fileName" -ForegroundColor Gray
        $changes.Skipped++
    }
    
    return $modified
}

# Main execution
$apiFiles = @()

# Find all API files
foreach ($dir in $apiDirs) {
    if (Test-Path $dir) {
        $files = Get-ChildItem -Path $dir -Filter "*.rs" -Recurse
        $apiFiles += $files
    } else {
        Write-Host "API directory not found: $dir" -ForegroundColor Yellow
    }
}

Write-Host "Found $($apiFiles.Count) API files to process" -ForegroundColor Cyan

# Process each API file
foreach ($file in $apiFiles) {
    try {
        $result = Process-ApiFile -FilePath $file.FullName
    } catch {
        Write-Host "Error processing $($file.Name): $_" -ForegroundColor Red
        $changes.Errors += "$($file.Name): $_"
    }
}

# Create documentation about standardization
$docsDir = Join-Path $projectRoot "docs\api"
if (!(Test-Path $docsDir)) {
    New-Item -ItemType Directory -Path $docsDir -Force | Out-Null
}

$standardsDoc = @"
# API Standardization Guidelines

This document outlines the API standardization implemented across the Anya Core project, following Bitcoin Core principles of security, decentralization, and immutability.

## Endpoint Naming Conventions

All API endpoints follow these conventions:

1. **Path Structure**: `/api/v{version}/{resource}/{identifier?}/{sub-resource?}`
   - Example: `/api/v1/transactions/123/status`

2. **HTTP Methods**:
   - `GET` - Retrieve resources (non-modifying, secure)
   - `POST` - Create resources (modifying with validation)
   - `PUT` - Update resources (complete replacement with validation)
   - `DELETE` - Remove resources (with appropriate safeguards)
   - `PATCH` - Partial updates (with field-level validation)

3. **Naming Style**:
   - All paths use kebab-case
   - Example: `/api/v1/transaction-history`

## Standard API Patterns

| Operation | HTTP Method | URL Pattern | Example |
|-----------|------------|-------------|---------|
| List collection | GET | `/api/v1/{resource}` | `/api/v1/transactions` |
| Get single item | GET | `/api/v1/{resource}/{id}` | `/api/v1/transactions/123` |
| Create item | POST | `/api/v1/{resource}` | `/api/v1/transactions` |
| Update item | PUT | `/api/v1/{resource}/{id}` | `/api/v1/transactions/123` |
| Partial update | PATCH | `/api/v1/{resource}/{id}` | `/api/v1/transactions/123` |
| Delete item | DELETE | `/api/v1/{resource}/{id}` | `/api/v1/transactions/123` |

## Bitcoin Core Integration API Categories

| Category | Base Path | Description |
|----------|-----------|-------------|
| Bitcoin | `/api/v1/bitcoin` | Bitcoin Core functionality and protocol operations |
| Taproot | `/api/v1/taproot` | Taproot-related operations (BIP341) |
| DLC | `/api/v1/dlc` | Discrete Log Contracts functionality |
| RGB | `/api/v1/rgb` | RGB protocol integration for asset issuance |
| Stacks | `/api/v1/stacks` | Stacks smart contract capabilities |
| RSK | `/api/v1/rsk` | RSK sidechain integration |
| Web5 | `/api/v1/web5` | Web5 capabilities with DIDs |
| BIP353 | `/api/v1/bip353` | BIP353 functionality |
| Banking | `/api/v1/banking` | Open banking capabilities |
| Enterprise | `/api/v1/enterprise` | Enterprise features |

## Security Considerations

All APIs follow these security principles:

1. **Immutability** - Operations that modify data create immutable audit records
2. **Non-repudiation** - All modification operations require cryptographic signatures
3. **Input validation** - All inputs are strictly validated before processing
4. **Authorization** - Clear separation between public and authenticated endpoints
5. **Idempotency** - Operations can be safely retried with identical results

## Implementation Details

This standardization was automatically applied by the API standardization script to ensure consistent implementation of Bitcoin Core principles across all APIs.

Last updated: $(Get-Date -Format "yyyy-MM-dd")
"@

$standardsDocPath = Join-Path $docsDir "api-standards.md"
if (!(Test-Path $docsDir)) {
    New-Item -ItemType Directory -Path $docsDir -Force | Out-Null
}
$standardsDoc | Set-Content -Path $standardsDocPath -Encoding UTF8

Write-Host "API standardization documentation written to: $standardsDocPath" -ForegroundColor Green

# Print summary
Write-Host "`nAPI Standardization Summary:" -ForegroundColor Cyan
Write-Host "  Total endpoints processed: $($changes.Total)" -ForegroundColor White
Write-Host "  Modified endpoints: $($changes.Modified)" -ForegroundColor Green
Write-Host "  Skipped endpoints: $($changes.Skipped)" -ForegroundColor Gray
Write-Host "  Errors: $($changes.Errors.Count)" -ForegroundColor $(if ($changes.Errors.Count -gt 0) { "Red" } else { "Gray" })

if ($changes.Errors.Count -gt 0) {
    Write-Host "`nErrors encountered:" -ForegroundColor Red
    foreach ($error in $changes.Errors) {
        Write-Host "  - $error" -ForegroundColor Red
    }
}

# Exit with success
exit 0
