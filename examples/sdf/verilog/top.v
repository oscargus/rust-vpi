`timescale 1ns/1ps

// Two trivial cells; the interesting delays live on the wires between them.
module drv(input a, output y);
  buf g(y, a);
endmodule

module rcv(input d, output q);
  buf g(q, d);
endmodule

module top;
  reg  in = 1'b0;
  wire n1, n2, out;

  drv u_drv  (.a(in), .y(n1));   // path 1: u_drv.y  -> u_rcv.d
  rcv u_rcv  (.d(n1), .q(n2));   // path 2: u_rcv.q  -> u_rcv2.d
  rcv u_rcv2 (.d(n2), .q(out));

  initial begin
    $sdf_annotate("test.sdf", top);
    #1  $intermod_test;          // VPI: get / negative / put / readback
    #9  in = 1'b1;               // t=10 : rising edge
    #20 in = 1'b0;               // t=30 : falling edge
    #30 $finish;
  end

  // Observe the delay effect at the receiving ports
  always @(u_rcv.d)  $display("t=%7.3f ns  u_rcv.d  = %b", $realtime, u_rcv.d);
  always @(u_rcv2.d) $display("t=%7.3f ns  u_rcv2.d = %b", $realtime, u_rcv2.d);
endmodule
