# PowerShell script to update symbolic links and system map
param (
    [string]$rootDir = (Get-Location)
)

# Function to create symbolic links
function Create-SymLink {
    param (
        [string]$source,
        [string]$target
    )
    
    if (Test-Path $target) {
        Write-Host "Removing existing link: $target"
        Remove-Item $target -Force -Recurse
    }
    
    Write-Host "Creating symbolic link: $source -> $target"
    New-Item -ItemType Junction -Path $target -Target $source -Force
}

# Function to update last modified date in markdown files
function Update-MarkdownDate {
    param (
        [string]$markdownFile
    )
    
    if (Test-Path $markdownFile) {
        $content = Get-Content $markdownFile -Raw
        $newDate = Get-Date -Format "yyyy-MM-dd"
        $content = $content -replace "Last updated: \d{4}-\d{2}-\d{2}", "Last updated: $newDate"
        Set-Content -Path $markdownFile -Value $content
        Write-Host "Updated timestamp in: $markdownFile"
    }
}

# Function to verify index links
function Verify-IndexLinks {
    param (
        [string]$indexFile
    )
    
    if (Test-Path $indexFile) {
        $content = Get-Content $indexFile -Raw
        $pattern = '\[([^\]]+)\]\(([^)]+)\)'
        $matches = [regex]::Matches($content, $pattern)
        $brokenLinks = @()
        $newLinks = @()
        
        # Cross-reference with system index
        $indexData = Get-Content "$anyaDir/system_index.json" -Raw | ConvertFrom-Json
        $componentHashes = @{}
        $indexData.component_paths.PSObject.Properties | ForEach-Object {
            $componentHashes[$_.Name] = $_.Value.hash
        }
        
        foreach ($match in $matches) {
            $linkText = $match.Groups[1].Value
            $linkPath = $match.Groups[2].Value
            
            if (-not $linkPath.StartsWith("http")) {
                $fullPath = Join-Path (Split-Path $indexFile) $linkPath.TrimStart("./")
                
                # Check physical path
                $pathExists = Test-Path $fullPath
                
                # Verify against system index
                $inIndex = $indexData.component_paths.ContainsValue($fullPath) -or 
                          $indexData.model_paths.ContainsValue($fullPath)
                
                if (-not $pathExists -or -not $inIndex) {
                    $brokenLinks += "$linkText -> $linkPath"
                }
                
                # Record new links for index update
                if ($pathExists -and -not $inIndex) {
                    $newLinks += @{
                        Path = $fullPath
                        Type = if ($fullPath -match "\.md$") { "Documentation" } else { "Component" }
                    }
                }
            }
        }
        
        # Verify content hashes
        foreach ($link in $newLinks) {
            $currentHash = (Get-FileHash $link.Path -Algorithm SHA256).Hash
            if ($componentHashes[$link.Path] -ne $currentHash) {
                Write-Warning "Content drift detected in $($link.Path)"
                $brokenLinks += "$($link.Path) [hash mismatch]"
            }
        }
        
        # Update system index with new links
        if ($newLinks.Count -gt 0) {
            $indexManager = [SystemIndexManager]::global()
            $newIndex = $indexManager.read_index().Result
            
            foreach ($link in $newLinks) {
                if (-not $newIndex.component_paths.ContainsKey($link.Path)) {
                    $newIndex.component_paths[$link.Path] = $link.Type
                }
            }
            
            $indexManager.update_index($newIndex).Wait()
        }
        
        # Enhanced Rust validation
        if ($link.Path -match "\.rs$") {
            $rustMetrics = $indexData.rust_metrics[$link.Path]
            
            if ($rustMetrics.cyclomatic_complexity -gt 25) {
                Write-Warning "High complexity in $($link.Path) ($($rustMetrics.cyclomatic_complexity))"
            }
            
            if ($rustMetrics.unsafe_usage_count -gt 0) {
                Write-Warning "Unsafe code detected in $($link.Path) ($($rustMetrics.unsafe_usage_count) instances)"
            }
            
            if ($rustMetrics.bitcoin_protocol_adherence -lt 0.9) {
                Write-Warning "Low Bitcoin protocol adherence in $($link.Path) ($($rustMetrics.bitcoin_protocol_adherence))"
            }
        }
        
        if ($brokenLinks.Count -gt 0) {
            Write-Warning "Broken links found in $(Split-Path $indexFile -Leaf)"
            $brokenLinks | ForEach-Object { Write-Warning "  $_" }
            return $false
        }
    }
    return $true
}

# Main execution
Write-Host "Updating Anya system links and documentation..."

# Ensure anya directory exists
$anyaDir = Join-Path -Path $rootDir -ChildPath "anya"
if (-not (Test-Path $anyaDir)) {
    New-Item -ItemType Directory -Path $anyaDir -Force
}

# Create symbolic links for main components
$components = @{
    "dash33" = "dash33"
    "enterprise" = "enterprise"
    "mobile" = "mobile"
}

foreach ($comp in $components.GetEnumerator()) {
    $source = Join-Path -Path $rootDir -ChildPath $comp.Key
    $target = Join-Path -Path $anyaDir -ChildPath $comp.Value
    
    if (Test-Path $source) {
        Create-SymLink -source $source -target $target
    } else {
        Write-Warning "Source directory not found: $source"
    }
}

# Update timestamps in index and system map files
$indexFiles = @(
    (Join-Path $rootDir "INDEX.md"),
    (Join-Path $anyaDir "INDEX.md"),
    (Join-Path $rootDir "dash33/INDEX.md"),
    (Join-Path $rootDir "enterprise/INDEX.md"),
    (Join-Path $rootDir "mobile/INDEX.md")
)

$systemMapPath = Join-Path -Path $anyaDir -ChildPath "SYSTEM_MAP.md"
$indexFiles += $systemMapPath

foreach ($file in $indexFiles) {
    Update-MarkdownDate -markdownFile $file
    Verify-IndexLinks -indexFile $file
}

Write-Host "System links and documentation updated successfully"
