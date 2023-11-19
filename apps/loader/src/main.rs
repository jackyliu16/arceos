#![allow(dead_code, unused)]
#![feature(asm_const)]
#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

use core::mem::size_of;

#[cfg(feature = "axstd")]
use axstd::println;
const PLASH_START: usize = 0x22000000;
const LOAD_START: usize = 0xffff_ffc0_8010_0000;

use log::{debug, error, info, trace, warn};

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    println!("RUN LOADER");
    let apps_start = PLASH_START as *const u8;
    let load_start = LOAD_START as *const u8;

    // Gain NUM
    let byte_num = unsafe { core::slice::from_raw_parts(apps_start, size_of::<u8>()) };
    let app_num = u8::from_be_bytes([byte_num[0]]);
    println!("DETACT {app_num} app");

    let byte = unsafe { core::slice::from_raw_parts(apps_start, size_of::<u64>()) };
    // for b in byte { println!("{:08b}", b); }
    // Gain Each App Size
    let mut apps: [APP; MAX_APP_NUM] = [APP::empty(); MAX_APP_NUM];
    let byte_apps_sizes = unsafe {
        // NOTE: BC Rust Internal structure autocomplete will fill vacancy, thus u16 rather than u8
        core::slice::from_raw_parts(
            apps_start.offset(size_of::<u16>() as isize),
            app_num as usize * size_of::<u16>(),
        )
    };
    println!("app sizes: {byte_apps_sizes:?}");

    let head_offset = size_of::<u8>() + app_num as usize * size_of::<u16>();
    for i in 0..app_num {
        let i = i as usize;
        apps[i] = unsafe {
            APP::new(
                apps_start.offset(head_offset as isize),
                u16::from_be_bytes([byte_apps_sizes[i * 2], byte_apps_sizes[i * 2 + 1]]) as usize,
            )
        }
    }

    println!("{apps:?}");

    // LOAD APPLICATION
    for i in 0..app_num {
        let i = i as usize;
        let read_only_app =
            unsafe { core::slice::from_raw_parts(apps[i].start_addr, apps[i].size) };
        let load_app =
            unsafe { core::slice::from_raw_parts_mut(load_start as *mut u8, apps[i].size) };
        println!("Copy App {i} data from {}", apps[i].start_addr as usize);

        load_app.copy_from_slice(read_only_app);

        trace!("Original App: ");
        trace!("{i}: {read_only_app:?}");

        trace!("Load App:");
        trace!("{i}: {load_app:?}");

        println!("Executing App {i}");
    }

    // register_abi(SYS_HELLO, abi_hello as usize);
    // register_abi(SYS_PUTCHAR, abi_putchar as usize);
    // register_abi(SYS_TERMINATE, abi_terminate as usize);
    // println!("Execute app ...");
    // unsafe {
    //     core::arch::asm!("
    //         li      t2, {run_start}
    //         jalr    t2",
    //         run_start = const LOAD_START,
    //     )
    // }
    // println!("App 1 Finish");

    // let jump_location: usize = load_start as usize + app_size_1 as usize;
    // println!("jump into {jump_location:x}");

    // // execute app
    // unsafe {
    //     core::arch::asm!("
    //     la      a7, {abi_table}
    //     mv      t2, {run_start}
    //     jalr    t2
    //     j       .",
    //         abi_table = sym ABI_TABLE,
    //         run_start = in(reg) jump_location
    //     )
    // }
    // println!("App 2 Finish");
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

const MAX_APP_NUM: usize = u8::MAX as usize;
#[derive(Clone, Copy)]
struct APP {
    pub start_addr: *const u8,
    pub size: usize,
}

impl APP {
    pub fn new(start_addr: *const u8, size: usize) -> Self {
        Self { start_addr, size }
    }
    pub fn empty() -> Self {
        Self {
            start_addr: 0x0 as *const u8,
            size: 0,
        }
    }
}

impl core::fmt::Debug for APP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // 如果 size 为 0，则不显示任何信息
        if self.size == 0 {
            return Ok(());
        }

        // 自定义打印的行为
        f.debug_struct("APP")
            .field("start_addr", &self.start_addr)
            .field("size", &self.size)
            .finish()
    }
}
