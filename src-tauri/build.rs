fn main() {
    cxx_build::bridge("src/main.rs") //correct
        .file("src/hello.cc") //correct
	.flag_if_supported("-std=c++14")
        .compile("tauri-windows-titlebar");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/hello.cc");
    println!("cargo:rerun-if-changed=include/hello.h");

    tauri_build::build()
}
