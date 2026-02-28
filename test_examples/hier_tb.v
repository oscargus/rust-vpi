`timescale 1ns/1ps

module soc_top_tb;
    reg        clk;
    reg        rst_n;
    reg        start;
    reg  [7:0] a;
    reg  [7:0] b;

    wire [7:0] y0;
    wire [7:0] y1;
    wire [7:0] mixed;
    wire       irq_and0;
    wire       irq_or0;
    wire       irq_and1;
    wire       irq_or1;

    integer cycle;
    real supply_nominal;
    time sim_start;
    event sample_event;

    soc_top dut (
        .clk(clk),
        .rst_n(rst_n),
        .start(start),
        .a(a),
        .b(b),
        .y0(y0),
        .y1(y1),
        .mixed(mixed),
        .irq_and0(irq_and0),
        .irq_or0(irq_or0),
        .irq_and1(irq_and1),
        .irq_or1(irq_or1)
    );

    always #5 clk = ~clk;

    always @(posedge clk or negedge rst_n) begin
        if (!rst_n)
            cycle <= 0;
        else
            cycle <= cycle + 1;
    end

    always @(sample_event) begin
        $display("[TB] t=%0t cycle=%0d a=%0h b=%0h y0=%0h y1=%0h mixed=%0h", $time, cycle, a, b, y0, y1, mixed);
        $display("     hier: p0.count=%0h p1.count=%0h p0.wrap=%0d p1.wrap=%0d",
                 dut.u_periph0.u_counter.count,
                 dut.u_periph1.u_counter.count,
                 dut.u_periph0.u_counter.wrap_count,
                 dut.u_periph1.u_counter.wrap_count);
    end

    initial begin
        clk = 1'b0;
        rst_n = 1'b0;
        start = 1'b0;
        a = 8'h12;
        b = 8'h34;
        cycle = 0;
        supply_nominal = 1.2;
        sim_start = $time;

        $display("=== Hierarchical Design Testbench ===");
        $display("supply_nominal = %0.2fV", supply_nominal);

        #20 rst_n = 1'b1;
        #10 start = 1'b1;

        repeat (8) begin
            @(posedge clk);
            a <= a + 8'h03;
            b <= b ^ 8'h1F;
            -> sample_event;
        end

        start = 1'b0;
        repeat (4) begin
            @(posedge clk);
            -> sample_event;
        end

        $display("Simulation runtime: %0t", $time - sim_start);
        $finish;
    end
endmodule
