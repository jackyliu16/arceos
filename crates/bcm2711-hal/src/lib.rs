#![no_std]
#![feature(asm, naked_functions)]

extern crate embedded_hal as hal;

pub use bcm2711_regs;
pub use aarch64_cpu;

pub mod gpio;
