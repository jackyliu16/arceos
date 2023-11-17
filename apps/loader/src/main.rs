#![feature(asm_const)]
#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

use core::mem::size_of;

#[cfg(feature = "axstd")]
use axstd::println;
const PLASH_START: usize = 0x22000000;
const LOAD_START: usize = 0xffff_ffc0_8010_0000;
#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    let apps_start = PLASH_START as *const u8;
    let byte = unsafe { core::slice::from_raw_parts(apps_start, size_of::<u16>()) };
    let app_size_1 = u8::from_be_bytes([byte[0]]);
    let app_size_2 = u8::from_be_bytes([byte[1]]);
    println!("size 1: {app_size_1}, size 2: {app_size_2}");

    println!("Load payload ...");

    println!("sizeByte: {byte:?}");

    let read_only_app1 = unsafe {
        core::slice::from_raw_parts(
            apps_start.offset(size_of::<u16>() as isize),
            app_size_1 as usize,
        )
    };
    let read_only_app2 = unsafe {
        core::slice::from_raw_parts(
            apps_start.offset((size_of::<u16>() + app_size_1 as usize) as isize),
            app_size_2 as usize,
        )
    };

    // println!("content: {:?}: ", code);
    println!("Load payload ok!");

    println!("Copy app ...");
    let load_start = LOAD_START as *const u8;

    // load app 1
    let load_app_1 =
        unsafe { core::slice::from_raw_parts_mut(load_start as *mut u8, app_size_1 as usize) };
    let load_app_2 = unsafe {
        core::slice::from_raw_parts_mut(
            load_start.offset(app_size_1 as isize) as *mut u8,
            app_size_2 as usize,
        )
    };

    // Copy App Data From ReadOnly Areas
    load_app_1.copy_from_slice(read_only_app1);
    load_app_2.copy_from_slice(read_only_app2);

    println!("Original App: ");
    println!("1: {read_only_app1:?}");
    println!("2: {read_only_app2:?}");

    println!("Load App:");
    println!("1: {load_app_1:?}");
    println!("2: {load_app_2:?}");

    println!("ORIGINAL AREAS: ");
    println!("{:?}", unsafe {
        core::slice::from_raw_parts(apps_start, 32)
    });

    println!("LOADING AREAS: ");
    println!("{:?}", unsafe {
        core::slice::from_raw_parts(load_start, 32)
    });

    println!("Execute app ...");
    unsafe {
        core::arch::asm!("
      li      t2, {run_start}
      jalr    t2",
          run_start = const LOAD_START,
        )
    }
    println!("App 1 Finish");
    register_abi(SYS_HELLO, abi_hello as usize);
    register_abi(SYS_PUTCHAR, abi_putchar as usize);
    register_abi(SYS_TERMINATE, abi_terminate as usize);

    let jump_location: usize = LOAD_START + app_size_1 as usize;
    println!("Execute app ...");

    // execute app
    println!("Execute app ...");

    // execute app
    unsafe {
        core::arch::asm!("
        la      a7, {abi_table}
        mv      t2, {run_start}
        jalr    t2
        j       .",
            abi_table = sym ABI_TABLE,
            run_start = in(reg) jump_location
        )
    }
}

const SYS_HELLO: usize = 1;
const SYS_PUTCHAR: usize = 2;
const SYS_TERMINATE: usize = 3;

static mut ABI_TABLE: [usize; 16] = [0; 16];

fn register_abi(num: usize, handle: usize) {
    unsafe {
        ABI_TABLE[num] = handle;
    }
}

fn abi_hello() {
    println!("[ABI:Hello] Hello, Apps!");
    unsafe { core::arch::asm!("la   a7,{}", sym ABI_TABLE) }
}

fn abi_putchar(c: char) {
    println!("[ABI:Print] {c}");
    unsafe { core::arch::asm!("la   a7,{}", sym ABI_TABLE) }
}

fn abi_terminate() -> ! {
    println!("[ABI:TERMINATE]: Shutting Down !!!");
    arceos_api::sys::ax_terminate();
}
