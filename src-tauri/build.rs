fn main() {
    use std::env;

    if env::var("DOCS_RS").is_ok() {
        return;
    }

    let mut cxx = cxx_build::bridge("src/main.rs");

    let out_dir = env::var("OUT_DIR").unwrap();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cxx.include(manifest_dir)
        .file("src/hello.cc")
        .include(out_dir)
        .flag_if_supported("-std=c++14")
        .compile("tauri-windows-titlebar");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/hello.cc");
    println!("cargo:rerun-if-changed=src/hello.h");

    tauri_build::build()
}
