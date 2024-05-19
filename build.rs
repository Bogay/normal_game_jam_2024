use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    build_vosk();
    build_our_ffi();
}

fn build_our_ffi() {
    println!("cargo:rerun-if-changed=src/ffi");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let dll_path = PathBuf::from("src/ffi/libentry.lib");
    if dll_path.exists() {
        std::fs::copy(dll_path, out_path.join("libentry.lib")).unwrap();
        println!("cargo:rustc-link-search={}", out_path.display());
        println!("cargo:rustc-link-lib=libentry");
    }

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("src/ffi/bullet.hpp")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn build_vosk() {
    // Directory containing the Vosk files
    let vosk_dir = PathBuf::from("vosk-win64-0.3.45");

    // Output directory (where Cargo will place the build artifacts)
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Ensure the output directory exists
    fs::create_dir_all(&out_dir).unwrap();

    // Copy all files from the Vosk directory to the output directory
    for entry in fs::read_dir(&vosk_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let file_name = path.file_name().unwrap();
            fs::copy(&path, out_dir.join(file_name)).unwrap();
        }
    }

    // Configure the linker search path
    println!("cargo:rustc-link-search=native={}", out_dir.display());

    // Link with the static Vosk library
    // Adjust the library name as necessary (omit the 'lib' prefix and '.a' suffix)
    println!("cargo:rustc-link-lib=static=libvosk");
}
