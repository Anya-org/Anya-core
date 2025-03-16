function Update-SecurityPatterns {
    param(
        [Parameter(Mandatory=$true)]
        [string]$ConfigPath
    )

    # Validate config file exists
    if (-Not (Test-Path $ConfigPath)) {
        Write-Error "Security patterns config file not found at $ConfigPath"
        exit 1
    }

    # Load security patterns configuration
    try {
        $patterns = Get-Content $ConfigPath | ConvertFrom-Json
    } catch {
        Write-Error "Failed to parse security patterns config: $_"
        exit 1
    }

    # Create security patterns directory if needed
    $securityDir = Join-Path $PSScriptRoot "../configs/security-patterns"
    if (-Not (Test-Path $securityDir)) {
        New-Item -ItemType Directory -Path $securityDir | Out-Null
    }

    # Generate pattern files for each check type
    foreach ($patternType in $patterns.PSObject.Properties.Name) {
        $patternConfig = $patterns.$patternType
        $outputPath = Join-Path $securityDir "$patternType-patterns.clar"
        
        @(
            ";; Security patterns for $patternType",
            ";; Generated at $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')",
            ";; Auto-updated from $ConfigPath",
            "",
            "(define-constant $patternType-patterns",
            "  (list",
            "    " + ($patternConfig.patterns -join "`n    "),
            "  ))",
            ""
        ) | Out-File $outputPath -Encoding utf8

        Write-Host "Generated security patterns for $patternType at $outputPath"
    }

    # Update test framework configuration
    $testConfigPath = Join-Path $PSScriptRoot "../configs/test-config.json"
    $testConfig = Get-Content $testConfigPath | ConvertFrom-Json
    
    # Add security_patterns property if missing
    if (-not (Get-Member -InputObject $testConfig -Name security_patterns)) {
        $testConfig | Add-Member -MemberType NoteProperty -Name security_patterns -Value $securityDir
    }
    else {
        $testConfig.security_patterns = $securityDir
    }

    $testConfig | ConvertTo-Json -Depth 5 | Out-File $testConfigPath

    Write-Host "Security patterns updated successfully!"
} 