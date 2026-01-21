<#
.SYNOPSIS
    QCLang Quantum Compiler Windows Installer
.DESCRIPTION
    Installs QCLang compiler for Windows with a single command.
.EXAMPLE
    PowerShell -Command "iwr -useb https://qclang.dev/install.ps1 | iex"
.NOTES
    Version: 1.0
    Author: QCLang Team
#>

param(
    [string]$InstallPath = "$env:LOCALAPPDATA\QCLang",
    [switch]$Silent,
    [switch]$Help
)

# Show help
if ($Help) {
    Write-Host "QCLang Windows Installer" -ForegroundColor Cyan
    Write-Host "========================" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Usage:" -ForegroundColor Yellow
    Write-Host "  PowerShell -Command `"iwr -useb https://qclang.dev/install.ps1 | iex`""
    Write-Host "  .\install.ps1 [-InstallPath C:\Path] [-Silent] [-Help]"
    Write-Host ""
    Write-Host "Options:" -ForegroundColor Yellow
    Write-Host "  -InstallPath    Installation directory (default: %LOCALAPPDATA%\QCLang)"
    Write-Host "  -Silent         Silent installation (no prompts)"
    Write-Host "  -Help           Show this help message"
    exit 0
}

# Banner
if (-not $Silent) {
    Write-Host ""
    Write-Host "┌─────────────────────────────────────────────────────┐" -ForegroundColor DarkGray
    Write-Host "│" -NoNewline -ForegroundColor DarkGray
    Write-Host "      QCLang Quantum Compiler v0.2.0 Windows        " -NoNewline -ForegroundColor Cyan
    Write-Host "│" -ForegroundColor DarkGray
    Write-Host "│" -NoNewline -ForegroundColor DarkGray
    Write-Host "        One-command quantum development            " -NoNewline -ForegroundColor DarkGray
    Write-Host "│" -ForegroundColor DarkGray
    Write-Host "└─────────────────────────────────────────────────────┘" -ForegroundColor DarkGray
    Write-Host ""
}

# Check admin rights (not required, but helpful)
$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")
if ($isAdmin -and (-not $Silent)) {
    Write-Host "⚠️  Running as administrator" -ForegroundColor Yellow
}

# Create installation directory structure
$directories = @(
    "$InstallPath\bin",
    "$InstallPath\examples",
    "$InstallPath\temp"
)

foreach ($dir in $directories) {
    if (-not (Test-Path $dir)) {
        New-Item -ItemType Directory -Path $dir -Force | Out-Null
        if (-not $Silent) { Write-Host "✓ Created: $dir" -ForegroundColor Green }
    }
}

# Download QCLang binary
$downloadUrls = @(
    "https://github.com/yourusername/qclang/releases/download/v0.2.0/qclang-windows.exe",
    "https://github.com/yourusername/qclang/releases/latest/download/qclang-windows.exe"
)

$binaryPath = "$InstallPath\bin\qclang.exe"
$downloadSuccess = $false

if (-not $Silent) {
    Write-Host "`n⬇️  Downloading QCLang binary..." -ForegroundColor Yellow
}

foreach ($url in $downloadUrls) {
    try {
        if (-not $Silent) {
            Write-Host "   Trying: $url" -ForegroundColor Gray
        }
        
        # Download with progress
        $ProgressPreference = 'SilentlyContinue'
        Invoke-WebRequest -Uri $url -OutFile $binaryPath -ErrorAction Stop
        
        # Verify download
        if (Test-Path $binaryPath -PathType Leaf) {
            $fileSize = (Get-Item $binaryPath).Length / 1MB
            if (-not $Silent) {
                Write-Host "   ✓ Downloaded: $([math]::Round($fileSize, 2)) MB" -ForegroundColor Green
            }
            $downloadSuccess = $true
            break
        }
    } catch {
        if (-not $Silent -and $url -eq $downloadUrls[-1]) {
            Write-Host "   ✗ Failed to download from: $url" -ForegroundColor Red
            Write-Host "   Error: $($_.Exception.Message)" -ForegroundColor Red
        }
        continue
    }
}

if (-not $downloadSuccess) {
    Write-Host "`n❌ Download failed!" -ForegroundColor Red
    Write-Host "Please download manually from:" -ForegroundColor Yellow
    Write-Host "https://github.com/yourusername/qclang/releases" -ForegroundColor Cyan
    Write-Host "`nOr build from source with:" -ForegroundColor Yellow
    Write-Host "cargo install qclang-compiler" -ForegroundColor White
    exit 1
}

# Create Windows launcher script
$launcherContent = @'
@echo off
REM QCLang Windows Launcher
REM This script provides better Windows compatibility for QCLang

setlocal enabledelayedexpansion

REM Set base path
if "%QCLANG_HOME%"=="" (
    if exist "%LOCALAPPDATA%\QCLang\bin\qclang.exe" (
        set QCLANG_HOME=%LOCALAPPDATA%\QCLang
    ) else if exist "%USERPROFILE%\.qclang\bin\qclang.exe" (
        set QCLANG_HOME=%USERPROFILE%\.qclang
    ) else (
        echo ERROR: QCLang not found!
        echo.
        echo Please install QCLang using:
        echo   PowerShell -Command "iwr -useb https://qclang.dev/install.ps1 ^| iex"
        echo.
        exit /b 1
    )
)

set QCLANG_BIN=%QCLANG_HOME%\bin\qclang.exe

REM Check if binary exists
if not exist "%QCLANG_BIN%" (
    echo ERROR: qclang.exe not found at %QCLANG_BIN%
    exit /b 1
)

