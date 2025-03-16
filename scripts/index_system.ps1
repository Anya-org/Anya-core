function Get-SystemIndex {
    param(
        [string]$RootDir = ".",
        [string]$OutputFile = "SYSTEM_INDEX.json"
    )

    $index = @{
        timestamp = [DateTime]::UtcNow.ToString("o")
        components = @()
        documents = @()
        code = @()
        security = @()
    }

    Get-ChildItem -Path $RootDir -Recurse -File | ForEach-Object {
        $entry = @{
            path = $_.FullName
            type = $_.Extension
            size = $_.Length
            modified = $_.LastWriteTimeUtc.ToString("o")
            hash = (Get-FileHash $_.FullName -Algorithm SHA256).Hash
            bitcoin_adherence = $null
            security_status = "unreviewed"
        }

        # Classify components
        if ($_.FullName -match "\\anya-bitcoin\\") {
            $entry.bitcoin_adherence = (Get-BitcoinAdherenceScore $_.FullName)
            $index.components += $entry
        }
        elseif ($_.Extension -match "\.md|\.txt") {
            $index.documents += $entry
        }
        elseif ($_.Extension -match "\.rs|\.go|\.tsx") {
            $index.code += $entry
        }
        elseif ($_.FullName -match "security|audit") {
            $index.security += $entry
        }
    }

    $index | ConvertTo-Json -Depth 4 | Out-File $OutputFile
    Update-SystemMap -IndexPath $OutputFile
}

function Get-BitcoinAdherenceScore {
    param([string]$FilePath)
    # Use existing Rust compliance checker
    cargo run --bin protocol_checker -- $FilePath | Out-String
} 