# [AIR-3][AIS-3][BPC-3] AI Label Validation Script
# This script validates that AI labels in the codebase follow the standardized format

param (
    [string]$file = "",
    [switch]$verbose = $false,
    [switch]$fix = $false,
    [switch]$stats = $false
)

# Set script version
$VERSION = "1.0.0"
$CANONICAL_DOC = "docs/standards/AI_LABELING.md"

Write-Host "AI Label Validation Tool v$VERSION" -ForegroundColor Green
Write-Host "Based on canonical documentation: $CANONICAL_DOC" -ForegroundColor Green
Write-Host ""

# Valid labels and their levels
$VALID_CATEGORIES = @(
    # Core categories
    "AIR", "AIS", "AIT", "AIM", "AIP", "AIE",
    # Extended categories
    "BPC", "RES", "SCL", "PFM", "DAO", "DID", "W5C", "UXA"
)

$MAX_LEVEL = 3  # 0-3 scale

# File extensions to check
$CODE_EXTENSIONS = @(
    ".rs", ".js", ".ts", ".jsx", ".tsx", ".py", ".go", 
    ".java", ".c", ".cpp", ".h", ".hpp", ".cs"
)

$DOC_EXTENSIONS = @(
    ".md", ".rst", ".txt", ".adoc"
)

function Test-ValidLabel {
    param (
        [string]$label
    )
    
    # Extract category and level using regex
    if ($label -match '\[([A-Z]{2,5})-([0-9])\]') {
        $category = $matches[1]
        $level = [int]$matches[2]
        
        # Check if category is valid
        if ($VALID_CATEGORIES -contains $category) {
            # Check if level is valid (0-3)
            if ($level -ge 0 -and $level -le $MAX_LEVEL) {
                return $true
            }
            else {
                return "Invalid level: $level (must be 0-$MAX_LEVEL)"
            }
        }
        else {
            return "Invalid category: $category"
        }
    }
    else {
        return "Invalid label format: $label (must be [XXX-N])"
    }
}

function Get-LabelsFromContent {
    param (
        [string]$content
    )
    
    $labels = @()
    $labelPattern = '\[([A-Z]{2,5})-([0-9])\]'
    $matches = [regex]::Matches($content, $labelPattern)
    
    foreach ($match in $matches) {
        $labels += $match.Value
    }
    
    return $labels
}

function Convert-LegacyToStandardLabel {
    param (
        [string]$legacyLabel
    )
    
    # Convert sequence format AIR-001 to [AIR-1]
    if ($legacyLabel -match '([A-Z]{2,5})-(\d{3})') {
        $category = $matches[1]
        $number = [int]$matches[2]
        
        # Map legacy sequence numbers to new scale
        $level = 1  # Default to level 1
        if ($number -le 2) { $level = 1 }
        elseif ($number -le 4) { $level = 2 }
        else { $level = 3 }
        
        return "[$category-$level]"
    }
    
    # Convert 1-5 scale to 0-3 scale
    if ($legacyLabel -match '\[([A-Z]{2,5})-([1-5])\]') {
        $category = $matches[1]
        $oldLevel = [int]$matches[2]
        
        # Map old 1-5 scale to new 0-3 scale
        $newLevel = 0
        switch ($oldLevel) {
            1 { $newLevel = 0 }
            2 { $newLevel = 1 }
            3 { $newLevel = 1 }
            4 { $newLevel = 2 }
            5 { $newLevel = 3 }
        }
        
        return "[$category-$newLevel]"
    }
    
    # Already in standard format
    return $legacyLabel
}

function Update-FileWithStandardLabels {
    param (
        [string]$filePath
    )
    
    try {
        $content = Get-Content -Path $filePath -Raw
        $originalContent = $content
        
        # Find all labels using regex
        $labelPattern = '\[([A-Z]{2,5})-([0-9]{1,3})\]'
        $matches = [regex]::Matches($content, $labelPattern)
        
        $replacements = @{}
        
        foreach ($match in $matches) {
            $legacyLabel = $match.Value
            $standardLabel = Convert-LegacyToStandardLabel -legacyLabel $legacyLabel
            
            if ($legacyLabel -ne $standardLabel) {
                $replacements[$legacyLabel] = $standardLabel
            }
        }
        
        # Apply replacements
        foreach ($key in $replacements.Keys) {
            $content = $content -replace [regex]::Escape($key), $replacements[$key]
        }
        
        # Write updated content if changed
        if ($content -ne $originalContent) {
            Set-Content -Path $filePath -Value $content
            return $replacements.Count
        }
        
        return 0
    }
    catch {
        Write-Error "Error updating file $filePath`: $_"
        return 0
    }
}

