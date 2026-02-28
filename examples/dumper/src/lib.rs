vpi::startup_routines!(sim_info_startup);

#[unsafe(no_mangle)]
pub extern "C" fn sim_info_startup() {
    vpi::register_cb(vpi::CbReason::StartOfSimulation, start_of_simulation);
    vpi::register_cb(vpi::CbReason::EndOfSimulation, end_of_simulation);
}

fn start_of_simulation(_cb_data: &vpi::CbData) {
    vpi::printf("=== Simulation Started ===\n");
    walk_hierarchy(&vpi::Handle::default(), 0);
}

fn end_of_simulation(_cb_data: &vpi::CbData) {
    vpi::printf("=== Simulation Ended ===\n");
}

fn walk_hierarchy(handle: &vpi::Handle, indent: usize) {
    if !handle.is_null() {
        let name = handle
            .get_str(vpi::Property::Name)
            .unwrap_or("<unnamed>".to_string());
        vpi::printf!("{}Module: {name}\n", "  ".repeat(indent));
    }

    let children = handle.iterator(vpi::ObjectType::Module);
    for child in children {
        walk_hierarchy(&child, indent + 1);
    }
    vpi::printf!("\n{}Signals", "  ".repeat(indent + 1));
    vpi::printf!("{}=======", "  ".repeat(indent + 1));
    for signal in handle.iterator(vpi::ObjectType::Net) {
        let name = signal
            .get_str(vpi::Property::Name)
            .unwrap_or("<unnamed>".to_string());
        let signal_type = signal
            .get_str(vpi::Property::Type)
            .unwrap_or("<unknown>".to_string());
        vpi::printf!(
            "{}Signal: {name} ({signal_type})\n",
            "  ".repeat(indent + 1)
        );
        signal.register_cb(vpi::CbReason::ValueChange, move |cb_data| {
            let name = cb_data
                .obj
                .get_str(vpi::Property::Name)
                .unwrap_or("<unnamed>".to_string());
            let value = cb_data
                .obj
                .get_value(vpi::ValueType::ObjType)
                .unwrap_or(vpi::Value::String("<unknown>".to_string()));
            vpi::printf!(
                "{}Value change on signal {name}: {value}\n",
                "  ".repeat(indent + 2)
            );
        });
    }
}
