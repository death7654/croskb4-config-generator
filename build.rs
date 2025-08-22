use cc;
use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search=/path/to/lib");

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=bz2");

    cc::Build::new()
        .cpp(true)
        .file("wrapper.cpp") // your wrapper
        .flag_if_supported("-std=c++17")
        .compile("vivaldi_wrapper");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-xc++")
        .clang_arg("-std=c++17")
        .layout_tests(false)
        .derive_default(true)
        .generate_comments(true)
        .generate_inline_functions(true)
        .allowlist_type(".*") // ⬅️ include ALL types
        .allowlist_function(".*") // ⬅️ include ALL functions
        .allowlist_var(".*") // ⬅️ include ALL constants/macros
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .unwrap();
}