function Get-FileStats {
    param (
        [string]$filePath
    )
    
    try {
        $content = Get-Content -Path $filePath -Raw
        $labels = Get-LabelsFromContent -content $content
        
        $stats = @{}
        foreach ($label in $labels) {
            if ($stats.ContainsKey($label)) {
                $stats[$label]++
            }
            else {
                $stats[$label] = 1
            }
        }
        
        return $stats
    }
    catch {
        Write-Error "Error getting stats for file $filePath`: $_"
        return @{}
    }
}

function Test-File {
    param (
        [string]$filePath
    )
    
    try {
        $content = Get-Content -Path $filePath -Raw
        $labels = Get-LabelsFromContent -content $content
        
        $fileExt = [System.IO.Path]::GetExtension($filePath)
        $isCode = $CODE_EXTENSIONS -contains $fileExt
        $isDoc = $DOC_EXTENSIONS -contains $fileExt
        
        $errors = @()
        $validCount = 0
        
        foreach ($label in $labels) {
            $result = Test-ValidLabel -label $label
            if ($result -eq $true) {
                $validCount++
            }
            else {
                $errors += "Invalid label in $filePath`: $label - $result"
            }
        }
        
        if ($errors.Count -eq 0) {
            if ($labels.Count -gt 0) {
                if ($verbose) {
                    Write-Host "✓ $filePath`: $validCount valid labels" -ForegroundColor Green
                }
            }
        }
        else {
            foreach ($error in $errors) {
                Write-Host "✗ $error" -ForegroundColor Red
            }
        }
        
        # Fix labels if requested
        if ($fix -and $errors.Count -gt 0) {
            $fixedCount = Update-FileWithStandardLabels -filePath $filePath
            if ($fixedCount -gt 0) {
                Write-Host "  Fixed $fixedCount labels in $filePath" -ForegroundColor Yellow
            }
        }
        
        # Return stats if requested
        if ($stats) {
            return Get-FileStats -filePath $filePath
        }
        
        return $errors.Count -eq 0
    }
    catch {
        Write-Error "Error processing file $filePath`: $_"
        return $false
    }
}

# Main execution
$allStats = @{}
$processedFiles = 0
$errorFiles = 0

# Process single file if specified
if ($file -ne "") {
    if (Test-Path $file) {
        $result = Test-File -filePath $file
        
        if ($stats) {
            $fileStats = Get-FileStats -filePath $file
            foreach ($key in $fileStats.Keys) {
                if ($allStats.ContainsKey($key)) {
                    $allStats[$key] += $fileStats[$key]
                }
                else {
                    $allStats[$key] = $fileStats[$key]
                }
            }
        }
        
        $processedFiles = 1
        $errorFiles = if ($result) { 0 } else { 1 }
    }
    else {
        Write-Error "File not found: $file"
        exit 1
    }
}
else {
    # Process all files in the repository
    $allFiles = Get-ChildItem -Path . -Recurse -File | Where-Object {
        $ext = [System.IO.Path]::GetExtension($_.FullName)
        ($CODE_EXTENSIONS -contains $ext) -or ($DOC_EXTENSIONS -contains $ext)
    }
    
    foreach ($f in $allFiles) {
        # Skip files in node_modules, target, etc.
        if ($f.FullName -match "(node_modules|target|dist|build|\.git)") {
            continue
        }
        
        $result = Test-File -filePath $f.FullName
        
        if ($stats) {
            $fileStats = Get-FileStats -filePath $f.FullName
            foreach ($key in $fileStats.Keys) {
                if ($allStats.ContainsKey($key)) {
                    $allStats[$key] += $fileStats[$key]
                }
                else {
                    $allStats[$key] = $fileStats[$key]
                }
            }
        }
        
        $processedFiles++
        if (-not $result) {
            $errorFiles++
        }
    }
}

# Print summary
Write-Host ""
Write-Host "Validation Summary:" -ForegroundColor Cyan
Write-Host "----------------" -ForegroundColor Cyan
Write-Host "Files processed: $processedFiles" -ForegroundColor White
Write-Host "Files with errors: $errorFiles" -ForegroundColor $(if ($errorFiles -gt 0) { "Red" } else { "Green" })
Write-Host "Success rate: $(100 - ($errorFiles / $processedFiles * 100))%" -ForegroundColor $(if ($errorFiles -eq 0) { "Green" } else { "Yellow" })

# Print stats if requested
if ($stats -and $allStats.Count -gt 0) {
    Write-Host ""
    Write-Host "Label Statistics:" -ForegroundColor Cyan
    Write-Host "----------------" -ForegroundColor Cyan
    
    $sortedStats = $allStats.GetEnumerator() | Sort-Object Name
    
    foreach ($stat in $sortedStats) {
        Write-Host "$($stat.Key): $($stat.Value)" -ForegroundColor White
    }
}

if ($errorFiles -gt 0) {
    exit 1
}
else {
    exit 0
} 