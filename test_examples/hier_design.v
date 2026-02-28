`timescale 1ns/1ps

module counter8 (
    input  wire       clk,
    input  wire       rst_n,
    input  wire       en,
    output wire [7:0] count_out
);
    reg [7:0] count;
    integer wrap_count;
    time last_tick;

    assign count_out = count;

    always @(posedge clk or negedge rst_n) begin
        if (!rst_n) begin
            count <= 8'd0;
            wrap_count <= 0;
            last_tick <= 0;
        end else if (en) begin
            last_tick <= $time;
            if (count == 8'hFF)
                wrap_count <= wrap_count + 1;
            count <= count + 8'd1;
        end
    end
endmodule

module alu8 (
    input  wire [7:0] a,
    input  wire [7:0] b,
    input  wire [1:0] op,
    output reg  [7:0] y
);
    real gain;

    always @* begin
        gain = 1.0;
        case (op)
            2'b00: y = a + b;
            2'b01: y = a - b;
            2'b10: y = a ^ b;
            default: y = a & b;
        endcase
    end
endmodule

module peripheral (
    input  wire       clk,
    input  wire       rst_n,
    input  wire [7:0] in_a,
    input  wire [7:0] in_b,
    input  wire       start,
    input  wire       irq_a,
    input  wire       irq_b,
    output wire [7:0] data_out,
    output wire       irq_and,
    output wire       irq_or
);
    wire [7:0] count;
    wire [7:0] alu_y;
    reg  [1:0] op_sel;
    reg        drive_enable;
    reg        force_bus;

    tri  [7:0] tri_bus;
    wand       and_bus;
    wor        or_bus;

    counter8 u_counter (
        .clk(clk),
        .rst_n(rst_n),
        .en(start),
        .count_out(count)
    );

    alu8 u_alu (
        .a(in_a),
        .b(in_b),
        .op(op_sel),
        .y(alu_y)
    );

    always @(posedge clk or negedge rst_n) begin
        if (!rst_n)
            op_sel <= 2'b00;
        else
            op_sel <= op_sel + 2'b01;
    end

    always @* begin
        drive_enable = start;
        force_bus = in_a[0];
    end

    assign tri_bus = drive_enable ? count : 8'bz;
    assign tri_bus = force_bus ? 8'hA5 : 8'bz;

    assign and_bus = irq_a;
    assign and_bus = irq_b;
    assign or_bus = irq_a;
    assign or_bus = irq_b;

    assign irq_and = and_bus;
    assign irq_or  = or_bus;

    assign data_out = (tri_bus === 8'bz) ? alu_y : tri_bus;
endmodule

module soc_top (
    input  wire       clk,
    input  wire       rst_n,
    input  wire       start,
    input  wire [7:0] a,
    input  wire [7:0] b,
    output wire [7:0] y0,
    output wire [7:0] y1,
    output wire [7:0] mixed,
    output wire       irq_and0,
    output wire       irq_or0,
    output wire       irq_and1,
    output wire       irq_or1
);
    supply1 vdd;
    supply0 vss;

    wire [7:0] a_inv;
    assign a_inv = ~a;

    peripheral u_periph0 (
        .clk(clk),
        .rst_n(rst_n),
        .in_a(a),
        .in_b(b),
        .start(start),
        .irq_a(vdd),
        .irq_b(vss),
        .data_out(y0),
        .irq_and(irq_and0),
        .irq_or(irq_or0)
    );

    peripheral u_periph1 (
        .clk(clk),
        .rst_n(rst_n),
        .in_a(a_inv),
        .in_b(y0),
        .start(start),
        .irq_a(irq_or0),
        .irq_b(vdd),
        .data_out(y1),
        .irq_and(irq_and1),
        .irq_or(irq_or1)
    );

    assign mixed = y0 ^ y1;
endmodule
