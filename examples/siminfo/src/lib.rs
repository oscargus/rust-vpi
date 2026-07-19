vpi::startup_routines!(sim_info_startup);

#[unsafe(no_mangle)]
pub extern "C" fn sim_info_startup() {
    vpi::register_cb(vpi::CbReason::EndOfSimulation, end_of_simulation);

    let sim_info = vpi::simulator_info();
    vpi::printf("=== Simulator Information ===\n");
    vpi::printf!("Simulator: {} {}\n", sim_info.product, sim_info.version);
    vpi::printf!("Command-line arguments:\n");
    for (i, arg) in sim_info.arguments.iter().enumerate() {
        vpi::printf!("  [{}] {}\n", i, arg);
    }
    vpi::printf!("Simulator name: {}\n", vpi::simulator_name());
    vpi::printf!("Simulator version: {}\n", vpi::simulator_version());
}

fn end_of_simulation(_cb_data: &vpi::CbData) {
    vpi::printf!(
        "End of simulation time: {}\n",
        vpi::current_simulation_time()
    );
    if let Some(error) = vpi::check_error() {
        vpi::printf!("Last error: {}\n", error);
    } else {
        vpi::printf!("No errors reported.\n");
    }
}
