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
    pub memory_addr: u64,
    pub memory_size: u64, 
    pub mmio_regions: Vec<(String, String)>,
} 

impl DtbInfo {
    pub fn new(addr: u64, size: u64, regions: Vec<(String, String)>) -> Self {
        Self {
            memory_addr: addr,
            memory_size: size,
            mmio_regions: regions,
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
		assert!(device_type == "memory");
	}
    let reg = dtb.get_property("/memory", "reg").unwrap();
	let (start_slice, size_slice) = reg.split_at(core::mem::size_of::<u64>());
	let ram_start = u64::from_be_bytes(start_slice.try_into().unwrap());
	let ram_size = u64::from_be_bytes(size_slice.try_into().unwrap());

    let mut vec = Vec::new();
    for subnode in dtb.enum_subnodes("/soc") {
        let parts: Vec<_> = subnode.split('@').collect();
        if parts[0] != "virtio_mmio" {
            continue;
        }

        let path = alloc::format!("/soc/{}", subnode);
        let out = dtb.get_property(path.as_str(), "reg");
        vec.push((String::from(parts[1]), alloc::format!("{:x}", u32::from_be_bytes(out.unwrap()[12..16].try_into().unwrap()))));
    }
    Ok(DtbInfo { memory_addr: ram_start, memory_size: ram_size, mmio_regions: vec })
}

// TODO if should be clean?
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