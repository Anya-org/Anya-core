# GitHub Credentials and GPG Setup for MCP Tools
# This script configures GitHub credentials for MCP tools integration
# Sets up authentication for Bo_theBig (botshelomokoka@gmail.com)
# Adheres to Bitcoin Core principles of security and transparency

param(
    [switch]$ConfigureGPG = $false,
    [switch]$StoreCredentials = $true,
    [string]$GPGKeyPath
)

$scriptName = "GitHub MCP Credentials Setup"
$scriptVersion = "1.0.0"
$logFile = Join-Path $PSScriptRoot "github-credentials-setup.log"

function Write-Log {
    param (
        [string]$Message,
        [string]$Level = "INFO"
    )
    
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    $logMessage = "[$timestamp] [$Level] $Message"
    
    # Output to console with appropriate color
    switch ($Level) {
        "INFO" { Write-Host $logMessage -ForegroundColor Cyan }
        "SUCCESS" { Write-Host $logMessage -ForegroundColor Green }
        "WARNING" { Write-Host $logMessage -ForegroundColor Yellow }
        "ERROR" { Write-Host $logMessage -ForegroundColor Red }
        default { Write-Host $logMessage }
    }
    
    # Append to log file
    Add-Content -Path $logFile -Value $logMessage
}

function Test-GitHubConnection {
    try {
        $result = git ls-remote https://github.com/anya-org/anya-core.git HEAD 2>&1
        if ($LASTEXITCODE -eq 0) {
            return $true
        } else {
            return $false
        }
    } catch {
        return $false
    }
}

function Setup-GitHubCredentials {
    # Set Git user configuration
    git config --global user.name "Bo_theBig"
    git config --global user.email "botshelomokoka@gmail.com"
    
    # Store credentials using Git credential helper
    if ($StoreCredentials) {
        git config --global credential.helper store
        
        Write-Log "Git global credentials configured for user: Bo_theBig <botshelomokoka@gmail.com>" -Level "SUCCESS"
        Write-Log "Credentials will be stored using Git credential helper" -Level "INFO"
    }
    
    # Create MCP credentials file
    $mcpCredentialsDir = Join-Path $HOME ".mcp"
    $mcpCredentialsFile = Join-Path $mcpCredentialsDir "github-credentials.json"
    
    if (-not (Test-Path $mcpCredentialsDir)) {
        New-Item -ItemType Directory -Path $mcpCredentialsDir -Force | Out-Null
    }
    
    $credentials = @{
        username = "Bo_theBig"
        email = "botshelomokoka@gmail.com"
        owner = "anya-org"
        repo = "anya-core"
        timestamp = Get-Date -Format "yyyy-MM-ddTHH:mm:ssZ"
    }
    
    $credentials | ConvertTo-Json | Set-Content -Path $mcpCredentialsFile
    
    Write-Log "MCP GitHub credentials saved to: $mcpCredentialsFile" -Level "SUCCESS"
}

function Setup-GPGKeys {
    if (-not $ConfigureGPG) {
        Write-Log "GPG setup skipped (use -ConfigureGPG to enable)" -Level "INFO"
        return
    }
    
    # Check if GPG is installed
    try {
        $gpgVersion = gpg --version
        Write-Log "GPG is installed: $($gpgVersion[0])" -Level "INFO"
    } catch {
        Write-Log "GPG is not installed. Please install GPG tools first." -Level "ERROR"
        return
    }
    
    # Import GPG key if path provided
    if ($GPGKeyPath -and (Test-Path $GPGKeyPath)) {
        Write-Log "Importing GPG key from: $GPGKeyPath" -Level "INFO"
        gpg --import $GPGKeyPath
        
        # Get the key ID from the imported key
        $keyId = gpg --list-secret-keys --keyid-format LONG | Select-String "sec" | ForEach-Object { 
            if ($_ -match "sec\s+[^/]+/([A-F0-9]+)") { 
                $matches[1]
            }
        } | Select-Object -First 1
        
        if ($keyId) {
            # Configure Git to use this signing key
            git config --global user.signingkey $keyId
            git config --global commit.gpgsign true
            
            Write-Log "Git configured to use GPG key: $keyId for signing" -Level "SUCCESS"
            
            # Add to MCP credentials file
            $mcpCredentialsFile = Join-Path $HOME ".mcp" "github-credentials.json"
            if (Test-Path $mcpCredentialsFile) {
                $credentials = Get-Content -Path $mcpCredentialsFile | ConvertFrom-Json
                $credentials | Add-Member -NotePropertyName "gpg_key_id" -NotePropertyValue $keyId -Force
                $credentials | ConvertTo-Json | Set-Content -Path $mcpCredentialsFile
                
                Write-Log "GPG key ID added to MCP credentials file" -Level "SUCCESS"
            }
        } else {
            Write-Log "Could not identify GPG key ID after import" -Level "WARNING"
        }
    } else {
        Write-Log "No valid GPG key path provided. GPG key import skipped." -Level "WARNING"
    }
}

