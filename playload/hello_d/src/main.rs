#![allow(dead_code, unused)]
#![feature(asm_const)]
#![no_std]
#![no_main]

const SYS_HELLO: usize = 1;
const SYS_PUTCHAR: usize = 2;
const SYS_TERMINATE: usize = 3;

static mut ABI_TABLE: u32 = 0;

#[no_mangle]
unsafe extern "C" fn _start() {
    // Read Information From a7 Register
    core::arch::asm!("mv    {}, a7", out(reg) ABI_TABLE);

    put_char('D');
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn put_char(c: char) {
    unsafe {
        let arg0: u8 = c as u8;
        core::arch::asm!("
            li      t0, {abi_num}
            slli    t0, t0, 3
            add     t1, a7, t0
            ld      t1, (t1)
            jalr    t1",
            clobber_abi("C"),
            abi_num = const SYS_PUTCHAR,
            in("a0") arg0,
        )
    }
}
