#!/usr/bin/env pwsh
# Script to build and run the error_test VPI example

$ErrorActionPreference = 'Stop'

$BuildProfile = if ($env:CARGO_BUILD_PROFILE) { $env:CARGO_BUILD_PROFILE } else { 'release' }

$CargoBuildArgs = @()
$TargetDir = "target/debug"
switch ($BuildProfile) {
    'release' {
        $CargoBuildArgs = @('--release')
        $TargetDir = "target/release"
    }
    'debug' {
        # default, no args needed
    }
    default {
        $CargoBuildArgs = @('--profile', $BuildProfile)
        $TargetDir = "target/$BuildProfile"
    }
}

Write-Host "=== Building error_test VPI Plugin ===" -ForegroundColor Cyan
cargo build @CargoBuildArgs -p error_test

Write-Host ""
Write-Host "=== Compiling Verilog Testbench ===" -ForegroundColor Cyan
& iverilog -g2012 -o test_examples/error_test_tb.vvp test_examples/error_test_tb.v test_examples/error_test_dut.v

Write-Host ""
Write-Host "=== Finding VPI Library ===" -ForegroundColor Cyan
$VPILib = Get-ChildItem $TargetDir -Filter "error_test.dll" -ErrorAction SilentlyContinue | Select-Object -First 1

if (-not $VPILib) {
    # Try alternative names
    $VPILib = Get-ChildItem $TargetDir -Filter "liberror_test.*" -ErrorAction SilentlyContinue | Select-Object -First 1
}

if (-not $VPILib) {
    Write-Host "Error: Could not find VPI shared library" -ForegroundColor Red
    Write-Host "Files in ${TargetDir}:"
    Get-ChildItem $TargetDir -Filter "*error_test*" -ErrorAction SilentlyContinue | Select-Object Name
    exit 1
}

Write-Host "Found VPI library: $($VPILib.FullName)"
Write-Host ""

Write-Host "=== Running error_test Testbench ===" -ForegroundColor Cyan
$ModuleFile = Split-Path -Leaf $VPILib.FullName
$ModuleDir = Split-Path -Parent $VPILib.FullName

if ($ModuleFile -match '\.dll$') {
    $ModuleName = $ModuleFile -replace '\.dll$', ''
    Copy-Item $VPILib.FullName "$ModuleDir\$ModuleName.vpi" -Force
    & vvp -M"$ModuleDir" -m"$ModuleName" test_examples/error_test_tb.vvp
} elseif ($ModuleFile -match '\.vpi$') {
    $ModuleName = $ModuleFile -replace '\.vpi$', ''
    & vvp -M"$ModuleDir" -m"$ModuleName" test_examples/error_test_tb.vvp
} else {
    & vvp -M. -m"$ModuleFile" test_examples/error_test_tb.vvp
}

Write-Host ""
Write-Host "=== error_test Test Complete ===" -ForegroundColor Cyan
