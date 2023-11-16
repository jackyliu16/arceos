
#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

use core::mem::size_of;

#[cfg(feature = "axstd")]
use axstd::println;
const PLASH_START: usize = 0x22000000;
#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    let apps_start = PLASH_START as *const u8;
    let byte = unsafe {
      core::slice::from_raw_parts(apps_start, size_of::<u32>())
    };
    let apps_size = u32::from_be_bytes([byte[0], byte[1], byte[2], byte[3]]);

    println!("Load payload ...");

    println!("sizeByte: {byte:?}");
    println!("size: {apps_size}");

    let code: &[u8] = unsafe { 
      // core::slice::from_raw_parts(apps_start.offset(size_of::<u16>() as isize), apps_size as usize) 
      core::slice::from_raw_parts(apps_start.offset(size_of::<u32>() as isize), apps_size as usize) 
    };

    println!("content: {:?}: ", code);
    println!("Load payload ok!");
}
