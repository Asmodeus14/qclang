<#
.SYNOPSIS
    Uninstall QCLang from Windows
#>

param(
    [switch]$Force,
    [switch]$Silent
)

if (-not $Silent) {
    Write-Host "QCLang Uninstaller" -ForegroundColor Cyan
    Write-Host "==================" -ForegroundColor Cyan
    Write-Host ""
}

# Possible installation locations
$locations = @(
    "$env:LOCALAPPDATA\QCLang",
    "$env:USERPROFILE\.qclang",
    "$env:ProgramFiles\QCLang"
)

$found = $false
foreach ($location in $locations) {
    if (Test-Path $location) {
        $found = $true
        
        if (-not $Silent) {
            Write-Host "Found installation at: $location" -ForegroundColor Yellow
        }
        
        if ($Force -or $Silent) {
            $confirm = 'Y'
        } else {
            $confirm = Read-Host "Remove this installation? (Y/N)"
        }
        
        if ($confirm -eq 'Y' -or $confirm -eq 'y') {
            try {
                Remove-Item -Path $location -Recurse -Force
                if (-not $Silent) {
                    Write-Host "✓ Removed: $location" -ForegroundColor Green
                }
            } catch {
                if (-not $Silent) {
                    Write-Host "✗ Error removing: $location" -ForegroundColor Red
                    Write-Host "  Error: $($_.Exception.Message)" -ForegroundColor Red
                }
            }
        }
    }
}

if (-not $found -and -not $Silent) {
    Write-Host "No QCLang installation found." -ForegroundColor Yellow
}

if (-not $Silent) {
    Write-Host ""
    Write-Host "Note: You may need to manually remove QCLang from your PATH" -ForegroundColor Gray
    Write-Host "      if you added it globally." -ForegroundColor Gray
    Write-Host ""
}