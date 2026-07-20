`timescale 1ns / 1ps

module error_test_dut;
    reg [31:0] test_value;

    initial begin
        $display("=== Error Test DUT Started ===");

        // This should execute successfully
        test_value = 42;
        $display("Test value set to: %d", test_value);

        // Trigger a runtime error using $error
        $display("Triggering runtime error...");
        $error("Intentional test error from Verilog simulation");

        $finish;
    end

endmodule
