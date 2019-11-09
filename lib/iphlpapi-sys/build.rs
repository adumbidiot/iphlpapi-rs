use std::{
    env,
    path::PathBuf,
};

fn main() {
    println!("cargo:rustc-link-lib=iphlpapi");
    println!("cargo:rerun-if-changed=wrapper.h");
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        //.parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .whitelist_function("GetAdaptersInfo")
        .whitelist_var("ERROR_(SUCCESS|BUFFER_OVERFLOW)")
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
