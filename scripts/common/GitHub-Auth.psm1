# GitHub CLI Authentication Helper for PowerShell
# Provides standardized GitHub authentication using GitHub CLI (gh)
# Adheres to Bitcoin Core principles of security and transparency

$ErrorActionPreference = "Stop"

# Check if GitHub CLI is installed
function Test-GitHubCLI {
    try {
        $null = gh --version
        return $true
    } catch {
        Write-Error "GitHub CLI (gh) is not installed. Please install it first."
        return $false
    }
}

# Check if GitHub CLI is authenticated
function Test-GitHubAuth {
    [CmdletBinding()]
    param (
        [Parameter()]
        [bool]$AutoRun = $false,
        
        [Parameter()]
        [bool]$YesAll = $false
    )
    
    try {
        $null = gh auth status
        return $true
    } catch {
        Write-Warning "Not authenticated with GitHub CLI."
        
        # If AutoRun is enabled, attempt automatic login
        if ($AutoRun) {
            Write-Host "Attempting automatic login with GitHub CLI..." -ForegroundColor Yellow
            
            $loginArgs = @()
            if ($YesAll) {
                # Use Web mode with default options for automation
                $loginArgs = @("--web")
            }
            
            try {
                $loginCommand = "gh auth login $($loginArgs -join ' ')"
                Invoke-Expression $loginCommand
                
                # Check if login was successful
                try {
                    $null = gh auth status
                    Write-Host "Successfully authenticated with GitHub CLI." -ForegroundColor Green
                    return $true
                } catch {
                    Write-Error "Automatic login failed."
                    return $false
                }
            } catch {
                Write-Error "Automatic login failed. Please run 'gh auth login' manually."
                return $false
            }
        } else {
            Write-Error "Please run 'gh auth login'."
            return $false
        }
    }
}

# Get GitHub auth info
function Get-GitHubAuthInfo {
    [CmdletBinding()]
    param (
        [Parameter()]
        [bool]$AutoRun = $false,
        
        [Parameter()]
        [bool]$YesAll = $false
    )
    
    if (-not (Test-GitHubCLI)) {
        return $null
    }
    
    if (-not (Test-GitHubAuth -AutoRun $AutoRun -YesAll $YesAll)) {
        return $null
    }
    
    try {
        # Get authenticated user info
        $authStatus = gh auth status --hostname github.com 2>&1 | Out-String
        $usernameMatch = [regex]::Match($authStatus, 'Logged in to github\.com account (\S+)')
        $username = if ($usernameMatch.Success) { $usernameMatch.Groups[1].Value } else { $null }
        
        # Get token from GitHub CLI
        $token = gh auth token
        
        # Get user data including email
        $userData = gh api user | ConvertFrom-Json
        $email = $userData.email
        
        # If email is not public, try to get from email API or git config
        if ([string]::IsNullOrEmpty($email)) {
            try {
                $emailData = gh api user/emails | ConvertFrom-Json
                $primaryEmail = $emailData | Where-Object { $_.primary -eq $true } | Select-Object -First 1
                if ($primaryEmail) {
                    $email = $primaryEmail.email
                }
            } catch {
                # Try git config as fallback
                try {
                    $email = git config --get user.email
                } catch {
                    $email = $null
                }
            }
        }
        
        return @{
            Username = $username
            Token = $token
            Email = $email
        }
    } catch {
        Write-Error "Error getting GitHub authentication: $_"
        return $null
    }
}

# Setup MCP environment variables for GitHub
function Set-MCPEnvironment {
    [CmdletBinding()]
    param (
        [Parameter()]
        [string]$DefaultOwner = "anya-org",
        
        [Parameter()]
        [string]$DefaultRepo = "anya-core",
        
        [Parameter()]
        [bool]$AutoRun = $false,
        
        [Parameter()]
        [bool]$YesAll = $false
    )
    
    $auth = Get-GitHubAuthInfo -AutoRun $AutoRun -YesAll $YesAll
    if ($null -eq $auth) {
        return $false
    }
    
    # Set environment variables
    $env:MCP_GITHUB_USERNAME = $auth.Username
    $env:MCP_GITHUB_EMAIL = $auth.Email
    $env:MCP_GITHUB_DEFAULT_OWNER = $DefaultOwner
    $env:MCP_GITHUB_DEFAULT_REPO = $DefaultRepo
    $env:GITHUB_TOKEN = $auth.Token
    
    Write-Host "MCP environment variables set for GitHub user: $($auth.Username)" -ForegroundColor Green
    return $true
}

# Create MCP GitHub configuration file
function New-MCPGitHubConfig {
    [CmdletBinding()]
    param (
        [Parameter(Mandatory = $true)]
        [string]$OutputFile,
        
        [Parameter()]
        [string]$DefaultOwner = "anya-org",
        
        [Parameter()]
        [string]$DefaultRepo = "anya-core",
        
        [Parameter()]
        [bool]$AutoRun = $false,
        
        [Parameter()]
        [bool]$YesAll = $false
    )
    
    $auth = Get-GitHubAuthInfo -AutoRun $AutoRun -YesAll $YesAll
    if ($null -eq $auth) {
        return $false
    }
    
    # Create directory if it doesn't exist
    $configDir = Split-Path -Parent $OutputFile
    if (-not (Test-Path $configDir)) {
        New-Item -ItemType Directory -Path $configDir -Force | Out-Null
    }
    
    # Create config object
    $config = @{
        github = @{
            username = $auth.Username
            email = $auth.Email
            auth_method = "github-cli"
            default_owner = $DefaultOwner
            default_repo = $DefaultRepo
        }
        user_preferences = @{
            log_level = "INFO"
            auto_update = $true
            auto_run = $AutoRun
            yes_all = $YesAll
        }
        bitcoin_core = @{
            principles = @("decentralization", "security", "immutability", "transparency")
            version = "24.0"
        }
    }
    
    # Save config to file
    $config | ConvertTo-Json -Depth 3 | Set-Content -Path $OutputFile
    
    Write-Host "MCP GitHub configuration saved to: $OutputFile" -ForegroundColor Green
    return $true
}

# Parse command line arguments for automation flags
function Get-GitHubCliArgs {
    [CmdletBinding()]
    param (
        [Parameter(ValueFromRemainingArguments = $true)]
        [string[]]$Arguments
    )
    
    $autoRun = $false
    $yesAll = $false
    
    foreach ($arg in $Arguments) {
        if ($arg -eq "--auto-run") {
            $autoRun = $true
        } elseif ($arg -eq "--yes-all") {
            $yesAll = $true
        }
    }
    
    return @{
        AutoRun = $autoRun
        YesAll = $yesAll
    }
}

# Export functions when script is imported
Export-ModuleMember -Function Test-GitHubCLI, Test-GitHubAuth, Get-GitHubAuthInfo, Set-MCPEnvironment, New-MCPGitHubConfig, Get-GitHubCliArgs
