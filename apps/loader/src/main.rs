#![allow(dead_code, unused)]
#![feature(asm_const)]
#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

use core::mem::size_of;

#[cfg(feature = "axstd")]
use axstd::{print, println};
const PLASH_START: usize = 0x2200_0000;
const LOAD_START: usize  = 0x4010_0000;

use log::{debug, error, info, trace, warn};

use elf::abi::PT_LOAD;
use elf::endian::AnyEndian;
use elf::ElfBytes;
use elf::segment::ProgramHeader;

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    println!("RUN LOADER");
    let apps_start = PLASH_START as *const u8;
    println!("{:?}", unsafe { core::slice::from_raw_parts(apps_start, 32) });

    // BC there is different between only load text segment and load elf files

    // First: modify the original image
    //  - do not use rustcopy to shrink the elf image into text only binrary
    //  - load elf image in PLASH point just like we did to binrary image file.
    //  - use theseus ways to load it into mem.

    // // Gain NUM
    let byte_num = unsafe { core::slice::from_raw_parts(apps_start, size_of::<u8>()) };
    let app_num = u8::from_be_bytes([byte_num[0]]);
    println!("DETACT {app_num} app");

    let byte = unsafe { core::slice::from_raw_parts(apps_start, size_of::<u64>()) };
    for b in byte { println!("{:08b}", b); }
    println!("===");
    println!("{:x}", unsafe { u16::from_be_bytes([byte[0], byte[1]])});
    println!("{:x}", unsafe { u16::from_be_bytes([byte[2], byte[3]])});
    println!("{:x}", unsafe { u16::from_be_bytes([byte[4], byte[5]])});
    println!("{:x}", unsafe { u16::from_be_bytes([byte[6], byte[7]])});

    // Gain Each App Size
    let mut apps: [APP; MAX_APP_NUM] = [APP::empty(); MAX_APP_NUM];
    let byte_apps_sizes = unsafe {
        // NOTE: BC Rust Internal structure autocomplete will fill vacancy, thus u16 rather than u8
        core::slice::from_raw_parts(
            apps_start.offset(size_of::<u16>() as isize),
            app_num as usize * size_of::<u16>(),
        )
    };
    println!("=====");
    println!("{:x}", unsafe { u16::from_be_bytes([byte_apps_sizes[0], byte_apps_sizes[1]])});
    println!("{:?}", unsafe { u16::from_be_bytes([byte_apps_sizes[0], byte_apps_sizes[1]]) as usize});
    println!("=====");
    println!("{:?}", unsafe {apps_start.offset(size_of::<u16>() as isize)});
    println!("{:?}", app_num as usize * size_of::<u16>());
    println!("app sizes: {byte_apps_sizes:?}");

    let mut head_offset = size_of::<u16>() + app_num as usize * size_of::<u32>();
    for i in 0..app_num {
        let i = i as usize;
        apps[i] = unsafe {
            APP::new(
                apps_start.offset(head_offset as isize),
                u16::from_be_bytes([byte_apps_sizes[i * 2], byte_apps_sizes[i * 2 + 1]]) as usize,
            )
        };
        head_offset += apps[i].size;
    }

    println!("{apps:?}");

    // println!("{:?}", unsafe {
    //     core::slice::from_raw_parts(apps_start, 32)
    // });

    unsafe {
        init_app_page_table();
        switch_app_aspace();
    }


    // // LOAD APPLICATION
    for i in 0..app_num {
        println!("====================");
        println!("= START OF APP {i} size: {} =", apps[i as usize].size);
        println!("====================");
        let i = i as usize;
        let read_only_elf =
            unsafe { core::slice::from_raw_parts(apps[i].start_addr, apps[i].size) };
        // println!("{:04x}", read_only_elf);
        let mut a = 0;
        for b in read_only_elf { 
            print!("{:04x} ", b); 
            a = a + 1;
            if a > 100 {
                break;
            }
        }
        println!("");
        println!("{} {:x}", read_only_elf.len(), read_only_elf.len());
        // println!("====================================================");

        parse_and_load_elf_executable(apps[i].start_addr, read_only_elf);
    }
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
            start_addr: 0xdead as *const u8,
            size: 0,
        }
    }
}

impl core::fmt::Debug for APP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.size == 0 {
            return Ok(());
        }

        f.debug_struct("APP")
            .field("start_addr", &self.start_addr)
            .field("size", &self.size)
            .finish()
    }
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



fn parse_and_load_elf_executable(
    start_addr: *const u8,
    file_contents: &[u8],
) {
    println!("inside function parse and load elf executable");
    let elf_file = ElfBytes::<AnyEndian>::minimal_parse(file_contents).expect("Open test1");
    
    for header in elf_file.segments().unwrap().iter() {
        if (header.p_type == PT_LOAD) {
            println!("LOAD Segment");
            println!("{:?}", header);

            let read_start = start_addr as usize + header.p_offset as usize;
            let load_start = LOAD_START as usize + header.p_vaddr as usize;
            let read_only_app =
                unsafe { core::slice::from_raw_parts(read_start as *const u8, header.p_memsz as usize) };
                
            let load_app =
                unsafe { core::slice::from_raw_parts_mut(load_start as *mut u8, header.p_memsz as usize) };

            println!("load {read_start:x} {:08x} into {load_start:x}", header.p_memsz as usize);
            load_app.copy_from_slice(read_only_app);

            // println!("{:?}", unsafe { core::slice::from_raw_parts(read_start as *const u8, 32) });
            // println!("{:?}", unsafe { core::slice::from_raw_parts(load_start as *const u8, 32) });
            assert_eq!(
                unsafe { core::slice::from_raw_parts(read_start as *const u8, 32) }, 
                unsafe { core::slice::from_raw_parts(load_start as *const u8, 32) }
            );
        }
    }
    show_symbol_table(elf_file);
}

fn show_section_table(elf_file: ElfBytes<AnyEndian>) {
    println!("--------------------------------------------------");
    println!("# DISPLAY SECTION TABLE #");
    // Get the section header table alongside its string table
    let (shdrs_opt, strtab_opt) = elf_file 
        .section_headers_with_strtab()
        .expect("shdrs offsets should be valid");
    let (shdrs, strtab) = (
        shdrs_opt.expect("Should have shdrs"),
        strtab_opt.expect("Should have strtab")
    );
    println!("{:?}", shdrs);
    println!("{:?}", strtab);

    println!("==================================================");
    println!("# ITER #");
    for shdr in shdrs.iter() {
        println!("{:?}", strtab.get(shdr.sh_name as usize).expect("Failure to get section name"));
        println!("{:?}", shdr);
    }
}

fn show_symbol_table(elf_file: ElfBytes<AnyEndian>) {
    println!("--------------------------------------------------");
    println!("# DISPLAY SYMBOL TABLE #");
    // Get the section header table alongside its string table
    let (symtabs, strtabs) = elf_file 
        .symbol_table()
        .expect("shdrs offsets should be valid")
        .expect("shdrs offsets should be valid");

    println!("{:?}", symtabs);
    println!("{:?}", strtabs);

    println!("==================================================");
    println!("# ITER #");
    for sym in symtabs.iter() {
        println!("{:?}", strtabs.get(sym.st_name as usize).expect("Failure to get section name"));
        println!("{:?}", sym);
    }
}

