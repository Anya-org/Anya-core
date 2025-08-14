@echo off
REM Creates required cache directories on Windows host F: drive for Anya Core dev container.
set DIRS=anya-rust-target anya-cargo-registry anya-cargo-git anya-sccache
for %%D in (%DIRS%) do (
  if not exist F:\%%D (
    echo Creating F:\%%D
    mkdir F:\%%D
  ) else (
    echo Exists: F:\%%D
  )
)
echo (Optional) Run PowerShell: Get-PSDrive -Name F  to check free space.
echo Done.
