# Configure Git Signing
# This script configures Git to use GPG for commit signing

# User details from custom instructions
$GIT_USER_NAME = "bo_thebig"
$GIT_USER_EMAIL = "botshelomokokoka@gmail.com"

Write-Host "Configuring Git user details and commit signing..." -ForegroundColor Green

# Check if git is installed
try {
    $gitVersion = git --version
    Write-Host "Git is installed: $gitVersion" -ForegroundColor Green
} catch {
    Write-Host "Git is not installed or not in PATH. Please install Git and try again." -ForegroundColor Red
    exit 1
}

# Check if GPG is installed
try {
    $gpgVersion = gpg --version
    Write-Host "GPG is installed: $($gpgVersion[0])" -ForegroundColor Green
} catch {
    Write-Host "GPG is not installed or not in PATH. Please install GPG and try again." -ForegroundColor Red
    exit 1
}

# Configure Git user information
Write-Host "Setting Git user name to: $GIT_USER_NAME" -ForegroundColor Yellow
git config --global user.name "$GIT_USER_NAME"

Write-Host "Setting Git user email to: $GIT_USER_EMAIL" -ForegroundColor Yellow
git config --global user.email "$GIT_USER_EMAIL"

# List available GPG keys
Write-Host "Available GPG keys:" -ForegroundColor Yellow
gpg --list-secret-keys --keyid-format LONG

# Prompt for key selection
$keyId = Read-Host "Enter the GPG key ID to use for signing (the 16-character ID after 'sec rsa4096/')"

# Configure Git to use the selected GPG key
if ($keyId) {
    Write-Host "Setting Git signing key to: $keyId" -ForegroundColor Yellow
    git config --global user.signingkey $keyId
    
    # Enable commit signing by default
    Write-Host "Enabling commit signing by default" -ForegroundColor Yellow
    git config --global commit.gpgsign true
    
    # Set GPG program path based on OS
    if ($IsWindows -or $env:OS -match "Windows") {
        # For Windows, typically the path is different
        $gpgPath = Read-Host "Enter the full path to gpg.exe (leave empty for auto-detect)"
        if (-not $gpgPath) {
            # Try common paths
            $commonPaths = @(
                "${env:ProgramFiles(x86)}\GnuPG\bin\gpg.exe",
                "${env:ProgramFiles}\GnuPG\bin\gpg.exe",
                "${env:ProgramFiles}\Git\usr\bin\gpg.exe"
            )
            
            foreach ($path in $commonPaths) {
                if (Test-Path $path) {
                    $gpgPath = $path
                    break
                }
            }
        }
        
        if ($gpgPath) {
            $gpgPath = $gpgPath -replace "\\", "/"
            git config --global gpg.program $gpgPath
            Write-Host "GPG program path set to: $gpgPath" -ForegroundColor Green
        } else {
            Write-Host "Could not find GPG program path. You may need to set it manually." -ForegroundColor Yellow
        }
    }
    
    # Test the configuration
    Write-Host "Testing GPG configuration..." -ForegroundColor Yellow
    $testResult = git config --global --get commit.gpgsign
    if ($testResult -eq "true") {
        Write-Host "Git is now configured to sign commits automatically!" -ForegroundColor Green
    } else {
        Write-Host "Configuration may have failed. Please check your settings." -ForegroundColor Red
    }
    
    # Information about adding the key to GitHub/GitLab
    Write-Host "`nTo add this GPG key to GitHub/GitLab:" -ForegroundColor Cyan
    Write-Host "1. Run: gpg --armor --export $keyId" -ForegroundColor Cyan
    Write-Host "2. Copy the output (including BEGIN and END lines)" -ForegroundColor Cyan
    Write-Host "3. Paste it into your GitHub/GitLab settings under SSH and GPG keys" -ForegroundColor Cyan
} else {
    Write-Host "No key ID provided. Git signing configuration aborted." -ForegroundColor Red
}

# Instructions for signing commits
Write-Host "`nTo sign commits:" -ForegroundColor Cyan
Write-Host "- Commits will be automatically signed (commit.gpgsign=true)" -ForegroundColor Cyan
Write-Host "- For manual signing: git commit -S -m 'your message'" -ForegroundColor Cyan
Write-Host "- To verify a signed commit: git verify-commit <commit-hash>" -ForegroundColor Cyan

Write-Host "`nConfiguration complete!" -ForegroundColor Green 