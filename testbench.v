// Simple Verilog testbench for testing VPI functionality
module testbench;
    // Clock signal
    reg clk;
    reg reset;

    // Counter for demonstration
    reg [7:0] counter;

    // Wire for testing
    wire [7:0] counter_out;

    assign counter_out = counter;

    // Clock generation - 10ns period (100MHz)
    initial begin
        clk = 0;
        forever #5 clk = ~clk;
    end

    // Reset and counter logic
    initial begin
        // Initialize signals
        reset = 1;
        counter = 0;

        // Display test start
        $display("=== VPI Testbench Started ===");
        $display("Time: %0t", $time);

        // Release reset after 20ns
        #20 reset = 0;

        // Run for some time
        #100;

        $display("=== VPI Testbench Complete ===");
        $display("Final counter value: %d", counter);
        $display("Time: %0t", $time);

        $finish;
    end

    // Counter increment on clock edge
    always @(posedge clk) begin
        if (reset)
            counter <= 0;
        else
            counter <= counter + 1;
    end

    // Monitor counter changes
    initial begin
        $monitor("Time: %0t | Reset: %b | Counter: %d", $time, reset, counter);
    end

endmodule
