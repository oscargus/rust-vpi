# error_test VPI Example

This example demonstrates how to detect and capture runtime errors from Verilog simulations using VPI callbacks.

## Overview

The `error_test` example consists of:

- **error_test_dut.v**: A simple DUT that intentionally performs a division by zero operation to trigger a runtime error
- **error_test_tb.v**: A testbench that instantiates the DUT
- **src/lib.rs**: A VPI plugin that registers error and PLI error callbacks to detect and log runtime errors

## How It Works

1. The Verilog DUT performs a safe division first to establish normal operation
2. It then attempts a division by zero, which triggers a simulator runtime error
3. The VPI plugin has registered callbacks for:
   - `CbReason::Error`: Catches general simulation errors
   - `CbReason::PLIError`: Catches PLI-level errors
   - `CbReason::EndOfSimulation`: Reports error statistics at simulation end

## Error Detection

The VPI plugin maintains:
- An atomic counter of errors encountered
- A thread-safe vector of error messages
- Detailed logging to stderr for debugging

When the simulation ends, the plugin reports:
- Total number of errors captured
- Details of each error with timestamp information
- Success/warning message about error detection

## Running the Test

### On Unix/Linux/macOS:
```bash
./test_scripts/run_error_test.sh
```

### On Windows (PowerShell):
```powershell
.\test_scripts\run_error_test.ps1
```

## Expected Output

You should see:
1. Error callback registration messages
2. Safe division output from the DUT
3. Error messages indicating division by zero was attempted
4. End-of-simulation report showing error was captured via VPI callbacks

This example is useful for:
- Understanding VPI error callback mechanisms
- Implementing error detection in Verilog simulations
- Testing how VPI plugins can intercept and log simulation errors
- Verifying error handling in VPI-based test infrastructure
