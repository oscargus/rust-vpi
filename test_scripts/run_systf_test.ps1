#!/usr/bin/env pwsh
# Script to build and run the systf VPI example on Windows

$ErrorActionPreference = "Stop"

Write-Host "=== Building systf VPI Plugin ==="
$originalRustFlags = $env:RUSTFLAGS
$env:RUSTFLAGS = @($originalRustFlags, "-C link-arg=/FORCE:UNRESOLVED") -join " "
try {
    cargo build --release -p systf
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
Write-Host "=== Running systf Verilog Testbench ==="
Write-Host ""

iverilog -o systf_tb.vvp test_examples/systf_tb.v

$vpiLib = Get-ChildItem target/release -File -Filter "systf.dll" -Recurse | Select-Object -First 1
if (-not $vpiLib) {
    Write-Error "Could not find VPI shared library. Expected target/release/systf.dll"
}

Write-Host "Found VPI library: $($vpiLib.FullName)"
Write-Host ""

$moduleDir = $vpiLib.DirectoryName
$moduleName = [System.IO.Path]::GetFileNameWithoutExtension($vpiLib.Name)

# Icarus on Windows searches for <module>.vpi. Mirror the built DLL to that name.
$vpiModulePath = Join-Path $moduleDir "$moduleName.vpi"
Copy-Item -Force $vpiLib.FullName $vpiModulePath

vvp -M"$moduleDir" -m"$moduleName" systf_tb.vvp

if ($LASTEXITCODE -ne 0) {
    Write-Error "vvp exited with code $LASTEXITCODE while loading module '$moduleName' from '$moduleDir'"
}

Write-Host ""
Write-Host "=== systf test complete ==="
