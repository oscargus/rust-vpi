use std::ffi::CStr;
use std::os::raw::c_char;

use vpi::{
    current_systf_call, get_systf_arg, register_systf, startup_routines, ObjectType, SysFuncType,
    SystfKind, Value, ValueType,
};

startup_routines!(systf_startup);

static TASK_NAME: &CStr = c"$rust_log_plus_one";
static FUNC_NAME: &CStr = c"$rust_add_one";
static REVERSE_FUNC_NAME: &CStr = c"$rust_reverse_bits";

#[unsafe(no_mangle)]
pub extern "C" fn systf_startup() {
    let _ = register_systf(
        SystfKind::Task,
        TASK_NAME,
        Some(calltf_log_plus_one),
        Some(compiletf_one_int_arg),
        None,
        std::ptr::null_mut(),
        None,
    );

    let _ = register_systf(
        SystfKind::Func,
        FUNC_NAME,
        Some(calltf_add_one),
        Some(compiletf_one_int_arg),
        None,
        std::ptr::null_mut(),
        Some(SysFuncType::Sized),
    );

    let _ = register_systf(
        SystfKind::Func,
        REVERSE_FUNC_NAME,
        Some(calltf_reverse_bits),
        Some(compiletf_one_int_arg),
        Some(sizetf_reverse_bits),
        std::ptr::null_mut(),
        Some(SysFuncType::Sized),
    );

    vpi::printf("Registered $rust_log_plus_one, $rust_add_one, and $rust_reverse_bits");
}

unsafe extern "C" fn compiletf_one_int_arg(_user_data: *mut c_char) -> i32 {
    // Keep compile-time checks minimal for simulator compatibility in this example.
    0
}

unsafe extern "C" fn calltf_log_plus_one(_user_data: *mut c_char) -> i32 {
    let arg = match get_systf_arg(0, ValueType::Int) {
        Some(Value::Int(v)) => v,
        _ => 0,
    };
    let result = arg + 1;
    vpi::printf!("$rust_log_plus_one arg={} result={}", arg, result);
    0
}

unsafe extern "C" fn calltf_add_one(_user_data: *mut c_char) -> i32 {
    let arg = match get_systf_arg(0, ValueType::Int) {
        Some(Value::Int(v)) => v,
        _ => 0,
    };
    let result = arg + 1;

    let call = current_systf_call();
    let _ = call.put_value(&Value::Int(result));

    vpi::printf!("$rust_add_one arg={} result={}", arg, result);
    0
}

unsafe extern "C" fn sizetf_reverse_bits(_user_data: *mut c_char) -> i32 {
    let call = current_systf_call();
    if call.is_null() {
        return 0;
    }

    let mut args = call.iterator(ObjectType::Argument);
    let Some(first_arg) = args.next() else {
        return 0;
    };

    i32::try_from(first_arg.get_u32(vpi::Property::Size).unwrap_or(0)).unwrap_or(0)
}

unsafe extern "C" fn calltf_reverse_bits(_user_data: *mut c_char) -> i32 {
    let Some(Value::BinStr(bits)) = get_systf_arg(0, ValueType::BinStr) else {
        return 0;
    };

    let reversed: String = bits.chars().rev().collect();
    let Some(reversed_vec) = vpi::string_to_scalar_vector(&reversed) else {
        return 0;
    };

    let call = current_systf_call();
    let _ = call.put_value(&Value::Vector(reversed_vec));

    vpi::printf!("$rust_reverse_bits arg={} result={}", bits, reversed);
    0
}
