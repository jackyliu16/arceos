#![allow(dead_code, unused)]
#![feature(asm_const)]
#![cfg_attr(feature = "axstd", no_std)] #![cfg_attr(feature = "axstd", no_main)]

use core::default;
use core::mem::size_of;
use core::ops::Index;

cfg_if::cfg_if!{
    if #[cfg(feature = "axstd")] {
        use axstd::{print, println};
        use axstd::string::String;
        use axstd::vec::Vec;
        use axstd::collections::BTreeMap;
    }
}

const PLASH_START: usize = 0x2200_0000;
const LOAD_START: usize  = 0x4010_0000;

use elf::parse::ParseAt;
use elf::section::SectionHeader;
use log::{debug, error, info, trace, warn};

// ELF Format Cheatsheet: https://gist.github.com/x0nu11byt3/bcb35c3de461e5fb66173071a2379779
use elf::abi::PT_LOAD;
use elf::endian::AnyEndian;
use elf::ElfBytes;
use elf::segment::ProgramHeader;
use elf::symbol::Symbol;
use elf::abi::{ET_EXEC, ET_REL, ET_DYN, ET_CORE, ET_LOOS, ET_HIOS}; // ELF FILE TYPE
use elf::abi::{SHT_REL, SHT_RELA, SHT_PROGBITS, }; // SECTION TYPE
use elf::abi::{R_RISCV_CALL, R_RISCV_RELAX}; // RELOCATION TYPE
use elf::abi::{R_RISCV_HI20, R_RISCV_LO12_I, R_RISCV_64, R_RISCV_RELATIVE, R_RISCV_JUMP_SLOT}; // RELOCATION TYPE (relate low)
use elf::relocation::Rel;
use elf::file::Class::ELF64;

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

    // Gain NUM
    // [ 02 ] 00 [ 00 00 17 d0 ] [ 00 0d 6f 90 ]
    let byte_num = unsafe { core::slice::from_raw_parts(apps_start, size_of::<u8>()) };
    let app_num = u8::from_be_bytes([byte_num[0]]);
    println!("DETACT {app_num} app");

    // Gain Each App Size
    let mut apps: [APP; MAX_APP_NUM] = [APP::empty(); MAX_APP_NUM];
    let byte_apps_sizes = unsafe {
        // [ 02 ] 00 [ 00 00 17 d0 ] [ 00 0d 6f 90 ]
        core::slice::from_raw_parts(
            apps_start.offset((size_of::<u16>()) as isize),
            app_num as usize * size_of::<u32>(),
        )
    };
    println!("{:?}", unsafe {apps_start.offset(size_of::<u16>() as isize)});
    println!("{:?}", app_num as usize * size_of::<u16>());
    println!("app sizes: {byte_apps_sizes:?}");

    let mut head_offset = size_of::<u16>() + app_num as usize * size_of::<u32>();
    for i in 0..app_num {
        let i = i as usize;
        apps[i] = unsafe {
            APP::new(
                apps_start.offset(head_offset as isize),
                u32::from_be_bytes([
                    byte_apps_sizes[i * 2], 
                    byte_apps_sizes[i * 2 + 1],
                    byte_apps_sizes[i * 2 + 2],
                    byte_apps_sizes[i * 2 + 3],
                ]) as usize,
            )
        };
        head_offset += apps[i].size;
    }

    println!("{apps:?}");

    // let test = unsafe { core::slice::from_raw_parts(apps_start, 2000 * size_of::<u8>())}; 
    // println!("{}", size_of::<u8>());
    // let mut cnt = 0;
    // for b in test { 
    //     print!("{:02x}", b);
    //     if cnt == 1370 {
    //         print!("\t");
    //         cnt += 1;
    //     } else {
    //         cnt += 1;
    //     };
    // }
    // println!("");

    
    unsafe {
        init_app_page_table();
        switch_app_aspace();
    }

    /// all object file inside the mem
    let mut elfs: Vec<ElfBytes<AnyEndian>> = Vec::new();
    /// relocatable file collection
    let mut E: Vec<ElfBytes<AnyEndian>> = Vec::new();
    /// unresolved symbol
    let mut U: Vec<(String, Symbol, ElfBytes<AnyEndian>)> = Vec::new();
    /// Parsed symbols
    let mut D: Vec<(Symbol, ElfBytes<AnyEndian>)> = Vec::new();

    /// BLUE PRINT
    /// 
    /// 1. LOAD APPLICATION
    /// 2. SYMBOL RESOLUTION: make sure all symbol has been meet.
    /// 3. RELOCATION:
    ///     [1] RELOCATION SECTION: 
    ///         merges all sections of the same type into a new aggregate section of the same typecombine 
    ///         BC we only need  SHT_PROGBITS thus not care about others (copy).
    ///     [2] RELOCATE SYMBOL REFERENCES IN SECTIONS: 
    ///         Modify references to each symbol in code and data sections so that they point to the correct run-time address

    // because we need to comtains both redirectable object file and shared object file in mem at
    // the same time, which means we should load they separately in different location.
    // LOAD APPLICATION
    // NOTE: After this, the ld exectually know the size of text section and data section.
    // NOTE: this loop will do only one times, which demand that the file should be give in order.
    for i in 0..app_num {
        let i = i as usize; println!("===================="); println!("= START OF APP {i} size: {} =", apps[i as usize].size); println!("====================");

        let read_only_elf = unsafe { core::slice::from_raw_parts(apps[i].start_addr, apps[i].size) };
        let elf_file = ElfBytes::<AnyEndian>::minimal_parse(read_only_elf).expect("Could Not Load ELF File From MEM");

        // SYMBOL RESOLUTION
        match elf_file.ehdr.e_type {
            ET_REL => { println!("Detect ET_REL");
                let (symtabs, strtabs) = elf_file
                    .symbol_table()
                    .expect("Failure when parse symtabs from elf file")
                    .expect("Failure when parse symtabs from elf file");

                for sym in symtabs.iter() {
                    // Because the corresponding values are not publicly available, we have to gain all of it
                    let sym_name = strtabs.get(sym.st_name as usize).expect("Failure to get section name");
                    if sym_name == "" { continue; }

                    let elf_file = ElfBytes::<AnyEndian>::minimal_parse(read_only_elf).expect("Could Not Load ELF File From MEM");
                    if sym.is_undefined() {
                        info!("U: push {sym_name}");
                        U.push((String::from(sym_name), sym.clone(), elf_file))
                    } else {
                        info!("D: push {sym_name}");
                        D.push((sym, elf_file));
                    }  
                }
                let elf_file = ElfBytes::<AnyEndian>::minimal_parse(read_only_elf).expect("Could Not Load ELF File From MEM");
                elfs.push(elf_file);
            },
            ET_DYN => { println!("Detect ET_DYN");
                // test each elf file if it contains a unresolved symbols
                let mut remove = Vec::new();
                for i in 0..U.len() {
                    let sym_name = &U.get(i).unwrap();
                    if let Some(obj_sym) = elf_file_contains_symbol(&elf_file, sym_name.0.clone()) {
                        // detect symbol sucessed
                        info!("detect symbol {} sucessed", sym_name.0);
                        let elf_file = ElfBytes::<AnyEndian>::minimal_parse(read_only_elf).expect("Could Not Load ELF File From MEM");
                        elfs.push(elf_file);
                        remove.push(i);
                        let elf_file = ElfBytes::<AnyEndian>::minimal_parse(read_only_elf).expect("Could Not Load ELF File From MEM");
                        D.push((obj_sym, elf_file));
                    }
                }
                for idx in (0..remove.len()).rev() {
                    U.remove(idx);
                }
            },
            ET_EXEC | ET_CORE | ET_LOOS | ET_HIOS | _ => {
                error!("Input a file with unsupported type: {}", elf_file.ehdr.e_type);
            },
        }
    }

    // RELOCATION[1]: RELOCATE SECTION AND SYMBOL DEFINITIONS:
    //  NOTE: Currently, Seems there is no ways to merge section in diff elf_file and same name into one, we have to use this ways to gain it.
    //  Maybe in future it could be replace by some automatic Vec or others, but not now.
    let mut sec_text: Vec<(&ElfBytes<AnyEndian>, usize)> = Vec::new();
    let mut sec_data: Vec<(&ElfBytes<AnyEndian>, usize)> = Vec::new();

    /// pointer to the Last unallocated run-time address 
    let mut unallocated_pointer = LOAD_START;
    /// a map between (Elf, Section) to run-time address
    let section_start_addrs_map: BTreeMap<(ElfBytes<AnyEndian>, SectionHeader), usize> = BTreeMap::new();

    for i in 0..app_num {
        let i = i as usize;
        let elf_file = &elfs[i];

        let (shdrs_opt, strtab_opt) = elf_file 
            .section_headers_with_strtab()
            .expect("shdrs offsets should be valid");
        let (shdrs, strtab) = (
            shdrs_opt.expect("Should have shdrs"),
            strtab_opt.expect("Should have strtab")
        );

        // find which section should be relocated
        for shdr in shdrs.iter() {
            match shdr.sh_type {
                SHT_PROGBITS => { info!("SHT_PROGBITS"); // LOAD .data and .text into mem
                    if let Ok(name) = strtab.get(shdr.sh_name as usize) {
                        match name { 
                            ".text" => { sec_text.push((elf_file, shdr.sh_name as usize)); } 
                            ".data" => { sec_data.push((elf_file, shdr.sh_name as usize)); } 
                            _ => { trace!("Skip Section Name: {}", shdr.sh_type); },
                        }
                    } 
                },
                _ => { trace!("Skip Section TYPE: {}", shdr.sh_type); },
            }
        }
    }
    for sec in &sec_text {
        println!("{}", sec.1);
    }
    for i in 0..app_num {
        let i = i as usize;
        for (file, idx) in &sec_text {
            if let Some(sectab) = file.section_headers() {
                println!("idx: {idx}");
                println!("idx: {}", *idx);
                let sec = sectab.get(*idx).unwrap();
                let read_only_data = unsafe {
                    core::slice::from_raw_parts(apps[i].start_addr.offset(sec.sh_offset as isize), sec.sh_size as usize)
                };
                let load_areas = unsafe { 
                    core::slice::from_raw_parts_mut(unallocated_pointer as *mut u8, sec.sh_size as usize)
                };
                load_areas.copy_from_slice(read_only_data); 
                unallocated_pointer += sec.sh_size as usize;
            }
        }
    }


    // 2. RELOCATE SYMBOL REFERENCES IN SECTIONS
    for i in 0..app_num {
        let i = i as usize;
        // relocation

        let elf_file = &elfs[i];

        let (shdrs_opt, strtab_opt) = elf_file 
            .section_headers_with_strtab()
            .expect("shdrs offsets should be valid");
        let (shdrs, strtab) = (
            shdrs_opt.expect("Should have shdrs"),
            strtab_opt.expect("Should have strtab")
        );


        for shdr in shdrs.iter() {
            match shdr.sh_type {
                SHT_REL | SHT_RELA => { // Relocate Symbol References in sections
                    if let Ok(iter_rel) = elf_file.section_data_as_rels(&shdr) {
                        for rel in iter_rel {
                            debug!("REL");
                        }
                    }

                    if let Ok(iter_rela) = elf_file.section_data_as_relas(&shdr) {
                        for rela in iter_rela {
                            debug!("RELA");
                            trace!("{:?}", rela);
                            match rela.r_type {
                                R_RISCV_64 => { info!("R_RISCV_64"); },                 // 2
                                R_RISCV_RELATIVE => { info!("R_RISCV_RELATIVE"); },     // 3
                                R_RISCV_JUMP_SLOT => { info!("R_RISCV_JUMP_SLOT"); },   // 5
                                R_RISCV_CALL => { info!("R_RISCV_CALL"); },             // 18
                                _ => { debug!("UNKNOWN RELA TYPE: {}", rela.r_type); }
                            }
                        }
                    }
                },
                _ => {
                    trace!("SECTION TYPE {} NOT DETECT", shdr.sh_type);
                }
            }
        }
    }

    println!("+++++++++++++++++++++++++++++");
    // println!("elfs: {elfs:?}");
    println!("+++++++++++++++++++++++++++++");
    // println!("E: {E:?}");
    // println!("+++++++++++++++++++++++++++++");
    // println!("U: {U:?}");
    // println!("+++++++++++++++++++++++++++++");
    // println!("D: {D:?}");
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

            assert_eq!(
                unsafe { core::slice::from_raw_parts(read_start as *const u8, 32) }, 
                unsafe { core::slice::from_raw_parts(load_start as *const u8, 32) }
            );
        }
    }

    // find(elf_file);
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

/// A stupid ways to find symbol 
fn find(elf_file: ElfBytes<AnyEndian>) {
    println!("--------------------------------------------------");
    println!("# FIND SYMBOL puts #");
    // Get the section header table alongside its string table
    let (symtabs, strtabs) = elf_file 
        .symbol_table()
        .expect("shdrs offsets should be valid")
        .expect("shdrs offsets should be valid");

    for sym in symtabs.iter() {
        let name = strtabs.get(sym.st_name as usize).expect("Failure to get symbol name");
        // if we gain the symbol that we want
        if name == "puts" {
            println!("{:?}", sym);
        }
    }
}

fn elf_file_contains_symbol(elf_file: &ElfBytes<AnyEndian>, sym_name: String) -> Option<Symbol> {  
    let (symtabs, strtabs) = elf_file
        .symbol_table()
        .expect("Failure when parse symtabs from elf file")    
        .expect("Failure when parse symtabs from elf file");

    // FIXME: If the name is repeated
    // symtabs.get(sym.st_name as usize); // Conflict may arise outside a given context
    for sym in symtabs.iter() {
        match strtabs.get(sym.st_name as usize).unwrap() {
            sym_name => return Some(sym.clone()),
        }
    }
    None
}

fn relocate_symbol_references() {

}