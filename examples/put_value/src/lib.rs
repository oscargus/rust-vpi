use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use vpi::{
    control, register_cb, register_cb_with_time, scalar_vector_to_string, startup_routines,
    string_to_scalar_vector, CbData, CbReason, Control, Handle, ScalarValue, Time, Value,
    ValueType,
};

startup_routines!(put_value_startup);

const BIT_IN: &str = "put_value_dut.bit_in";
const VEC_IN: &str = "put_value_dut.vec_in";
const INT_IN: &str = "put_value_dut.int_in";
const BIT_OUT: &str = "put_value_dut.bit_out";
const VEC_OUT: &str = "put_value_dut.vec_out";
const INT_OUT: &str = "put_value_dut.int_out";
const ARR_IN_0: &str = "put_value_dut.arr_in[0]";
const ARR_IN_1: &str = "put_value_dut.arr_in[1]";
const ARR_OUT_0: &str = "put_value_dut.arr_out[0]";
const ARR_OUT_1: &str = "put_value_dut.arr_out[1]";

#[derive(Copy, Clone)]
struct TestCase {
    name: &'static str,
    bit_in: ScalarValue,
    vec_in: &'static str,
    int_in: i32,
    arr_in: [&'static str; 2],
    verify_delay: u64,
    inter_test_delay: u64,
}

struct DutHandles {
    bit_in: Handle,
    vec_in: Handle,
    int_in: Handle,
    bit_out: Handle,
    vec_out: Handle,
    int_out: Handle,
    arr_in_0: Handle,
    arr_in_1: Handle,
    arr_out_0: Handle,
    arr_out_1: Handle,
}

const TESTS: [TestCase; 5] = [
    TestCase {
        name: "ones_and_alternating_a5",
        bit_in: ScalarValue::One,
        vec_in: "10100101",
        int_in: 41,
        arr_in: ["10", "01"],
        verify_delay: 1,
        inter_test_delay: 2,
    },
    TestCase {
        name: "zeros_and_all_zero",
        bit_in: ScalarValue::Zero,
        vec_in: "00000000",
        int_in: 0,
        arr_in: ["00", "00"],
        verify_delay: 2,
        inter_test_delay: 3,
    },
    TestCase {
        name: "mixed_3c_and_negative",
        bit_in: ScalarValue::One,
        vec_in: "00111100",
        int_in: -7,
        arr_in: ["11", "10"],
        verify_delay: 1,
        inter_test_delay: 2,
    },
    TestCase {
        name: "mixed_f0_and_large_positive",
        bit_in: ScalarValue::Zero,
        vec_in: "11110000",
        int_in: 12345,
        arr_in: ["01", "11"],
        verify_delay: 3,
        inter_test_delay: 1,
    },
    TestCase {
        name: "mixed_xzhl_states",
        bit_in: ScalarValue::One,
        vec_in: "XZHL10ZH",
        int_in: 99,
        arr_in: ["ZH", "XL"],
        verify_delay: 2,
        inter_test_delay: 1,
    },
];

static CURRENT_TEST_INDEX: AtomicUsize = AtomicUsize::new(0);
static HAD_FAILURE: AtomicBool = AtomicBool::new(false);

#[unsafe(no_mangle)]
pub extern "C" fn put_value_startup() {
    let _ = register_cb(CbReason::StartOfSimulation, start_of_simulation);
}

fn start_of_simulation(_cb_data: &CbData) {
    CURRENT_TEST_INDEX.store(0, Ordering::SeqCst);
    HAD_FAILURE.store(false, Ordering::SeqCst);

    vpi::printf!("=== put_value test suite start ({} cases) ===", TESTS.len());

    let _ = register_cb_with_time(CbReason::AfterDelay, Time::Sim(1), run_next_test);
}

fn resolve_dut_handles() -> Option<DutHandles> {
    let handles = DutHandles {
        bit_in: Handle::handle_by_name(BIT_IN),
        vec_in: Handle::handle_by_name(VEC_IN),
        int_in: Handle::handle_by_name(INT_IN),
        bit_out: Handle::handle_by_name(BIT_OUT),
        vec_out: Handle::handle_by_name(VEC_OUT),
        int_out: Handle::handle_by_name(INT_OUT),
        arr_in_0: Handle::handle_by_name(ARR_IN_0),
        arr_in_1: Handle::handle_by_name(ARR_IN_1),
        arr_out_0: Handle::handle_by_name(ARR_OUT_0),
        arr_out_1: Handle::handle_by_name(ARR_OUT_1),
    };

    if [
        handles.bit_in.is_null(),
        handles.vec_in.is_null(),
        handles.int_in.is_null(),
        handles.bit_out.is_null(),
        handles.vec_out.is_null(),
        handles.int_out.is_null(),
        handles.arr_in_0.is_null(),
        handles.arr_in_1.is_null(),
        handles.arr_out_0.is_null(),
        handles.arr_out_1.is_null(),
    ]
    .into_iter()
    .any(|x| x)
    {
        None
    } else {
        Some(handles)
    }
}

fn invert_binary_scalar(value: ScalarValue) -> ScalarValue {
    match value {
        ScalarValue::Zero => ScalarValue::One,
        ScalarValue::One => ScalarValue::Zero,
        _ => ScalarValue::X,
    }
}

fn normalize_vector_roundtrip_scalar(value: ScalarValue) -> ScalarValue {
    match value {
        ScalarValue::H => ScalarValue::One,
        ScalarValue::L => ScalarValue::Zero,
        ScalarValue::DontCare => ScalarValue::X,
        other => other,
    }
}

fn normalized_scalar_string(bits: &str) -> Option<String> {
    let values = string_to_scalar_vector(bits)?;
    let values: Vec<ScalarValue> = values
        .iter()
        .copied()
        .map(normalize_vector_roundtrip_scalar)
        .collect();
    Some(scalar_vector_to_string(&values))
}

fn inverted_scalar_string(bits: &str) -> Option<String> {
    let values = string_to_scalar_vector(bits)?;
    let values: Vec<ScalarValue> = values
        .iter()
        .copied()
        .map(normalize_vector_roundtrip_scalar)
        .map(invert_binary_scalar)
        .collect();
    Some(scalar_vector_to_string(&values))
}

fn run_next_test(_cb_data: &CbData) {
    let index = CURRENT_TEST_INDEX.load(Ordering::SeqCst);

    if index >= TESTS.len() {
        if HAD_FAILURE.load(Ordering::SeqCst) {
            vpi::printf("FAIL: put_value test suite failed");
        } else {
            vpi::printf("PASS: put_value test suite passed");
        }
        control(Control::Finish);
        return;
    }

    let Some(handles) = resolve_dut_handles() else {
        vpi::printf("ERROR: could not resolve one or more DUT handles");
        HAD_FAILURE.store(true, Ordering::SeqCst);
        control(Control::Finish);
        return;
    };

    let test = TESTS[index];
    vpi::printf!("Running test {}: {}", index + 1, test.name);

    let Some(vec_in_values) = string_to_scalar_vector(test.vec_in) else {
        vpi::printf!(
            "ERROR [{}]: invalid vec_in scalar string '{}'",
            test.name,
            test.vec_in
        );
        HAD_FAILURE.store(true, Ordering::SeqCst);
        control(Control::Finish);
        return;
    };
    let Some(arr_in_0_values) = string_to_scalar_vector(test.arr_in[0]) else {
        vpi::printf!(
            "ERROR [{}]: invalid arr_in[0] scalar string '{}'",
            test.name,
            test.arr_in[0]
        );
        HAD_FAILURE.store(true, Ordering::SeqCst);
        control(Control::Finish);
        return;
    };
    let Some(arr_in_1_values) = string_to_scalar_vector(test.arr_in[1]) else {
        vpi::printf!(
            "ERROR [{}]: invalid arr_in[1] scalar string '{}'",
            test.name,
            test.arr_in[1]
        );
        HAD_FAILURE.store(true, Ordering::SeqCst);
        control(Control::Finish);
        return;
    };

    let _ = handles.bit_in.put_value(&Value::Scalar(test.bit_in));
    let _ = handles.vec_in.put_value(&Value::Vector(vec_in_values));
    let _ = handles.int_in.put_value(&Value::Int(test.int_in));
    let _ = handles.arr_in_0.put_value(&Value::Vector(arr_in_0_values));
    let _ = handles.arr_in_1.put_value(&Value::Vector(arr_in_1_values));

    let _ = register_cb_with_time(
        CbReason::AfterDelay,
        Time::Sim(test.verify_delay),
        verify_current_test,
    );
}

fn verify_current_test(_cb_data: &CbData) {
    let index = CURRENT_TEST_INDEX.load(Ordering::SeqCst);
    if index >= TESTS.len() {
        HAD_FAILURE.store(true, Ordering::SeqCst);
        control(Control::Finish);
        return;
    }

    let test = TESTS[index];
    let mut ok = true;

    let Some(handles) = resolve_dut_handles() else {
        vpi::printf("ERROR: could not resolve one or more DUT handles");
        HAD_FAILURE.store(true, Ordering::SeqCst);
        control(Control::Finish);
        return;
    };

    match handles.bit_in.get_value(ValueType::Scalar) {
        Some(Value::Scalar(v)) if v == test.bit_in => {}
        other => {
            ok = false;
            vpi::printf!(
                "ERROR [{}]: bit_in expected {}, got {:?}",
                test.name,
                char::from(test.bit_in),
                other
            );
        }
    }

    let Some(expected_in_vec) = normalized_scalar_string(test.vec_in) else {
        vpi::printf!(
            "ERROR [{}]: invalid vec_in scalar string '{}'",
            test.name,
            test.vec_in
        );
        HAD_FAILURE.store(true, Ordering::SeqCst);
        control(Control::Finish);
        return;
    };
    match handles.vec_in.get_value(ValueType::Vector) {
        Some(Value::Vector(v)) => {
            let s = scalar_vector_to_string(&v);
            if s != expected_in_vec {
                ok = false;
                vpi::printf!(
                    "ERROR [{}]: vec_in expected {}, got {}",
                    test.name,
                    expected_in_vec,
                    s
                );
            }
        }
        other => {
            ok = false;
            vpi::printf!(
                "ERROR [{}]: vec_in expected vector, got {:?}",
                test.name,
                other
            );
        }
    }

    for (label, handle, expected_src) in [
        ("arr_in[0]", &handles.arr_in_0, test.arr_in[0]),
        ("arr_in[1]", &handles.arr_in_1, test.arr_in[1]),
    ] {
        let Some(expected) = normalized_scalar_string(expected_src) else {
            vpi::printf!(
                "ERROR [{}]: invalid {} scalar string '{}'",
                test.name,
                label,
                expected_src
            );
            HAD_FAILURE.store(true, Ordering::SeqCst);
            control(Control::Finish);
            return;
        };

        match handle.get_value(ValueType::Vector) {
            Some(Value::Vector(v)) => {
                let s = scalar_vector_to_string(&v);
                if s != expected {
                    ok = false;
                    vpi::printf!(
                        "ERROR [{}]: {} expected {}, got {}",
                        test.name,
                        label,
                        expected,
                        s
                    );
                }
            }
            other => {
                ok = false;
                vpi::printf!(
                    "ERROR [{}]: {} expected vector, got {:?}",
                    test.name,
                    label,
                    other
                );
            }
        }
    }

    match handles.int_in.get_value(ValueType::Int) {
        Some(Value::Int(v)) if v == test.int_in => {}
        other => {
            ok = false;
            vpi::printf!(
                "ERROR [{}]: int_in expected {}, got {:?}",
                test.name,
                test.int_in,
                other
            );
        }
    }

    let expected_bit_out = invert_binary_scalar(test.bit_in);
    match handles.bit_out.get_value(ValueType::Scalar) {
        Some(Value::Scalar(v)) if v == expected_bit_out => {}
        other => {
            ok = false;
            vpi::printf!(
                "ERROR [{}]: bit_out expected {}, got {:?}",
                test.name,
                char::from(expected_bit_out),
                other
            );
        }
    }

    let Some(expected_out_vec) = inverted_scalar_string(test.vec_in) else {
        vpi::printf!(
            "ERROR [{}]: invalid vec_in scalar string '{}'",
            test.name,
            test.vec_in
        );
        HAD_FAILURE.store(true, Ordering::SeqCst);
        control(Control::Finish);
        return;
    };
    match handles.vec_out.get_value(ValueType::Vector) {
        Some(Value::Vector(v)) => {
            let s = scalar_vector_to_string(&v);
            if s != expected_out_vec {
                ok = false;
                vpi::printf!(
                    "ERROR [{}]: vec_out expected {}, got {}",
                    test.name,
                    expected_out_vec,
                    s
                );
            }
        }
        other => {
            ok = false;
            vpi::printf!(
                "ERROR [{}]: vec_out expected vector, got {:?}",
                test.name,
                other
            );
        }
    }

    for (label, handle, expected_src) in [
        ("arr_out[0]", &handles.arr_out_0, test.arr_in[0]),
        ("arr_out[1]", &handles.arr_out_1, test.arr_in[1]),
    ] {
        let Some(expected) = inverted_scalar_string(expected_src) else {
            vpi::printf!(
                "ERROR [{}]: invalid source for {} scalar string '{}'",
                test.name,
                label,
                expected_src
            );
            HAD_FAILURE.store(true, Ordering::SeqCst);
            control(Control::Finish);
            return;
        };

        match handle.get_value(ValueType::Vector) {
            Some(Value::Vector(v)) => {
                let s = scalar_vector_to_string(&v);
                if s != expected {
                    ok = false;
                    vpi::printf!(
                        "ERROR [{}]: {} expected {}, got {}",
                        test.name,
                        label,
                        expected,
                        s
                    );
                }
            }
            other => {
                ok = false;
                vpi::printf!(
                    "ERROR [{}]: {} expected vector, got {:?}",
                    test.name,
                    label,
                    other
                );
            }
        }
    }

    let expected_int_out = test.int_in + 1;
    match handles.int_out.get_value(ValueType::Int) {
        Some(Value::Int(v)) if v == expected_int_out => {}
        other => {
            ok = false;
            vpi::printf!(
                "ERROR [{}]: int_out expected {}, got {:?}",
                test.name,
                expected_int_out,
                other
            );
        }
    }

    if ok {
        vpi::printf!("PASS [{}]", test.name);
    } else {
        HAD_FAILURE.store(true, Ordering::SeqCst);
        vpi::printf!("FAIL [{}]", test.name);
    }

    CURRENT_TEST_INDEX.fetch_add(1, Ordering::SeqCst);
    let _ = register_cb_with_time(
        CbReason::AfterDelay,
        Time::Sim(test.inter_test_delay),
        run_next_test,
    );
}
