#![feature(asm_const)]
#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[cfg(feature = "axstd")]
use axstd::println;

use core::mem::size_of;

const PLASH_START: usize = 0x22000000;

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    let load_start = PLASH_START as *const u8;
    let byte = unsafe {
    core::slice::from_raw_parts(load_start, size_of::<u8>())
    };
    let load_size = u8::from_be_bytes([byte[0]]) as usize;
    println!("{byte:?}");

    println!("Load payload ...");

    let load_code = unsafe { core::slice::from_raw_parts(load_start.offset(size_of::<u8>() as isize), load_size) };
    println!("load code {:?}; address [{:?}]", load_code, load_code.as_ptr());

    // app running aspace
    // SBI(0x80000000) -> App <- Kernel(0x80200000)
    // 0xffff_ffc0_0000_0000
    const RUN_START: usize = 0xffff_ffc0_8010_0000;

    let run_code = unsafe {
        core::slice::from_raw_parts_mut(RUN_START as *mut u8, load_size)
    };
    run_code.copy_from_slice(load_code);
    println!("run code {:?}; address [{:?}]", run_code, run_code.as_ptr());

    println!("Load payload ok!");
    
    register_abi(SYS_HELLO, abi_hello as usize);
    register_abi(SYS_PUTCHAR, abi_putchar as usize);
    register_abi(SYS_TERMINATE, abi_terminate as usize);

    println!("Execute app ...");

    // execute app
    // load table symbol into a7
    // jump to first app
    unsafe { core::arch::asm!("
        la      a7, {abi_table}
        li      t2, {run_start}
        jalr    t2",
        run_start = const RUN_START,
        abi_table = sym ABI_TABLE,
    )}
}

const SYS_HELLO: usize = 1;
const SYS_PUTCHAR: usize = 2;
const SYS_TERMINATE: usize = 3;

static mut ABI_TABLE: [usize; 16] = [0; 16];

fn register_abi(num: usize, handle: usize) {
    unsafe { ABI_TABLE[num] = handle; }
}

fn abi_hello() {
    println!("[ABI:Hello] Hello, Apps!");
    unsafe { core::arch::asm!("la a7, {abi_table}", abi_table = sym ABI_TABLE )}
}

fn abi_putchar(c: char) {
    println!("[ABI:Print] {c}");
    unsafe { core::arch::asm!("la a7, {abi_table}", abi_table = sym ABI_TABLE )}
}

fn abi_terminate() -> ! {
    println!("[ABI:Terminate] Shutting Down !!!");
    arceos_api::sys::ax_terminate()
}