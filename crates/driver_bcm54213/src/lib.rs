#![no_std]
#![allow(dead_code, unused)]

extern crate alloc;

mod consts;
use consts::*;
mod netspeed;

use alloc::string::String;
use alloc::{fmt, format};
use core::marker::PhantomData;
use core::ptr::{read_volatile, write_volatile, NonNull};
use log::{debug, info, trace};
use netspeed::LinkSpeed;

pub trait Bcm54213HalTraits {
    fn phys_to_virt(pa: usize) -> usize {
        pa
    }
    fn virt_to_phys(va: usize) -> usize {
        va
    }
    fn dma_alloc_pages(pages: usize) -> (usize, usize);

    fn dma_free_pages(vaddr: usize, pages: usize);

    fn mdelay(m_times: usize);

    fn current_time() -> usize;
}

pub struct Bcm54213NicDevice<A: Bcm54213HalTraits> {
    iobase_pa: usize,
    iobase_va: usize,
    link_speed: LinkSpeed,
    phantom: PhantomData<A>,
}

impl<A: Bcm54213HalTraits> Bcm54213NicDevice<A> {
    pub fn new(iobase_pa: usize) -> Self {
        let iobase_va = A::phys_to_virt(iobase_pa);
        let mut nic = Bcm54213NicDevice::<A> {
            iobase_pa,
            iobase_va,
            link_speed: LinkSpeed::NetDeviceSpeedUnknown, // TODO
            phantom: PhantomData,
        };
        nic.init();
        nic
    }

    pub fn init(&mut self) {
        info!("Init Bcm54213NicDevice");

        // u32 reg = sys_readl (SYS_REV_CTRL);	// read GENET HW version
        // u8 major = (reg >> 24 & 0x0f);
        // if (major == 6)
        //  major = 5;
        // else if (major == 5)
        //  major = 4;
        // else if (major == 0)
        //  major = 1;
        // if (major != GENET_V5)
        // {
        //  CLogger::Get ()->Write (FromBcm54213, LogError,
        //     "GENET version mismatch, got: %d, configured for: %d",
        //     (unsigned) major, GENET_V5);
        //
        //  return FALSE;
        // }
    }
}

// base on circle
pub trait CNetDevice {
    fn get_mac_address(&self) -> [u8; 6];
    fn get_link_speed(&self) -> String;
    fn get_net_device();
    fn is_send_frame_advisable() -> bool;
    fn is_link_up() -> bool;

    fn send_frame() -> bool;
    fn receive_frame() -> bool;

    fn update_phy() -> bool;
}

impl<A: Bcm54213HalTraits> CNetDevice for Bcm54213NicDevice<A> {
    fn get_mac_address(&self) -> [u8; 6] {
        trace!("get_mac_address");
        let mut ret: [u8; 6] = [0; 6];
        unsafe {
            // TODO CHANGE IT CORRECT
            let hi = read_volatile((self.iobase_va + 0) as *mut u32);
            let lo = read_volatile((self.iobase_va + 0) as *mut u32);
            ret[0] = (lo & 0xff) as u8;
            ret[1] = ((lo >> 8) & 0xff) as u8;
            ret[2] = ((lo >> 16) & 0xff) as u8;
            ret[3] = ((lo >> 24) & 0xff) as u8;
            ret[4] = (hi & 0xff) as u8;
            ret[5] = ((hi >> 8) & 0xff) as u8;
        }
        ret
    }
    fn get_link_speed(&self) -> String {
        trace!("get_link_speed");
        format!("{}", self.link_speed)
    }
    fn get_net_device() {
        trace!("get_net_device");
    }
    fn is_send_frame_advisable() -> bool {
        trace!("is_send_frame_advisable");
        true
    }
    fn is_link_up() -> bool {
        trace!("is_link_up");
        true
    }

    fn send_frame() -> bool {
        trace!("send_frame");
        true
    }
    fn receive_frame() -> bool {
        trace!("receive_frame");
        true
    }

    fn update_phy() -> bool {
        trace!("update_phy");
        false
    }
}
