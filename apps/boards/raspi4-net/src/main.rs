#![no_std]
#![no_main]

#[cfg(feature = "axstd")]
use axstd::println;

#[no_mangle]
fn main() {
    println!("Hello, world!");
}
