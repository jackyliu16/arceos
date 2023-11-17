#![allow(dead_code, unused_variables, unused)]
#![feature(asm_const)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;
const SYS_HELLO: usize = 1;
const SYS_PUTCHAR: usize = 2;
const SYS_TERMINATE: usize = 3;

#[no_mangle]
unsafe extern "C" fn _start() {
    hello();
    hello();
    putchar('a');
    // putchars("hello, world!");
    terminate();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop { }
}

fn hello() {
    unsafe {
        core::arch::asm!("
        li      t0, {abi_num}
        slli    t0, t0, 3
        add     t1, a7, t0
        ld      t1, (t1)
        jalr    t1",
        abi_num = const SYS_HELLO,
    )}
}

fn putchars(s: &str) {
    for c in s.chars() {
        putchar(c);
    }
}

fn putchar(c: char) {
    let args0 = c as usize;
    unsafe {
        core::arch::asm!("
        li      t0, {abi_num}
        slli    t0, t0, 3
        add     t1, a7, t0
        ld      t1, (t1)
        jalr    t1",
        abi_num = const SYS_PUTCHAR,
        in("a0") args0,
    )}
}

fn terminate() {
    unsafe {
        core::arch::asm!("
        li      t0, {abi_num}
        slli    t0, t0, 3
        add     t1, a7, t0
        ld      t1, (t1)
        jalr    t1",
        abi_num = const SYS_TERMINATE,
        options(noreturn)
    )}
}