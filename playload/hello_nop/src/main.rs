#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
unsafe extern "C" fn _start() {
    core::arch::asm!("nop")
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
