//! VPI test for vpi_handle_multi(vpiInterModPath), vpi_get_delays, vpi_put_delays.
//! Pairs with ../top.v and ../test.sdf. Registers the system task $intermod_test.

use std::ffi::c_char;
use std::sync::atomic::{AtomicU32, Ordering};

use vpi::{DelayData, DelayTimeType, Handle, ObjectType, SystfKind, Time};

static ERRORS: AtomicU32 = AtomicU32::new(0);

fn check(cond: bool, msg: &str) {
    if cond {
        vpi::printf!("  PASS: {}\n", msg);
    } else {
        vpi::printf!("  FAIL: {}\n", msg);
        ERRORS.fetch_add(1, Ordering::Relaxed);
    }
}

fn near(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-3
}

/// Find a port object by module path + port name. handle_by_name("top.u_drv.y")
/// would return the net rather than the port, so iterate the module's ports.
fn find_port(module: &str, port: &str) -> Handle {
    let m = Handle::handle_by_name(module);
    if m.is_null() {
        return Handle::null();
    }
    for p in m.iterator(ObjectType::Port) {
        if p.get_name().as_deref() == Some(port) {
            return p;
        }
    }
    Handle::null()
}

fn real(t: &Time) -> f64 {
    match t {
        Time::ScaledReal(r) => *r,
        _ => f64::NAN,
    }
}

/// Read `n` delays as min:typ:max triples.
fn get_mtm(path: &Handle, n: usize) -> Option<Vec<f64>> {
    let data = path.get_delays_mtm(n, DelayTimeType::ScaledReal)?;
    data.mtm.then(|| data.delays.iter().map(real).collect())
}

fn check_mtm(path: &Handle, label: &str, expected: [f64; 6]) {
    let Some(got) = get_mtm(path, 2) else {
        check(false, &format!("{label} get_delays_mtm failed"));
        return;
    };
    if got.len() != expected.len() {
        check(
            false,
            &format!(
                "{label} returned {} MTM values (expected {})",
                got.len(),
                expected.len()
            ),
        );
        return;
    }
    // Order: rise min,typ,max then fall min,typ,max.
    for (i, (g, e)) in got.iter().zip(expected.iter()).enumerate() {
        check(
            near(*g, *e),
            &format!("{label} mtm[{i}]={g} (expected {e})"),
        );
    }
}

/// Print the simulator's pending VPI error (if any).
fn print_vpi_error(ctx: &str) {
    match vpi::check_error() {
        Some(error) => vpi::printf!("  DIAG {}: vpi_chk_error {}\n", ctx, error),
        None => vpi::printf!("  DIAG {}: no pending VPI error\n", ctx),
    }
}

/// Explain why vpi_handle_multi(vpiInterModPath, src, dst) might return NULL.
fn diagnose(label: &str, src: &Handle, dst: &Handle) {
    let full = |h: &Handle| h.get_full_name().unwrap_or_default();
    let hiconn = |h: &Handle| {
        h.get(ObjectType::HighConn)
            .get_full_name()
            .unwrap_or_default()
    };
    vpi::printf!(
        "  DIAG {}: src={} dir={:?} hiconn={} | dst={} dir={:?} hiconn={}\n",
        label,
        full(src),
        src.get_direction(),
        hiconn(src),
        full(dst),
        dst.get_direction(),
        hiconn(dst)
    );
    // Both hiconn names should be the same net (top.n1 / top.n2).
    let _ = src.get_intermod_path(dst);
    print_vpi_error(label);
    let rev = dst.get_intermod_path(src);
    vpi::printf!(
        "  DIAG {}: reversed order gives {}\n",
        label,
        if rev.is_null() { "NULL" } else { "a handle" }
    );
}

fn summary() {
    let e = ERRORS.load(Ordering::Relaxed);
    vpi::printf!(
        "== intermod_test: {} ({} errors) ==\n\n",
        if e == 0 { "PASSED" } else { "FAILED" },
        e
    );
}

