@echo off
title QCLang Windows Installer
color 0A

echo ========================================
echo    QCLang Quantum Compiler Installer
echo ========================================
echo.

echo This will install QCLang for Windows.
echo.
echo Requirements:
echo   - Windows 10/11
echo   - PowerShell 5.1+
echo   - Internet connection
echo.
set /p confirm="Continue? (Y/N): "
if /i not "%confirm%"=="Y" goto :eof

echo.
echo Checking PowerShell...
where powershell >nul 2>nul
if errorlevel 1 (
    echo ERROR: PowerShell is required.
    echo Download from: https://aka.ms/powershell
    pause
    exit /b 1
)

echo.
echo Downloading installer...
powershell -Command "Invoke-WebRequest -Uri 'https://raw.githubusercontent.com/yourusername/qclang/main/windows/installer/install.ps1' -OutFile '%TEMP%\qclang-install.ps1'"

echo.
echo Running installer...
echo ---------------------------------------------------
powershell -ExecutionPolicy Bypass -File "%TEMP%\qclang-install.ps1"
echo ---------------------------------------------------

echo.
echo Cleaning up...
del "%TEMP%\qclang-install.ps1" 2>nul

echo.
echo Installation complete!
echo.
echo To use QCLang:
echo   1. Open a NEW Command Prompt
echo   2. Type: qclang --version
echo   3. Try: qclang examples\hello.qc
echo.
pause