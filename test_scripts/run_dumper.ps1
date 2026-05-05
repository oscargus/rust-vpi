#!/usr/bin/env pwsh
# Script to build and run the hierarchical Verilog testbench with dumper VPI plugin on Windows

$ErrorActionPreference = "Stop"

Write-Host "=== Building Dumper VPI Plugin ==="
$originalRustFlags = $env:RUSTFLAGS
$env:RUSTFLAGS = @($originalRustFlags, "-C link-arg=/FORCE:UNRESOLVED") -join " "
try {
    cargo build --release -p dumper
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
Write-Host "=== Compiling Hierarchical Verilog Testbench ==="
Write-Host "Using Icarus Verilog (iverilog/vvp)"
Write-Host ""

# Compile hierarchical Verilog design and testbench
iverilog -g2012 -o test_examples/hier_tb.vvp test_examples/hier_design.v test_examples/hier_tb.v

Write-Host "Compilation successful"
Write-Host ""

# Find the shared library
$vpiLib = Get-ChildItem target/release -File -Filter "dumper.dll" -Recurse | Select-Object -First 1

if (-not $vpiLib) {
    Write-Error "Could not find VPI shared library. Expected target/release/dumper.dll"
}

Write-Host "Found VPI library: $($vpiLib.FullName)"
Write-Host ""
Write-Host "=== Running Simulation with Dumper VPI Plugin ==="
Write-Host ""

# Run simulation with VPI plugin loaded
$moduleDir = $vpiLib.DirectoryName
$moduleName = [System.IO.Path]::GetFileNameWithoutExtension($vpiLib.Name)
vvp -M"$moduleDir" -m"$moduleName" test_examples/hier_tb.vvp

if ($LASTEXITCODE -ne 0) {
    Write-Error "vvp exited with code $LASTEXITCODE while loading module '$moduleName' from '$moduleDir'"
}

Write-Host ""
Write-Host "=== Simulation Complete ==="
