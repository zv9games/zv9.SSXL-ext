// rust/ssxl_cli/build.rs
use std::env;
use std::path::PathBuf;

fn main() {
    // 1. Get the output directory for the current build step
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // 2. Calculate the path to the 'deps' directory where ssxl_ext.dll.lib lives.
    // We traverse up 3 levels from the build script output:
    // target/debug/build/ssxl_cli-XYZ/out -> target/debug/deps
    let deps_dir = out_dir
        .parent().unwrap() // ssxl_cli-XYZ
        .parent().unwrap() // build
        .parent().unwrap() // debug
        .join("deps");

    // 3. Add the search path for the linker
    println!("cargo:rustc-link-search=native={}", deps_dir.display());

    // 4. Link against the 'ssxl_ext' library dynamically.
    // CRITICAL FIX: Link against the DLL name ('ssxl_ext.dll') to correctly find the
    // MSVC-generated import library ('ssxl_ext.dll.lib').
    println!("cargo:rustc-link-lib=dylib=ssxl_ext.dll"); // <-- FIXED

    // 5. Ensure we rebuild if the library changes
    println!("cargo:rerun-if-changed=../ssxl_ext");
}