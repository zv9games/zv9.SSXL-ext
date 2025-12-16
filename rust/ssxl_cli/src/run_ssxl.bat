@echo off
setlocal

cd /d "%~dp0.."

echo Killing stale processes...
taskkill /IM ssxl_cli.exe /F >nul 2>&1
taskkill /IM Godot_v4.5.1-stable_win64.exe /F >nul 2>&1

echo Building ssxl_ext...
cargo build -p ssxl_ext --features godot-binding
if %errorlevel% neq 0 (
    echo ssxl_ext build failed.
    exit /b 1
)

echo Copying ssxl_ext.dll...
copy /Y target\debug\ssxl_ext.dll ..\SSXLtester2\ssxl_ext.dll

echo Launching ssxl_cli...
cargo run -p ssxl_cli

endlocal
