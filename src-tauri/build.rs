fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/hello.cc")
        .compile("src-tauri");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/hello.cc");
    println!("cargo:rerun-if-changed=src/include/hello.h");

    tauri_build::build()
}
