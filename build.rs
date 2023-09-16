fn main() {
    // Enable compiling C backend for systems that need it.
    #[cfg(any(target_os = "openbsd", target_os = "macos", target_os = "freebsd"))]
    compile_c_backend();
}

#[allow(dead_code)]
fn compile_c_backend() {
    let dest = cmake::build("src/backend/c");

    println!("cargo:rustc-link-search=native={}", dest.display());
    println!("cargo:rustc-link-lib=static=libpci-rs-c-backend");
}
