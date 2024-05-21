use std::env;
use std::path::PathBuf;

fn main() {
    system_deps::Config::new().probe().unwrap();

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let build_rs_path = manifest_dir.join("build.rs");
    let file_path = manifest_dir.join("src").join("vqsort.cpp");

    println!("cargo:rerun-if-changed={}", build_rs_path.display());
    println!("cargo:rerun-if-changed={}", file_path.display());

    cc::Build::new().file(file_path).cpp(true).compile("vqsort");
    println!("cargo:rustc-link-lib=static=vqsort");
}
