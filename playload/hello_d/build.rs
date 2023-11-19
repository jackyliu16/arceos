//! A simple linker script

fn main() {
    println!("cargo:rustc-link-arg=-T./playload/hello_app/linker.ld");
}