function Setup-GitHubMCPConfig {
    # Create MCP config file with GitHub authentication
    $mcpConfigDir = Join-Path $HOME ".mcp"
    $mcpConfigFile = Join-Path $mcpConfigDir "config.json"
    
    if (-not (Test-Path $mcpConfigDir)) {
        New-Item -ItemType Directory -Path $mcpConfigDir -Force | Out-Null
    }
    
    $config = @{
        github = @{
            username = "Bo_theBig"
            email = "botshelomokoka@gmail.com"
            auth_method = "direct"
            default_owner = "anya-org"
            default_repo = "anya-core"
        }
        user_preferences = @{
            log_level = "INFO"
            auto_update = $true
        }
        bitcoin_core = @{
            principles = @("decentralization", "security", "immutability", "transparency")
            version = "24.0"
        }
    }
    
    $config | ConvertTo-Json -Depth 3 | Set-Content -Path $mcpConfigFile
    
    Write-Log "MCP configuration saved to: $mcpConfigFile" -Level "SUCCESS"
    
    # Create environment variable for session
    $env:MCP_GITHUB_USERNAME = "Bo_theBig"
    $env:MCP_GITHUB_EMAIL = "botshelomokoka@gmail.com"
    $env:MCP_GITHUB_DEFAULT_OWNER = "anya-org"
    $env:MCP_GITHUB_DEFAULT_REPO = "anya-core"
    
    Write-Log "Environment variables set for current session" -Level "INFO"
    
    # Create a startup script to set these variables
    $startupScriptPath = Join-Path $PSScriptRoot "set_mcp_env.ps1"
    @"
# Set MCP GitHub environment variables
`$env:MCP_GITHUB_USERNAME = "Bo_theBig"
`$env:MCP_GITHUB_EMAIL = "botshelomokoka@gmail.com"
`$env:MCP_GITHUB_DEFAULT_OWNER = "anya-org"
`$env:MCP_GITHUB_DEFAULT_REPO = "anya-core"

Write-Host "MCP GitHub environment variables set for: Bo_theBig" -ForegroundColor Green
"@ | Set-Content -Path $startupScriptPath
    
    Write-Log "Created startup script for setting environment variables: $startupScriptPath" -Level "SUCCESS"
}

function Export-MCPCredentialsToEnv {
    # Save environment variables to a file that can be sourced in shell sessions
    $envFilePath = Join-Path $PSScriptRoot "mcp_github_env.ps1"
    
    @"
# MCP GitHub Environment Variables
# Source this file to set up GitHub credentials for MCP tools
# Generated: $(Get-Date -Format "yyyy-MM-dd HH:mm:ss")

`$env:MCP_GITHUB_USERNAME = "Bo_theBig"
`$env:MCP_GITHUB_EMAIL = "botshelomokoka@gmail.com"
`$env:MCP_GITHUB_DEFAULT_OWNER = "anya-org"
`$env:MCP_GITHUB_DEFAULT_REPO = "anya-core"

# Usage instructions:
# PowerShell: . "$envFilePath"
# Command line: powershell -File "$envFilePath"

Write-Host "MCP GitHub credentials loaded for: Bo_theBig <botshelomokoka@gmail.com>" -ForegroundColor Green
"@ | Set-Content -Path $envFilePath
    
    Write-Log "Created environment variables export file: $envFilePath" -Level "SUCCESS"
    
    return $envFilePath
}

# Main execution
Write-Log "===== $scriptName v$scriptVersion =====" -Level "INFO"
Write-Log "Starting GitHub credentials setup..." -Level "INFO"

# Setup Git user configuration
Setup-GitHubCredentials

# Setup GPG keys if requested
Setup-GPGKeys

# Create MCP configuration
Setup-GitHubMCPConfig

# Export credentials to environment
$envFilePath = Export-MCPCredentialsToEnv

# Check GitHub connection
if (Test-GitHubConnection) {
    Write-Log "GitHub connection test successful!" -Level "SUCCESS"
} else {
    Write-Log "GitHub connection test failed. Please check your credentials or network connection." -Level "WARNING"
}

Write-Log "GitHub credentials setup complete" -Level "SUCCESS"
Write-Log "To use these credentials in your current session, run: . `"$envFilePath`"" -Level "INFO"
