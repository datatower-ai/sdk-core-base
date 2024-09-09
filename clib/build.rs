extern crate cbindgen;

use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    cbindgen::Builder::new().with_crate(crate_dir)
        .with_language(cbindgen::Language::C)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("dt_core_clib.h");

    csbindgen::Builder::default()
        .input_extern_file("src/lib.rs")
        .csharp_dll_name("dt_core_clib")
        .csharp_namespace("DTCore.Native")
        .csharp_class_accessibility("internal")
        .generate_csharp_file("../csharp/DTCore/DTCore/Native/DtCoreNative.g.cs")
        .unwrap();
}