use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use vpi::{register_cb, startup_routines, CbData, CbReason};

startup_routines!(error_test_startup);

static ERROR_COUNT: AtomicUsize = AtomicUsize::new(0);
static ERROR_MESSAGES: Mutex<Vec<String>> = Mutex::new(Vec::new());

fn error_callback(data: &CbData) {
    ERROR_COUNT.fetch_add(1, Ordering::SeqCst);
    let message = format!(
        "Error callback triggered at time {:?}, reason: {:?}",
        data.time, data.reason
    );
    if let Ok(mut msgs) = ERROR_MESSAGES.lock() {
        msgs.push(message.clone());
    }
    eprintln!("[ERROR_TEST VPI] {}", message);
}

fn pli_error_callback(data: &CbData) {
    ERROR_COUNT.fetch_add(1, Ordering::SeqCst);
    let message = format!("PLI error callback triggered at time {:?}", data.time);
    if let Ok(mut msgs) = ERROR_MESSAGES.lock() {
        msgs.push(message.clone());
    }
    eprintln!("[ERROR_TEST VPI] {}", message);
}

fn end_of_simulation_callback(_data: &CbData) {
    let count = ERROR_COUNT.load(Ordering::SeqCst);
    let messages = ERROR_MESSAGES.lock().unwrap();

    eprintln!(
        "[ERROR_TEST VPI] Simulation ended. Total errors captured: {}",
        count
    );
    for (i, msg) in messages.iter().enumerate() {
        eprintln!("[ERROR_TEST VPI] Error {}: {}", i + 1, msg);
    }

    if count > 0 {
        eprintln!("[ERROR_TEST VPI] SUCCESS: Runtime error was detected via VPI callbacks");
    } else {
        eprintln!("[ERROR_TEST VPI] WARNING: No errors were captured");
    }
}

pub extern "C" fn error_test_startup() {
    eprintln!("[ERROR_TEST VPI] Registering error callbacks");

    // Register error callback
    let _ = register_cb(CbReason::Error, error_callback);
    if let Some(data) = vpi::check_error() {
        dbg!("[ERROR_TEST VPI] check_error returned data: {:?}", data);
    }

    // Register PLI error callback
    let _ = register_cb(CbReason::PLIError, pli_error_callback);
    if let Some(data) = vpi::check_error() {
        dbg!("[ERROR_TEST VPI] check_error returned data: {:?}", data);
    }

    // Register end of simulation callback to report results
    let _ = register_cb(CbReason::EndOfSimulation, end_of_simulation_callback);

    eprintln!("[ERROR_TEST VPI] Callbacks registered successfully");
}
