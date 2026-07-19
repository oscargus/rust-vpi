`timescale 1ns / 1ps

module put_value_dut;
    reg bit_in;
    reg [7:0] vec_in;
    integer int_in;
    reg [1:0] arr_in [0:1];
    wire bit_out;
    wire [7:0] vec_out;
    wire [31:0] int_out;
    wire [1:0] arr_out [0:1];

    initial begin
        bit_in = 1'b0;
        vec_in = 8'h00;
        int_in = 0;
        arr_in[0] = 2'b00;
        arr_in[1] = 2'b00;
    end

    assign bit_out = ~bit_in;
    assign vec_out = ~vec_in;
    assign int_out = int_in + 1;
    assign arr_out[0] = ~arr_in[0];
    assign arr_out[1] = ~arr_in[1];

endmodule
