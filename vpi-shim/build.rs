fn main() {
    println!("cargo:rerun-if-changed=src/printf.c");

    let target_os = std::env::var("CARGO_CFG_TARGET_OS")
        .expect("Cargo must provide the target operating system");
    if matches!(target_os.as_str(), "windows" | "macos") {
        cc::Build::new()
            .file("src/printf.c")
            .warnings(true)
            .compile("vpi_shim_printf");
    }
}
