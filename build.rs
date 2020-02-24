extern crate cc;
// Example custom build script.
/*
   ref https://rust-embedded.github.io/book/interoperability/c-with-rust.html

   A build.rs script is a file written in Rust syntax,
   that is executed on your compilation machine,
   AFTER dependencies of your project have been built,
   but BEFORE your project is built.
*/
// @ref
// https://doc.rust-lang.org/cargo/reference/build-scripts.html
// https://github.com/alexcrichton/rust-ffi-examples
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=src/cpp/foo.cpp");
    // Use the `cc` crate to build a C file and statically link it.
    // https://github.com/alexcrichton/cc-rs
    cc::Build::new()
        .cpp(true)
        .file("src/cpp/foo.cpp")
        .compile("libmycppcode.a");
}
