@echo off
echo Checking if Rust is installed...

where rustc >nul 2>nul
if %errorlevel% neq 0 (
    echo Installing Rust...
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rustup-init.exe
    rustup-init.exe -y
    del rustup-init.exe
) else (
    echo Rust is already installed
)

echo Building the project...
cargo build --release

echo Creating run script...
(
echo @echo off
echo echo Starting Sensex Backend...
echo .\target\release\sensex-backend.exe
) > run.bat

echo Installation complete!
echo You can now run the application using: run.bat
pause
