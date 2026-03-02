use std::env;
use std::path::PathBuf;

fn main() {
    // Rerun in case the header files change
    println!("cargo:rerun-if-changed=vpi_user.h");
    println!("cargo:rerun-if-changed=vpi_compatibility.h");
    println!("cargo:rerun-if-changed=sv_vpi_user.h");

    #[cfg(feature = "sv")]
    let bindings = bindgen::Builder::default()
        .header("sv_vpi_user.h")
        .generate()
        .expect("Unable to generate bindings");

    #[cfg(not(feature = "sv"))]
    let bindings = bindgen::Builder::default()
        .header("vpi_user.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
