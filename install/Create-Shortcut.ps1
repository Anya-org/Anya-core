$WshShell = New-Object -comObject WScript.Shell
$Shortcut = $WshShell.CreateShortcut("$env:USERPROFILE\Desktop\Anya Core Installer.lnk")
$Shortcut.TargetPath = "powershell.exe"
$Shortcut.Arguments = "-NoProfile -ExecutionPolicy Bypass -File `"$PWD\Modules\AnyaInstaller\Start-AnyaInstaller.ps1`""
$Shortcut.WorkingDirectory = $PWD
$Shortcut.IconLocation = "powershell.exe,0"
$Shortcut.Save()

Write-Host "Created shortcut on desktop: 'Anya Core Installer'" -ForegroundColor Green
Write-Host "Right-click the shortcut and select 'Run as administrator' to start the installer" -ForegroundColor Yellow
