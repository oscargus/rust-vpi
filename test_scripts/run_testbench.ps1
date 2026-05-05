#!/usr/bin/env pwsh
# Script to build and run the Verilog testbench with VPI plugin on Windows

$ErrorActionPreference = "Stop"

Write-Host "=== Building VPI Plugin ==="
$originalRustFlags = $env:RUSTFLAGS
$env:RUSTFLAGS = @($originalRustFlags, "-C link-arg=/FORCE:UNRESOLVED") -join " "
try {
    cargo build --release -p siminfo
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
Write-Host "=== Running Verilog Testbench with VPI ==="
Write-Host "Using Icarus Verilog (iverilog/vvp)"
Write-Host ""

# Compile Verilog testbench
iverilog -o testbench.vvp test_examples/testbench.v

# Find the shared library
$vpiLib = Get-ChildItem target/release -File -Filter "siminfo.dll" -Recurse | Select-Object -First 1

if (-not $vpiLib) {
    Write-Error "Could not find VPI shared library. Expected target/release/siminfo.dll"
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
Write-Host "=== Simulation Complete ==="
