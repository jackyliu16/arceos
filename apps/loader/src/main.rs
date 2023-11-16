
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
      core::slice::from_raw_parts(apps_start, size_of::<u16>())
    };
    let app_size_1 = u8::from_be_bytes([byte[0]]);
    let app_size_2 = u8::from_be_bytes([byte[1]]);
    println!("size 1: {app_size_1}, size 2: {app_size_2}");

    println!("Load payload ...");

    println!("sizeByte: {byte:?}");
    
// # ][  u16  ]  [  u16  ]
// # ][        4B        ] [ package 1 ]
// # ][        4B + size_of(package 1) ] [ NEXT ] 

    // let code: &[u8] = unsafe { 
    //   core::slice::from_raw_parts(apps_start.offset(size_of::<u32>() as isize), apps_size as usize) 
    // };

    println!("Code Size 1: {:?}",
      unsafe {
        core::slice::from_raw_parts(apps_start.offset(size_of::<u16>() as isize),
        app_size_1 as usize
      )}
    );

    println!("Code Size 2: {:?}",
      unsafe {
        core::slice::from_raw_parts(apps_start.offset((size_of::<u16>() + app_size_1 as usize) as isize),
        app_size_2 as usize
      )}
    );


    // println!("content: {:?}: ", code);
    println!("Load payload ok!");
}
