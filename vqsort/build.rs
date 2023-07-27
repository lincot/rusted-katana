use std::env;
use std::path::PathBuf;

fn main() {
    const FILE_NAME: &str = "vqsort.cpp";
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let build_rs_path = manifest_dir.join("build.rs");
    let file_path = manifest_dir.join("src").join(FILE_NAME);

    println!("cargo:rerun-if-changed={}", build_rs_path.display());
    println!("cargo:rerun-if-changed={}", file_path.display());

    cc::Build::new()
        .file(file_path)
        .cpp(true)
        .flag_if_supported("/EHsc")
        .flag_if_supported("/Zc:__cplusplus")
        .flag_if_supported("/std:c++20")
        .flag_if_supported("-std=c++20")
        .flag_if_supported("-fdiagnostics-color=always")
        .flag("-march=native")
        .force_frame_pointer(false)
        .define("NDEBUG", None)
        .debug(false)
        .compiler("clang++")
        .opt_level(3)
        .compile(FILE_NAME);

    println!("cargo:rustc-link-search={}", env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-lib=static={FILE_NAME}");
    println!("cargo:rustc-link-lib=hwy_contrib");
}
