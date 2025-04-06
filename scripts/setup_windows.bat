@echo off
REM Anya Core Windows Setup Script [AIS-3][BPC-3]
echo Setting up Anya Core for Windows...

REM Create platform configuration directories
mkdir config\platform 2>nul

REM Create Windows configuration file if it doesn't exist
if not exist config\platform\windows.yaml (
    echo Creating Windows platform configuration...
    echo paths:> config\platform\windows.yaml
    echo   base: "%%USERPROFILE%%\.anya">> config\platform\windows.yaml
    echo   data: "%%USERPROFILE%%\.anya\data">> config\platform\windows.yaml
    echo   logs: "%%USERPROFILE%%\.anya\logs">> config\platform\windows.yaml
    echo   bitcoin: "%%APPDATA%%\Bitcoin">> config\platform\windows.yaml
    echo.>> config\platform\windows.yaml
    echo security:>> config\platform\windows.yaml
    echo   keystore: "%%USERPROFILE%%\.anya\keystore">> config\platform\windows.yaml
    echo   permissions: "0600">> config\platform\windows.yaml
    echo.>> config\platform\windows.yaml
    echo network:>> config\platform\windows.yaml
    echo   default_interface: "auto">> config\platform\windows.yaml
    echo   timeout_ms: 3000>> config\platform\windows.yaml
)

REM Create Unix configuration file if it doesn't exist
if not exist config\platform\unix.yaml (
    echo Creating Unix platform configuration...
    echo paths:> config\platform\unix.yaml
    echo   base: "$HOME/.anya">> config\platform\unix.yaml
    echo   data: "$HOME/.anya/data">> config\platform\unix.yaml
    echo   logs: "$HOME/.anya/logs">> config\platform\unix.yaml
    echo   bitcoin: "$HOME/.bitcoin">> config\platform\unix.yaml
    echo.>> config\platform\unix.yaml
    echo security:>> config\platform\unix.yaml
    echo   keystore: "$HOME/.anya/keystore">> config\platform\unix.yaml
    echo   permissions: "0600">> config\platform\unix.yaml
    echo.>> config\platform\unix.yaml
    echo network:>> config\platform\unix.yaml
    echo   default_interface: "auto">> config\platform\unix.yaml
    echo   timeout_ms: 3000>> config\platform\unix.yaml
)

REM Fix path handling in Rust files
echo Fixing path handling in Rust files...
python .\scripts\fix_path_handling.py .\src

REM Create Anya configuration directories
echo Creating Anya configuration directories...
mkdir "%USERPROFILE%\.anya" 2>nul
mkdir "%USERPROFILE%\.anya\config" 2>nul
mkdir "%USERPROFILE%\.anya\data" 2>nul
mkdir "%USERPROFILE%\.anya\logs" 2>nul
mkdir "%USERPROFILE%\.anya\keystore" 2>nul

REM Copy default configuration
echo Copying default configuration...
copy config\default.yaml "%USERPROFILE%\.anya\config\" >nul

echo Setup complete!
echo You can now build Anya Core with: cargo build --release 