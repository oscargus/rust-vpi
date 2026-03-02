use vpi::{
    printf, register_cb, startup_routines, CbData, CbReason, Handle, ObjectType, Property, Value,
    ValueType,
};

startup_routines!(sim_info_startup);

#[unsafe(no_mangle)]
pub extern "C" fn sim_info_startup() {
    register_cb(CbReason::StartOfSimulation, start_of_simulation);
    register_cb(CbReason::EndOfSimulation, end_of_simulation);
}

fn start_of_simulation(_cb_data: &CbData) {
    printf("=== Simulation Started ===\n");
    walk_hierarchy(&Handle::default(), 0);
}

fn end_of_simulation(_cb_data: &CbData) {
    printf("=== Simulation Ended ===\n");
}

fn value_change_cb(cb_data: &CbData) {
    let sig = &cb_data.obj;
    let name = sig
        .get_str(Property::Name)
        .unwrap_or("<unnamed>".to_string());
    let value = sig
        .get_value(ValueType::ObjType)
        .unwrap_or(Value::String("<unknown>".to_string()));
    printf!("Value change on signal {name}: {value}");
}

fn walk_hierarchy(handle: &Handle, indent: usize) {
    if !handle.is_null() {
        let name = handle
            .get_str(Property::Name)
            .unwrap_or("<unnamed>".to_string());
        printf!("{}Module: {name}", " ".repeat(indent));
    }

    let children = handle.iterator(ObjectType::Module);
    for child in children {
        walk_hierarchy(&child, indent + 1);
    }
    printf!("\n{}Signals", " ".repeat(indent + 1));
    printf!("{}=======", " ".repeat(indent + 1));
    for signal in handle.iterators(&[
        ObjectType::Net,
        ObjectType::Reg,
        ObjectType::Variables,
        ObjectType::Parameter,
    ]) {
        let name = signal
            .get_str(Property::Name)
            .unwrap_or("<unnamed>".to_string());
        let signal_type = signal
            .get_str(Property::Type)
            .unwrap_or("<unknown>".to_string());
        printf!("{}Signal: {name} ({signal_type})", " ".repeat(indent + 1));
        signal.register_cb(CbReason::ValueChange, value_change_cb);
    }

    for memory in handle.iterator(ObjectType::Memory) {
        let name = memory
            .get_str(Property::Name)
            .unwrap_or("<unnamed>".to_string());
        printf!("{}Memory: {name}", " ".repeat(indent + 1));
        for word in memory.iterator(ObjectType::MemoryWord) {
            word.register_cb(CbReason::ValueChange, value_change_cb);
        }
    }
}
