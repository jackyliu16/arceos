//! A simple linker script

fn main() {
  let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
  if !arch.contains("riscv") {
    println!("arch {} is not allow in this script", arch);
  }

  let ld_content = std::fs::read_to_string("linker.ld").unwrap();
  let ld_content = ld_content.replace("%APP_SIZE%", "32");

  std::fs::write("linker_with_app_size.ld", ld_content).unwrap();

  println!("cargo:rustc-link-arg=-T./playload/hello_app/linker_with_app_size.ld");
}