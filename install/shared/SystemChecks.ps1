#Requires -Version 5.0

$systemRequirements = @{
    Standalone = @{
        RAM = 8
        CPU = 4
        Disk = 50
    }
    Node = @{
        RAM = 16
        CPU = 8
        Disk = 100
    }
    Cluster = @{
        RAM = 32
        CPU = 16
        Disk = 200
    }
}

function Test-SystemRequirements {
    param(
        [ValidateSet('Standalone', 'Node', 'Cluster')]
        [string]$DeploymentType = 'Standalone'
    )
    
    $reqs = $systemRequirements[$DeploymentType]
    $systemInfo = Get-CimInstance Win32_ComputerSystem
    $diskInfo = Get-PSDrive C
    
    $results = @{
        RAM = [math]::Round($systemInfo.TotalPhysicalMemory/1GB, 2) -ge $reqs.RAM
        CPU = $systemInfo.NumberOfLogicalProcessors -ge $reqs.CPU
        Disk = [math]::Round($diskInfo.Free/1GB, 2) -ge $reqs.Disk
        PowerShell = $PSVersionTable.PSVersion.Major -ge 5
    }
    
    return @{
        Passed = $results.Values -notcontains $false
        Details = $results
        SystemInfo = @{
            RAM = [math]::Round($systemInfo.TotalPhysicalMemory/1GB, 2)
            CPU = $systemInfo.NumberOfLogicalProcessors
            Disk = [math]::Round($diskInfo.Free/1GB, 2)
        }
        Requirements = $reqs
    }
}

Export-ModuleMember -Function * -Variable systemRequirements