fn run() {
    vpi::printf!("\n== intermod_test (Rust) ==\n");

    let p_y = find_port("top.u_drv", "y");
    let p_d = find_port("top.u_rcv", "d");
    let p_q = find_port("top.u_rcv", "q");
    let p_d2 = find_port("top.u_rcv2", "d");
    let p_a = find_port("top.u_drv", "a");
    let all = [&p_y, &p_d, &p_q, &p_d2, &p_a].iter().all(|h| !h.is_null());
    check(all, "all port handles found");
    if !all {
        return;
    }

    // 1. vpi_handle_multi, connected ports
    let path1 = p_y.get_intermod_path(&p_d);
    let path2 = p_q.get_intermod_path(&p_d2);
    check(!path1.is_null(), "path u_drv.y -> u_rcv.d exists");
    check(!path2.is_null(), "path u_rcv.q -> u_rcv2.d exists");

    // 2. vpi_handle_multi, unconnected ports must give NULL
    let bad = p_a.get_intermod_path(&p_d);
    check(bad.is_null(), "no path u_drv.a -> u_rcv.d (negative test)");

    if path1.is_null() || path2.is_null() {
        // The negative test above is only meaningful if the positive one works:
        // a call that always returns NULL passes it too.
        vpi::printf!("  NOTE: negative test is not informative while connected ports give NULL\n");
        diagnose("path1", &p_y, &p_d);
        diagnose("path2", &p_q, &p_d2);
        summary();
        return;
    }

    // 3. vpi_get_delays against the SDF values
    check_mtm(&path1, "path1 get", [0.5, 0.6, 0.7, 0.3, 0.4, 0.5]);
    check_mtm(&path2, "path2 get", [1.0, 1.0, 1.0, 1.2, 1.2, 1.2]);

    // 3b. safe API (mtm_flag = 0): two delays, rise and fall. Simulator picks
    //     min/typ/max per its delay-selection mode, so only check the range.
    match path1.get_delays(2, DelayTimeType::ScaledReal) {
        Some(d) if d.delays.len() == 2 => {
            let (r, f) = (real(&d.delays[0]), real(&d.delays[1]));
            check(
                (0.5..=0.7).contains(&r),
                &format!("safe get rise={r} within 0.5..0.7"),
            );
            check(
                (0.3..=0.5).contains(&f),
                &format!("safe get fall={f} within 0.3..0.5"),
            );
        }
        other => check(false, &format!("safe get_delays returned {other:?}")),
    }

    // 4. vpi_put_delays on path1 (rise 2.0, fall 2.5), then read back
    let put = DelayData::with_time_type(
        vec![Time::ScaledReal(2.0), Time::ScaledReal(2.5)],
        DelayTimeType::ScaledReal,
    );
    check(path1.put_delays(&put), "put_delays accepted");
    match path1.get_delays(2, DelayTimeType::ScaledReal) {
        Some(d) if d.delays.len() == 2 => {
            check(
                near(real(&d.delays[0]), 2.0),
                &format!("put/get rise={} (expected 2.0)", real(&d.delays[0])),
            );
            check(
                near(real(&d.delays[1]), 2.5),
                &format!("put/get fall={} (expected 2.5)", real(&d.delays[1])),
            );
        }
        other => check(false, &format!("readback returned {other:?}")),
    }

    let e = ERRORS.load(Ordering::Relaxed);
    vpi::printf!(
        "== intermod_test: {} ({} errors) ==\n\n",
        if e == 0 { "PASSED" } else { "FAILED" },
        e
    );
}

unsafe extern "C" fn intermod_test_calltf(_user_data: *mut c_char) -> i32 {
    // Never let a panic unwind into the simulator.
    let _ = std::panic::catch_unwind(run);
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn intermod_test_register() {
    let _ = vpi::register_systf(
        SystfKind::Task,
        c"$intermod_test",
        Some(intermod_test_calltf),
        None,
        None,
        std::ptr::null_mut(),
        None,
    );
}

vpi::startup_routines!(intermod_test_register);
