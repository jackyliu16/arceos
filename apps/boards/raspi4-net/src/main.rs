#![no_std]
#![no_main]

#[cfg(feature = "axstd")]
use axstd::println;

extern crate bcm2711_hal as hal;

#[no_mangle]
fn main() {
    println!("Hello, world!");
}
