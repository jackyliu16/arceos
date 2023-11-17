#![feature(asm_const)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;
const SYS_HELLO: usize = 1;
const SYS_PUTCHAR: usize = 2;
const SYS_TERMINATE: usize = 3;

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
 core::arch::asm!(
  "wfi",
  options(noreturn)
 )
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
 loop { }
}
