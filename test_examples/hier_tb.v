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
    integer transaction_count;
    integer error_count;
    integer data_sum;
    integer max_value;
    integer min_value;
    integer histogram[0:15];  // Array of integers - histogram buckets
    integer fifo_buffer[0:7]; // Array of integers - FIFO buffer
    integer fifo_head;
    integer fifo_tail;
    real supply_nominal;
    time sim_start;
    event sample_event;

    // Array of wires for multi-channel data
    wire [7:0] channels [0:3];  // 4 channels of 8-bit data
    wire       status_bits [0:7]; // 8 individual status bits

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

    // Connect wire arrays - channels derived from outputs
    assign channels[0] = y0;
    assign channels[1] = y1;
    assign channels[2] = mixed;
    assign channels[3] = y0 ^ y1;

    // Connect status bits array
    assign status_bits[0] = irq_and0;
    assign status_bits[1] = irq_or0;
    assign status_bits[2] = irq_and1;
    assign status_bits[3] = irq_or1;
    assign status_bits[4] = start;
    assign status_bits[5] = clk;
    assign status_bits[6] = rst_n;
    assign status_bits[7] = (y0 == y1);

    always #5 clk = ~clk;

    always @(posedge clk or negedge rst_n) begin
        integer i;
        if (!rst_n) begin
            cycle <= 0;
            transaction_count <= 0;
            error_count <= 0;
            data_sum <= 0;
            max_value <= 0;
            min_value <= 32'h7FFFFFFF;
            fifo_head <= 0;
            fifo_tail <= 0;

            // Initialize histogram and FIFO arrays
            for (i = 0; i < 16; i = i + 1)
                histogram[i] <= 0;
            for (i = 0; i < 8; i = i + 1)
                fifo_buffer[i] <= 0;
        end else begin
            cycle <= cycle + 1;

            if (start) begin
                transaction_count <= transaction_count + 1;
                data_sum <= data_sum + y0 + y1;

                if (y0 > max_value)
                    max_value <= y0;
                if (y0 < min_value && y0 != 0)
                    min_value <= y0;

                if (y0 === 8'hXX || y1 === 8'hXX)
                    error_count <= error_count + 1;

                // Update histogram - count occurrences in each nibble range
                if (y0[3:0] < 16)
                    histogram[y0[3:0]] <= histogram[y0[3:0]] + 1;

                // FIFO buffer - store recent y0 values
                fifo_buffer[fifo_head] <= y0;
                fifo_head <= (fifo_head + 1) % 8;
                if (fifo_head == fifo_tail)
                    fifo_tail <= (fifo_tail + 1) % 8;
            end
        end
    end

    always @(sample_event) begin
        $display("[TB] t=%0t cycle=%0d a=%0h b=%0h y0=%0h y1=%0h mixed=%0h", $time, cycle, a, b, y0, y1, mixed);
        $display("     hier: p0.count=%0h p1.count=%0h p0.wrap=%0d p1.wrap=%0d",
                 dut.u_periph0.u_counter.count,
                 dut.u_periph1.u_counter.count,
                 dut.u_periph0.u_counter.wrap_count,
                 dut.u_periph1.u_counter.wrap_count);
        $display("     stats: txns=%0d errors=%0d sum=%0d max=%0d min=%0d",
                 transaction_count, error_count, data_sum, max_value, min_value);
        $display("     channels: ch0=%0h ch1=%0h ch2=%0h ch3=%0h",
                 channels[0], channels[1], channels[2], channels[3]);
        $display("     fifo: head=%0d tail=%0d [0]=%0d [1]=%0d [2]=%0d",
                 fifo_head, fifo_tail, fifo_buffer[0], fifo_buffer[1], fifo_buffer[2]);
    end

    initial begin
        integer i;
        clk = 1'b0;
        rst_n = 1'b0;
        start = 1'b0;
        a = 8'h12;
        b = 8'h34;
        cycle = 0;
        transaction_count = 0;
        error_count = 0;
        data_sum = 0;
        max_value = 0;
        min_value = 32'h7FFFFFFF;
        fifo_head = 0;
        fifo_tail = 0;
        supply_nominal = 1.2;
        sim_start = $time;

        // Initialize arrays
        for (i = 0; i < 16; i = i + 1)
            histogram[i] = 0;
        for (i = 0; i < 8; i = i + 1)
            fifo_buffer[i] = 0;

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

        $display("");
        $display("=== Final Statistics ===");
        $display("Total cycles:       %0d", cycle);
        $display("Total transactions: %0d", transaction_count);
        $display("Total errors:       %0d", error_count);
        $display("Data sum:           %0d", data_sum);
        $display("Max value seen:     %0d", max_value);
        $display("Min value seen:     %0d", min_value);
        $display("");
        $display("Histogram (nibble counts):");
        for (i = 0; i < 16; i = i + 1) begin
            if (histogram[i] > 0)
                $display("  [%0h]: %0d", i, histogram[i]);
        end
        $display("");
        $display("FIFO Buffer Contents:");
        for (i = 0; i < 8; i = i + 1)
            $display("  fifo[%0d] = %0d", i, fifo_buffer[i]);
        $display("");
        $display("Wire Arrays:");
        $display("  channels[0-3]: %0h %0h %0h %0h", channels[0], channels[1], channels[2], channels[3]);
        $display("  status_bits: %b%b%b%b%b%b%b%b",
                 status_bits[7], status_bits[6], status_bits[5], status_bits[4],
                 status_bits[3], status_bits[2], status_bits[1], status_bits[0]);
        $display("");
        $display("Simulation runtime: %0t", $time - sim_start);
        $finish;
    end
endmodule