REM Handle special cases
if "%1"=="" (
    "%QCLANG_BIN%" --help
    exit /b %ERRORLEVEL%
)

if "%1"=="--help" (
    "%QCLANG_BIN%" --help
    exit /b %ERRORLEVEL%
)

if "%1"=="--version" (
    "%QCLANG_BIN%" --version
    exit /b %ERRORLEVEL%
)

if "%1"=="install-examples" (
    if not exist "%QCLANG_HOME%\examples" mkdir "%QCLANG_HOME%\examples"
    echo Creating examples...
    echo fn main() -> int { qubit q = ^|0^>; q = H(q); cbit r = measure(q); return 0; } > "%QCLANG_HOME%\examples\hello.qc"
    echo fn main() -> int { qubit a = ^|0^>; qubit b = ^|0^>; a = H(a); b = CNOT(a, b); cbit a_res = measure(a); cbit b_res = measure(b); return 0; } > "%QCLANG_HOME%\examples\bell.qc"
    echo Examples created at: %QCLANG_HOME%\examples
    exit /b 0
)

REM Pass all arguments to the real binary
"%QCLANG_BIN%" %*
'@

$launcherPath = "$InstallPath\bin\qclang.cmd"
Set-Content -Path $launcherPath -Value $launcherContent -Encoding ASCII

# Create PowerShell wrapper
$psWrapper = @'
#!/usr/bin/env pwsh
# QCLang PowerShell Wrapper

param(
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]]$Arguments
)

$qclangExe = Join-Path $PSScriptRoot "qclang.exe"
if (-not (Test-Path $qclangExe)) {
    Write-Error "qclang.exe not found in $PSScriptRoot"
    exit 1
}

& $qclangExe @Arguments
'@

$psWrapperPath = "$InstallPath\bin\qclang.ps1"
Set-Content -Path $psWrapperPath -Value $psWrapper -Encoding UTF8

# Add to PATH
$pathChanged = $false
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
$binPath = "$InstallPath\bin"

if ($userPath -notlike "*$binPath*") {
    if (-not $Silent) {
        Write-Host "`n🔗 Adding to user PATH..." -ForegroundColor Yellow
    }
    
    $newPath = if ($userPath.EndsWith(';')) {
        "$userPath$binPath"
    } else {
        "$userPath;$binPath"
    }
    
    [Environment]::SetEnvironmentVariable("Path", $newPath, "User")
    $pathChanged = $true
    
    if (-not $Silent) {
        Write-Host "   ✓ Added: $binPath" -ForegroundColor Green
    }
}

# Create example files
$examples = @{
    "hello.qc" = @'
// Hello Quantum - Simple superposition
fn main() -> int {
    qubit q = |0>;
    q = H(q);
    cbit result = measure(q);
    return 0;
}
'@
    "bell.qc" = @'
// Bell State - Quantum entanglement
fn main() -> int {
    qubit a = |0>;
    qubit b = |0>;
    a = H(a);
    b = CNOT(a, b);
    cbit a_res = measure(a);
    cbit b_res = measure(b);
    return 0;
}
'@
    "test.bat" = @'
@echo off
echo Testing QCLang installation...
echo.
qclang --version
echo.
echo Creating test circuit...
echo fn main() -> int { qubit q = ^|0^>; q = H(q); cbit r = measure(q); return 0; } > test.qc
echo.
echo Compiling test.qc...
qclang test.qc
echo.
if exist test.qasm (
    type test.qasm
    del test.qasm
) else (
    echo ERROR: Compilation failed!
)
del test.qc
echo.
echo Test complete!
pause
'@
}

foreach ($file in $examples.Keys) {
    $filePath = "$InstallPath\examples\$file"
    Set-Content -Path $filePath -Value $examples[$file] -Encoding UTF8
}

# Create desktop shortcut
if (-not $Silent) {
    $createShortcut = Read-Host "`nCreate desktop shortcut? (Y/N)"
    if ($createShortcut -eq 'Y' -or $createShortcut -eq 'y') {
        $shortcutPath = "$env:USERPROFILE\Desktop\QCLang Examples.lnk"
        $WshShell = New-Object -ComObject WScript.Shell
        $Shortcut = $WshShell.CreateShortcut($shortcutPath)
        $Shortcut.TargetPath = "$InstallPath\examples"
        $Shortcut.Save()
        Write-Host "   ✓ Created desktop shortcut" -ForegroundColor Green
    }
}

# Installation complete
Write-Host ""
Write-Host "═══════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "                    INSTALLATION COMPLETE!                   " -ForegroundColor Green
Write-Host "═══════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host ""

Write-Host "📦 Installation Directory:" -ForegroundColor Yellow
Write-Host "   $InstallPath" -ForegroundColor White

Write-Host "`n🚀 Quick Start:" -ForegroundColor Yellow
Write-Host "   qclang --version" -ForegroundColor White
Write-Host "   qclang `"$InstallPath\examples\hello.qc`"" -ForegroundColor White

if ($pathChanged) {
    Write-Host "`n⚠️  Note:" -ForegroundColor Yellow
    Write-Host "   Restart your terminal for PATH changes to take effect" -ForegroundColor White
    Write-Host "   or run: `$env:Path = [System.Environment]::GetEnvironmentVariable('Path','User')" -ForegroundColor Gray
}

Write-Host "`n📚 Examples:" -ForegroundColor Yellow
Write-Host "   Directory: $InstallPath\examples" -ForegroundColor White

Write-Host "`n❓ Need Help:" -ForegroundColor Yellow
Write-Host "   qclang --help" -ForegroundColor White
Write-Host "   https://github.com/yourusername/qclang" -ForegroundColor Cyan

Write-Host ""