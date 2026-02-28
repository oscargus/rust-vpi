vpi::startup_routines!(timescale_startup);

#[unsafe(no_mangle)]
pub extern "C" fn timescale_startup() {
    vpi::register_cb(vpi::CbReason::StartOfSimulation, timescale_callback);
}

fn timescale_callback(_cb_data: &vpi::CbData) {
    println!("=== Timescale Information ===");

    // Get simulator info
    let sim_info = vpi::simulator_info();
    println!("Simulator: {} {}", sim_info.product, sim_info.version);
    println!();

    // Get timescale for all top-level modules
    let timescales = vpi::get_top_module_timescales();

    if timescales.is_empty() {
        println!("No modules found");
    } else {
        println!("Module timescales:");
        for (module_name, timescale) in timescales {
            match timescale {
                Some(ts) => {
                    println!("  {} : {}", module_name, ts);
                    println!("    Unit: {} (10^{} s)", ts.unit_str(), ts.unit);
                    println!(
                        "    Precision: {} (10^{} s)",
                        ts.precision_str(),
                        ts.precision
                    );
                }
                None => {
                    println!("  {} : No timescale defined", module_name);
                }
            }
        }
    }

    println!();
}
