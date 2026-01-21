<#
.SYNOPSIS
    Build QCLang for Windows
.DESCRIPTION
    Cross-compile QCLang from WSL/Linux or build natively on Windows
#>

param(
    [switch]$Native,    # Build natively (requires Rust on Windows)
    [switch]$Release,
    [string]$Target = "x86_64-pc-windows-gnu"
)

$ErrorActionPreference = "Stop"

# Check if we're in WSL or Windows
$isWindows = $env:OS -eq "Windows_NT"

if ($Native -or $isWindows) {
    # Native Windows build
    Write-Host "Building QCLang natively on Windows..." -ForegroundColor Yellow
    
    # Check for Rust
    if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
        Write-Host "ERROR: Rust/Cargo not found!" -ForegroundColor Red
        Write-Host "Install Rust from: https://rustup.rs/" -ForegroundColor Cyan
        exit 1
    }
    
    # Build
    $buildArgs = @("build")
    if ($Release) { $buildArgs += "--release" }
    
    Write-Host "Running: cargo $($buildArgs -join ' ')" -ForegroundColor Gray
    cargo @buildArgs
    
    # Copy to windows directory
    $source = if ($Release) { "target\release\qclang.exe" } else { "target\debug\qclang.exe" }
    $dest = "..\windows\dist\qclang-windows.exe"
    
    if (Test-Path $source) {
        Copy-Item $source $dest -Force
        Write-Host "✓ Built: $dest" -ForegroundColor Green
    }
} else {
    # Cross-compile from WSL/Linux
    Write-Host "Cross-compiling for Windows from WSL..." -ForegroundColor Yellow
    
    # Check for mingw
    if (-not (Get-Command x86_64-w64-mingw32-gcc -ErrorAction SilentlyContinue)) {
        Write-Host "Installing mingw-w64..." -ForegroundColor Yellow
        sudo apt-get update
        sudo apt-get install -y gcc-mingw-w64
    }
    
    # Add Windows target
    Write-Host "Adding Windows target..." -ForegroundColor Gray
    rustup target add $Target
    
    # Build
    $buildArgs = @("build", "--target", $Target)
    if ($Release) { $buildArgs += "--release" }
    
    Write-Host "Running: cargo $($buildArgs -join ' ')" -ForegroundColor Gray
    cargo @buildArgs
    
    # Copy to windows directory (assuming WSL mount at /mnt/c)
    $source = if ($Release) { 
        "target/$Target/release/qclang.exe" 
    } else { 
        "target/$Target/debug/qclang.exe" 
    }
    
    $dest = "/mnt/c/CODE/qclang/windows/dist/qclang-windows.exe"
    
    if (Test-Path $source) {
        Copy-Item $source $dest -Force
        Write-Host "✓ Built: $dest" -ForegroundColor Green
    } else {
        Write-Host "ERROR: Build failed or binary not found at: $source" -ForegroundColor Red
    }
}