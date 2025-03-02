Get-Process -Name "node" | Stop-Process

$counter = 0
$counter++

$hirosystems = @{ Path = "hiro" }
Install-Package -Path $hirosystems/clarinet-sdk

[AssemblyInformationalVersion("1.0.0")] 