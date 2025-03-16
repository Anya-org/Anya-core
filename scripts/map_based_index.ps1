function Get-MappedIndex {
    param(
        [string]$SystemMapPath = "SYSTEM_MAP.md",
        [string]$OutputFile = "REPO_INDEX.json"
    )

    $index = @{
        core = @{}
        bitcoin = @{}
        enterprise = @{}
        mobile = @{}
        docs = @{}
        validation = @{}
    }

    # Parse SYSTEM_MAP categories
    $content = Get-Content $SystemMapPath -Raw
    $currentSection = ""
    
    $content -split "[\r\n]+" | ForEach-Object {
        if ($_ -match "^## (.*)") {
            $currentSection = $matches[1].Trim()
        }
        elseif ($_ -match "\[(.*)\]\((.*)\)") {
            $name = $matches[1].Trim()
            $path = $matches[2].Trim()
            
            if (-not [System.IO.Path]::IsPathRooted($path)) {
                $fullPath = Join-Path (Get-Location) $path
            }
            
            if (Test-Path $fullPath) {
                $entry = Get-FileSystemEntry -Path $fullPath
                Add-MappedEntry -Section $currentSection -Name $name -Entry $entry
            }
        }
    }

    $index | ConvertTo-Json -Depth 5 | Out-File $OutputFile
    Update-ValidationStatus -IndexPath $OutputFile
}

function Get-FileSystemEntry {
    param($Path)
    
    return @{
        path = $Path
        type = if (Test-Path $Path -PathType Container) { "directory" } else { "file" }
        hash = (Get-FileHash $Path -Algorithm SHA256 -ErrorAction SilentlyContinue).Hash
        modified = (Get-Item $Path).LastWriteTimeUtc.ToString("o")
        bitcoin_adherence = if ($Path -match "anya-bitcoin") { 
            (cargo run --bin protocol_checker -- $Path | Out-String).Trim()
        } else { $null }
    }
}

function Add-MappedEntry {
    param($Section, $Name, $Entry)
    $index[$Section][$Name] = $Entry
}

function Update-ValidationStatus {
    param($IndexPath)
    $indexData = Get-Content $IndexPath | ConvertFrom-Json
    
    $total = ($indexData.bitcoin.PSObject.Properties | Measure-Object).Count
    $compliant = ($indexData.bitcoin.PSObject.Properties | 
        Where-Object { $_.Value.bitcoin_adherence -ge 0.9 }).Count
    
    $mapContent = Get-Content .\SYSTEM_MAP.md -Raw
    $mapContent = $mapContent -replace "{{INDEX_TIMESTAMP}}", [DateTime]::Now.ToString("o")
    $mapContent = $mapContent -replace "{{COMPONENT_COUNT}}", $total
    $mapContent = $mapContent -replace "{{ADHERENCE_SCORE}}", [math]::Round(($compliant/$total)*100, 2)
    $mapContent | Out-File .\SYSTEM_MAP.md
} 