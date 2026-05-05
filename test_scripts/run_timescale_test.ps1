#!/usr/bin/env pwsh
# Script to test timescale VPI functionality on Windows

$ErrorActionPreference = "Stop"

Write-Host "=== Building Timescale VPI Plugin ==="
$originalRustFlags = $env:RUSTFLAGS
$env:RUSTFLAGS = @($originalRustFlags, "-C link-arg=/FORCE:UNRESOLVED") -join " "
try {
    cargo build --release -p timescale
}
finally {
    if ([string]::IsNullOrWhiteSpace($originalRustFlags)) {
        Remove-Item Env:RUSTFLAGS -ErrorAction SilentlyContinue
    }
    else {
        $env:RUSTFLAGS = $originalRustFlags
    }
}

Write-Host ""
Write-Host "=== Running Verilog Testbench with Timescale VPI ==="
Write-Host ""

# Compile Verilog testbench if not already compiled
if (-not (Test-Path testbench.vvp)) {
    iverilog -o testbench.vvp test_examples/testbench.v
}

# Find the shared library
$vpiLib = Get-ChildItem target/release -File -Filter "timescale.dll" -Recurse | Select-Object -First 1

if (-not $vpiLib) {
    Write-Error "Could not find VPI shared library. Expected target/release/timescale.dll"
}

Write-Host "Found VPI library: $($vpiLib.FullName)"
Write-Host ""

# Run simulation with VPI plugin loaded
$moduleDir = $vpiLib.DirectoryName
$moduleName = [System.IO.Path]::GetFileNameWithoutExtension($vpiLib.Name)
vvp -M"$moduleDir" -m"$moduleName" testbench.vvp

if ($LASTEXITCODE -ne 0) {
    Write-Error "vvp exited with code $LASTEXITCODE while loading module '$moduleName' from '$moduleDir'"
}

Write-Host ""
Write-Host "=== Test Complete ==="
