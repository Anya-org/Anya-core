# Bitcoin Core Alignment Fix Script [BPC-3][AIS-3]
# This script systematically fixes syntax errors across the codebase
# to ensure proper Bitcoin Core alignment

# Define the pattern to find
$pattern = '-> Result<\(\), Box<dyn Error>>'

# Define the directories to process
$directories = @("src\web5", "src\ml")

Write-Host "Starting Bitcoin Core alignment syntax fix [BPC-3][AIS-3]..."

foreach ($dir in $directories) {
    Write-Host "Processing directory: $dir"
    
    # Get all Rust files in the directory
    $files = Get-ChildItem -Path $dir -Filter "*.rs" -Recurse
    
    foreach ($file in $files) {
        Write-Host "Checking file: $($file.FullName)"
        
        # Read the file content
        $content = Get-Content -Path $file.FullName -Raw
        
        # Check if the pattern exists in the file
        if ($content -match $pattern) {
            Write-Host "  Found syntax issues, fixing..."
            
            # Fix function signatures with double return types
            # Pattern: match any return type followed by the error result type
            $newContent = $content -replace '(->.*?)  -> Result<\(\), Box<dyn Error>>', '$1'
            
            # Write the fixed content back to the file
            Set-Content -Path $file.FullName -Value $newContent
            
            Write-Host "  Fixed syntax issues in: $($file.Name)"
        } else {
            Write-Host "  No syntax issues found in: $($file.Name)"
        }
    }
}

# Fix the validation.rs file with the unclosed delimiter issue
$validationPath = "src\bitcoin\validation.rs"
if (Test-Path $validationPath) {
    Write-Host "Checking for unclosed delimiter in $validationPath"
    $validationContent = Get-Content -Path $validationPath -Raw
    
    # The issue seems to be with the TransactionValidator impl
    # Make sure there's a closing bracket after the last function
    if ($validationContent -match "impl TransactionValidator \{" -and 
        $validationContent -match "\s+pub fn stats\(&self\) -> &VerificationStats \{") {
        
        # Ensure proper closing of the implementation
        if (!($validationContent -match "\s+&self\.verification_stats\s+\}\s+\}\s*")) {
            Write-Host "  Fixing unclosed delimiter in TransactionValidator implementation"
            $fixedContent = $validationContent -replace "(\s+&self\.verification_stats\s+\})\s*$", '$1' + "`n}"
            Set-Content -Path $validationPath -Value $fixedContent
            Write-Host "  Fixed unclosed delimiter in: $validationPath"
        }
    }
}

Write-Host "Bitcoin Core alignment syntax fix completed [BPC-3][AIS-3]"
