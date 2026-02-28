vpi::startup_routines!(sim_info_startup);

#[unsafe(no_mangle)]
pub extern "C" fn sim_info_startup() {
    let sim_info = vpi::simulator_info();
    dbg!(sim_info);
}
