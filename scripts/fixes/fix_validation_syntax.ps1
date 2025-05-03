# Script to fix the unclosed delimiter issue in src/bitcoin/validation.rs
# Ensures proper code structure and syntax compliance

param(
    [switch]$DryRun
)

$scriptName = "Validation Syntax Fixer"
$scriptVersion = "1.0.0"
$rootDir = Join-Path $PSScriptRoot "..\\.."

Write-Host "===== $scriptName v$scriptVersion =====" -ForegroundColor Cyan
Write-Host "Fixing syntax issues in validation.rs..."

# Target file with unclosed delimiter issue
$targetFilePath = Join-Path $rootDir "src\bitcoin\validation.rs"

if (-not (Test-Path $targetFilePath)) {
    Write-Host "Target file not found: $targetFilePath" -ForegroundColor Red
    exit 1
}

# Read the file content
$content = Get-Content -Path $targetFilePath -Raw
$originalContent = $content

# Check for unclosed delimiter around line 911
$lines = $content -split "`n"
$contextStart = [Math]::Max(0, 911 - 10)
$contextEnd = [Math]::Min($lines.Length - 1, 911 + 10)

Write-Host "Examining context around line 911 for unclosed delimiter" -ForegroundColor Yellow
for ($i = $contextStart; $i -le $contextEnd; $i++) {
    $lineText = $lines[$i].TrimEnd()
    
    # Look for typical unclosed delimiter patterns (unclosed braces, parentheses, etc.)
    $openBraces = ($lineText -split "{").Length - 1
    $closeBraces = ($lineText -split "}").Length - 1
    $openParens = ($lineText -split "\(").Length - 1
    $closeParens = ($lineText -split "\)").Length - 1
    
    if (($openBraces -gt $closeBraces) -or ($openParens -gt $closeParens)) {
        Write-Host "Line $($i+1) has potential unclosed delimiter: $lineText" -ForegroundColor Yellow
    }
    
    # Look for specific class/method declarations without closing braces
    if ($lineText -match "class\s+\w+\s*(\{|$)") {
        Write-Host "Line $($i+1) has class declaration that may be missing closure: $lineText" -ForegroundColor Yellow
    }
}

# Attempt to fix the unclosed delimiter
# First, look for any TransactionValidator.VerificationRecord class and ensure it has proper closure

$modified = $false

if ($content -match "class\s+TransactionValidator") {
    # Find the VerificationRecord inner class
    if ($content -match "class\s+VerificationRecord\s*\{[^}]*$") {
        # This pattern matches a class definition that doesn't have a closing brace
        Write-Host "Found unclosed VerificationRecord class definition" -ForegroundColor Yellow
        
        # Add a closing brace for the class
        $fixedContent = $content -replace "class\s+VerificationRecord\s*\{([^}]*)$", "class VerificationRecord {`$1}`n}"
        $content = $fixedContent
        $modified = $true
        Write-Host "Added closing brace for VerificationRecord class" -ForegroundColor Green
    }
}

# Sometimes the issue could be malformed comments or string literals
if (-not $modified) {
    # Check for unclosed block comments /* without */
    if ($content -match "/\*(?!\*/)(.*?)$") {
        $fixedContent = $content -replace "/\*(?!\*/)(.*?)$", "/* `$1 */"
        $content = $fixedContent
        $modified = $true
        Write-Host "Fixed unclosed block comment" -ForegroundColor Green
    }
    
    # Check for unclosed string literals " without closing "
    if ($content -match '"[^"]*$') {
        $fixedContent = $content -replace '"([^"]*$)', '"`$1"'
        $content = $fixedContent
        $modified = $true
        Write-Host "Fixed unclosed string literal" -ForegroundColor Green
    }
}

# If more specific patterns fail, try a general approach of balancing braces
if (-not $modified) {
    # Count total open and close braces in the file
    $totalOpenBraces = ($content -split "{").Length - 1
    $totalCloseBraces = ($content -split "}").Length - 1
    
    if ($totalOpenBraces -gt $totalCloseBraces) {
        $bracesMissing = $totalOpenBraces - $totalCloseBraces
        Write-Host "File has $bracesMissing missing closing braces" -ForegroundColor Yellow
        
        # Add closing braces at the end of the file
        $fixedContent = $content.TrimEnd() + ("`n}" * $bracesMissing) + "`n"
        $content = $fixedContent
        $modified = $true
        Write-Host "Added $bracesMissing closing braces at the end of the file" -ForegroundColor Green
    }
}

# Check if the content was modified
if ($modified) {
    if (-not $DryRun) {
        # Write the fixed content back to the file
        $content | Set-Content -Path $targetFilePath -Encoding UTF8
        Write-Host "Fixed unclosed delimiter in validation.rs" -ForegroundColor Green
    } else {
        Write-Host "Would fix unclosed delimiter in validation.rs (dry run)" -ForegroundColor Yellow
    }
} else {
    Write-Host "Could not automatically identify or fix the unclosed delimiter issue" -ForegroundColor Red
    Write-Host "Manual inspection may be required" -ForegroundColor Red
}

Write-Host "Validation syntax fixing completed" -ForegroundColor Green
