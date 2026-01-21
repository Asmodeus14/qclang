<#
.SYNOPSIS
    Test QCLang installation on Windows
#>

Write-Host "Testing QCLang Installation" -ForegroundColor Cyan
Write-Host "===========================" -ForegroundColor Cyan
Write-Host ""

# Test 1: Check if qclang is in PATH
Write-Host "1. Checking PATH..." -ForegroundColor Yellow
$qclangPath = Get-Command qclang -ErrorAction SilentlyContinue
if ($qclangPath) {
    Write-Host "   ✓ Found: $($qclangPath.Source)" -ForegroundColor Green
} else {
    Write-Host "   ✗ Not found in PATH" -ForegroundColor Red
}

# Test 2: Check version
Write-Host "`n2. Checking version..." -ForegroundColor Yellow
try {
    $version = qclang --version 2>&1
    Write-Host "   ✓ $version" -ForegroundColor Green
} catch {
    Write-Host "   ✗ Failed to get version" -ForegroundColor Red
}

# Test 3: Create and compile test file
Write-Host "`n3. Testing compilation..." -ForegroundColor Yellow
$testFile = "$env:TEMP\test_qclang.qc"
@'
fn main() -> int {
    qubit q = |0>;
    q = H(q);
    cbit r = measure(q);
    return 0;
}
'@ | Out-File $testFile -Encoding UTF8

try {
    qclang $testFile 2>&1 | Out-Null
    if (Test-Path "$env:TEMP\test_qclang.qasm") {
        $qasm = Get-Content "$env:TEMP\test_qclang.qasm" -Raw
        Write-Host "   ✓ Compilation successful" -ForegroundColor Green
        Write-Host "   Generated QASM:" -ForegroundColor Gray
        $qasm -split "`n" | ForEach-Object { Write-Host "     $_" -ForegroundColor Gray }
    } else {
        Write-Host "   ✗ No output file created" -ForegroundColor Red
    }
} catch {
    Write-Host "   ✗ Compilation failed: $_" -ForegroundColor Red
}

# Cleanup
Remove-Item $testFile -ErrorAction SilentlyContinue
Remove-Item "$env:TEMP\test_qclang.qasm" -ErrorAction SilentlyContinue

Write-Host ""
Write-Host "Test complete!" -ForegroundColor Green