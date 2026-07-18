#!/usr/bin/env pwsh
# Script to build and run the put_value VPI example on Windows

$ErrorActionPreference = "Stop"

Write-Host "=== Building put_value VPI Plugin ==="
$originalRustFlags = $env:RUSTFLAGS
$env:RUSTFLAGS = @($originalRustFlags, "-C link-arg=/FORCE:UNRESOLVED") -join " "
try {
    cargo build --release -p put_value
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
Write-Host "=== Running put_value Verilog Testbench ==="
Write-Host ""

iverilog -g2012 -o put_value_tb.vvp test_examples/put_value_dut.v

$vpiLib = Get-ChildItem target/release -File -Filter "put_value.dll" -Recurse | Select-Object -First 1
if (-not $vpiLib) {
    Write-Error "Could not find VPI shared library. Expected target/release/put_value.dll"
}

Write-Host "Found VPI library: $($vpiLib.FullName)"
Write-Host ""

$moduleDir = $vpiLib.DirectoryName
$moduleName = [System.IO.Path]::GetFileNameWithoutExtension($vpiLib.Name)

# Icarus on Windows searches for <module>.vpi.
$vpiModulePath = Join-Path $moduleDir "$moduleName.vpi"
Copy-Item -Force $vpiLib.FullName $vpiModulePath

vvp -M"$moduleDir" -m"$moduleName" put_value_tb.vvp

if ($LASTEXITCODE -ne 0) {
    Write-Error "vvp exited with code $LASTEXITCODE while loading module '$moduleName' from '$moduleDir'"
}

Write-Host ""
Write-Host "=== put_value test complete ==="
