use std::env;
use std::path::PathBuf;

use bindgen;

fn main() {
    // Re-run the build script if these files change
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=keyboard.h");

    // Generate the bindings
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-xc++") // use C++ instead of C
        .clang_arg("-std=c++17")
        .clang_arg("-fms-compatibility") // 🔹 Avoid MSVC C++ issues
        .clang_arg("-DUSHORT=uint16_t")
        .clang_arg("-DULONG=uint32_t")
        .clang_arg("-DPULONG=uint32_t*")
        .clang_arg("-DBOOLEAN=bool")
        .clang_arg("-DTRUE=1")
        .clang_arg("-DFALSE=0")
        .clang_arg("-include")
        .clang_arg("stdint.h") // ensure stdint types like uint16_t are known
        .allowlist_type("KEYBOARD_INPUT_DATA")
        .allowlist_var("KEY_.*")
        .allowlist_var("K_.*")
        .allowlist_var("VIVALDI_.*")
        .allowlist_var("CROSKBHID_.*")
        .allowlist_var("REMAP_CFG_MAGIC")
        .generate()
        .expect("Unable to generate bindings");

    // Write bindings to the output directory
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
