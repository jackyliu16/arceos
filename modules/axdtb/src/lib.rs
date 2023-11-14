#![no_std]
#![allow(unused_imports, unused_variables, dead_code)]
#![feature(error_in_core)]

extern crate alloc;

#[macro_use]
extern crate log;

use core::str;
use alloc::string::String;
use alloc::vec::Vec;
use core::result::Result;
use core::option::Option::{self, *};
use log::debug;
use hermit_dtb::Dtb;


pub struct DtbInfo {
    pub memory_addr: usize,
    pub memory_size: usize, 
    pub mmio_regions: Vec<(usize, usize)>,
} 

impl DtbInfo {
    pub fn new() -> Self {
        Self {
            memory_addr: 0,
            memory_size: 0,
            mmio_regions: Vec::new(),
        }
    }
}

pub fn parse_dtb(dtb_pa: usize) -> Result<DtbInfo, DtbError> {
    info!("parse_dtb: {dtb_pa:x}");
    let dtb = unsafe {
		Dtb::from_raw(dtb_pa as *const u8)
			.expect(".dtb file has invalid header")
	};

	if let Some(device_type) = dtb.get_property("/memory", "device_type") {
		let device_type = core::str::from_utf8(device_type)
			.unwrap()
			.trim_matches(char::from(0));
        info!("device_type: {device_type}");
		assert!(device_type == "memory");
	}
    let reg = dtb.get_property("/memory", "reg").unwrap();
	let (start_slice, size_slice) = reg.split_at(core::mem::size_of::<u64>());
	let ram_start = u64::from_be_bytes(start_slice.try_into().unwrap());
	let ram_size = u64::from_be_bytes(size_slice.try_into().unwrap());
    info!("{ram_start:x}, {ram_size:x}");

    let virtio_mmio = dtb.get_property("/soc", "virtio_mmio");


    info!("==================================================");
    for item in dtb.enum_properties("/") {
        info!("{item}");
    }
    info!("==================================================");
    for item in dtb.enum_subnodes("/") {
        info!("{item}");
        for subnode in dtb.enum_subnodes(item) {
            info!("\t{subnode}");
            let part: Vec<_> = subnode.split('@').collect();
            info!("{part:?}");
            let mut str = String::from("/");
            str.push_str(item);
            str.push_str("/");
            str.push_str(subnode);
            info!("str: {str}");
            for pro in dtb.enum_properties(str.as_str()) {
                info!("\t\t{pro}");
            }
        }
    }
    Ok(DtbInfo::new())
}

#[derive(Debug)]
pub enum DtbError {
    SomeError,
}
impl core::error::Error for DtbError {}
impl core::fmt::Display for DtbError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match *self {
            DtbError::SomeError => write!(f, "Failed to open device tree file"),
        }
    }
}