#![no_std]
#![allow(dead_code, unused)]

extern crate alloc;

mod consts;
use consts::*;
mod netspeed;

#[macro_use]
mod macros;

use alloc::string::String;
use alloc::{fmt, format};
use core::marker::PhantomData;
use core::ptr::{read_volatile, write_volatile, NonNull};
use log::{debug, error, info, trace};
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
    fn udelay(m_times: usize);

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
        debug!("Bcm54213NicDevice new");
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

    pub fn init(&mut self) -> bool {
        info!("Init Bcm54213NicDevice");

        // ref on circle:lib/bcm54213.cpp
        let mut reg = unsafe {
            read_volatile((ARM_BCM54213_BASE + GENET_SYS_OFF + SYS_REV_CTRL) as *const u32)
        };
        trace!("GENET HW version regs: {:0>32b}", reg);
        let major: u8 = (reg >> 24 & 0x0f) as u8;
        debug!("major: {:0>8b}", major);

        let major = match major {
            6 => 5,
            5 => 4,
            0 => 1,
            c => c,
        };
        if major != 5 {
            error!("GENET version mismatch, got: {major}, configured for: 5");
            return false;
        };

        assert!(((reg >> 16) & 0x0F) == 0); // minor version
        assert!((reg & 0xFFFF) == 0); // EPHY version

        unsafe {
            self.reset_umac();
            self.umac_reset2();
            self.init_umac();

            debug!("B");

            trace!("init_umac");
            reg = read_volatile_wrapper!(genet_io!("umac", UMAC_CMD)); // make sure we reflect the value of CRC_CMD_FWD
            debug!("B");
            let m_crc_fwd_en = !!(reg & (CMD_CRC_FWD as u32));

            debug!("B");
            let ret = self.set_hw_addr();
        }
        info!("Reach the end of Bcm54213NicDevice Init");
        true
    }

    unsafe fn reset_umac(&self) {
        trace!("reset_umac");
        write_volatile_wrapper!(0, SYS_RBUF_FLUSH_CTRL + GENET_SYS_OFF + ARM_BCM54213_BASE);

        debug!("A");
        A::udelay(10);

        // disable MAC while updating its registers
        debug!("A");
        write_volatile_wrapper!(0, UMAC_CMD + GENET_UMAC_OFF + ARM_BCM54213_BASE);
        // issue soft reset with (rg)mii loopback to ensure a stable rxclk
        debug!("A");
        write_volatile_wrapper!(
            CMD_SW_RESET | CMD_LCL_LOOP_EN,
            UMAC_CMD + GENET_UMAC_OFF + ARM_BCM54213_BASE
        );
        A::udelay(2);
        debug!("A");
        write_volatile_wrapper!(0, UMAC_CMD + GENET_UMAC_OFF + ARM_BCM54213_BASE);
        debug!("A");
    }

    unsafe fn umac_reset2(&self) {
        trace!("umac_reset2");
        let mut reg: u32 = read_volatile_wrapper!(genet_io!("sys", SYS_RBUF_FLUSH_CTRL));
        reg |= (1 << 1);
        write_volatile_wrapper!(0, genet_io!("sys", SYS_RBUF_FLUSH_CTRL));
        A::udelay(10);
        reg &= !(1 << 1);
        write_volatile_wrapper!(reg as usize, genet_io!("sys", SYS_RBUF_FLUSH_CTRL));
    }

    unsafe fn init_umac(&self) {
        trace!("init_umac");
        self.reset_umac();
        trace!("C");
        // clear tx/rx counter
        write_volatile_wrapper!(
            MIB_RESET_RX | MIB_RESET_TX | MIB_RESET_RUNT,
            genet_io!("umac", UMAC_MIB_CTRL)
        );
        trace!("C");
        write_volatile_wrapper!(0, genet_io!("umac", UMAC_MIB_CTRL));
        trace!("C");
        write_volatile_wrapper!(
            ENET_MAX_MTU_SIZE,
            (ARM_BCM54213_BASE + GENET_UMAC_OFF + UMAC_MAX_FRAME_LEN) as *mut u32
        );
        // init rx registers, enable ip header optimization
        trace!("C");
        let mut reg: usize = read_volatile_wrapper!(genet_io!("rbuf", RBUF_CTRL)) as usize;
        reg |= RBUF_ALIGN_2B;
        trace!("C");
        // write_volatile_wrapper!(reg, genet_io!("rbuf", RBUF_CTRL));
        // write_volatile_wrapper!(1, genet_io!("rbuf", RBUF_TBUF_SIZE_CTRL));

        trace!("C");
        // self.intr_disable();
        // intr_disable(); // TODO CHECK
        // Enable MDIO interrupts on GENET v3+
        // NOTE: MDIO interrupts do not work
        //intrl2_0_writel(UMAC_IRQ_MDIO_DONE | UMAC_IRQ_MDIO_ERROR, INTRL2_CPU_MASK_CLEAR);
    }

    unsafe fn intr_disable(&self) {
        trace!("intr_disable");
        // Mask all interrupts TODO maybe incorrect with docs
        write_volatile_wrapper!(0xFFFF_FFFF, genet_io!("intrl2_0", INTRL2_CPU_MASK_SET));
        write_volatile_wrapper!(0xFFFF_FFFF, genet_io!("intrl2_0", INTRL2_CPU_CLEAR));
        write_volatile_wrapper!(0xFFFF_FFFF, genet_io!("intrl2_1", INTRL2_CPU_MASK_SET));
        write_volatile_wrapper!(0xFFFF_FFFF, genet_io!("intrl2_1", INTRL2_CPU_CLEAR));
    }

    // ref on raspiberry:linux
    unsafe fn set_hw_addr(&self) {
        // write_volatile_wrapper!(xk)
        // TODO let's beg it could work
        // TODO MAC Address could be get from message box or somthing else.
        let mac0 = read_volatile_wrapper!(genet_io!("umac", UMAC_MAC0));
        let mac1 = read_volatile_wrapper!(genet_io!("umac", UMAC_MAC1));

        trace!("mac0: {mac0}");
        trace!("mac1: {mac1}");
    }
}
// int CBcm54213Device::set_hw_addr(void)
// {
// 	m_MACAddress.Set (MACAddress.Address);
//
// 	CString MACString;
// 	m_MACAddress.Format (&MACString);
// 	CLogger::Get ()->Write (FromBcm54213, LogDebug, "MAC address is %s",
// 				(const char *) MACString);
//
// 	umac_writel(  (MACAddress.Address[0] << 24)
// 		    | (MACAddress.Address[1] << 16)
// 		    | (MACAddress.Address[2] << 8)
// 		    |  MACAddress.Address[3], UMAC_MAC0);
// 	umac_writel((  MACAddress.Address[4] << 8)
// 		     | MACAddress.Address[5], UMAC_MAC1);
//
// 	return 0;
// }
//
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
