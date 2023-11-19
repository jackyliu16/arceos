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
    let byte = unsafe { core::slice::from_raw_parts(load_start, size_of::<u8>()) };
    let load_size = u8::from_be_bytes([byte[0]]) as usize;
    println!("{byte:?}");

    println!("Load payload ...");

    unsafe {
        init_app_page_table();
    }
    unsafe {
        switch_app_aspace();
    }
    // app running aspace
    // SBI(0x80000000) -> App <- Kernel(0x80200000)
    // 0xffff_ffc0_0000_0000
    const RUN_START: usize = 0x4010_0000;

    let load_code = unsafe {
        core::slice::from_raw_parts(load_start.offset(size_of::<u8>() as isize), load_size)
    };
    println!(
        "load code {:?}; address [{:?}]",
        load_code,
        load_code.as_ptr()
    );

    let run_code = unsafe { core::slice::from_raw_parts_mut(RUN_START as *mut u8, load_size) };
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
    unsafe {
        core::arch::asm!("
        la      a7, {abi_table}
        li      t2, {run_start}
        jalr    t2",
            run_start = const RUN_START,
            abi_table = sym ABI_TABLE,
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
    unsafe { core::arch::asm!("la a7, {abi_table}", abi_table = sym ABI_TABLE ) }
}

fn abi_putchar(c: char) {
    println!("[ABI:Print] {c}");
    unsafe { core::arch::asm!("la a7, {abi_table}", abi_table = sym ABI_TABLE ) }
}

fn abi_terminate() -> ! {
    println!("[ABI:Terminate] Shutting Down !!!");
    arceos_api::sys::ax_terminate()
}

//
// App aspace
//

#[link_section = ".data.app_page_table"]
static mut APP_PT_SV39: [u64; 512] = [0; 512];

unsafe fn init_app_page_table() {
    // 0x8000_0000..0xc000_0000, VRWX_GAD, 1G block
    APP_PT_SV39[2] = (0x80000 << 10) | 0xef;
    // 0xffff_ffc0_8000_0000..0xffff_ffc0_c000_0000, VRWX_GAD, 1G block
    APP_PT_SV39[0x102] = (0x80000 << 10) | 0xef;

    // 0x0000_0000..0x4000_0000, VRWX_GAD, 1G block
    APP_PT_SV39[0] = (0x00000 << 10) | 0xef;

    // For App aspace!
    // 0x4000_0000..0x8000_0000, VRWX_GAD, 1G block
    APP_PT_SV39[1] = (0x80000 << 10) | 0xef;
}

unsafe fn switch_app_aspace() {
    use riscv::register::satp;
    let page_table_root = APP_PT_SV39.as_ptr() as usize - axconfig::PHYS_VIRT_OFFSET;
    satp::set(satp::Mode::Sv39, 0, page_table_root >> 12);
    riscv::asm::sfence_vma_all();
}
