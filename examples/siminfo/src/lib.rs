vpi::startup_routines!(sim_info_startup);

#[unsafe(no_mangle)]
pub extern "C" fn sim_info_startup() {
    let sim_info = vpi::simulator_info();
    vpi::printf("=== Simulator Information ===\n");
    vpi::printf!("Simulator: {} {}\n", sim_info.product, sim_info.version);
}
