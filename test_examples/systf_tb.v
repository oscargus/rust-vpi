`timescale 1ns / 1ps

module systf_tb;
    integer x;
    integer y;
    reg [12:0] in13;
    reg [0:0] in1;
    wire [8:0] w9;
    reg [12:0] out13;
    reg [0:0] out1;
    reg [8:0] out9;

    assign w9 = 9'b10x1z0011;

    initial begin
        x = 41;
        $display("=== systf test started ===");
        $display("input x=%0d", x);

        // Task callback logs input and computed result inside the Rust plugin.
        $rust_log_plus_one(x);

        // Function callback returns x + 1 from Rust.
        y = $rust_add_one(x);
        $display("returned y=%0d", y);

        if (y != 42) begin
            $display("ERROR: expected y=42, got y=%0d", y);
            $finish_and_return(1);
        end

        in13 = 13'b1011001001110;
        out13 = $rust_reverse_bits(in13);
        $display("reverse13 in=%b out=%b", in13, out13);
        if (out13 !== 13'b0111001001101) begin
            $display("ERROR: expected reverse13=0111001001101, got %b", out13);
            $finish_and_return(1);
        end

        in1 = 1'b1;
        out1 = $rust_reverse_bits(in1);
        $display("reverse1 in=%b out=%b", in1, out1);
        if (out1 !== 1'b1) begin
            $display("ERROR: expected reverse1=1, got %b", out1);
            $finish_and_return(1);
        end

        out9 = $rust_reverse_bits(w9);
        $display("reverse9 in=%b out=%b", w9, out9);
        if (out9 !== 9'b1100z1x01) begin
            $display("ERROR: expected reverse9=1100z1x01, got %b", out9);
            $finish_and_return(1);
        end

        $display("PASS: systf function returned expected value");
        $finish;
    end
endmodule
