function Test-SystemChecks {
    $networkOk = Test-Connection -ComputerName "8.8.8.8" -Count 2 -Quiet
    $diskFree = (Get-PSDrive -Name C).Free
    $diskOk = $diskFree -gt 10GB
    return @{
        "NetworkStatus" = ($networkOk ? "Connected" : "Not Connected")
        "DiskSpaceOK"   = $diskOk
        "FreeDiskSpace" = "{0:N2} GB" -f ($diskFree/1GB)
    }
}

Export-ModuleMember -Function Test-SystemChecks
