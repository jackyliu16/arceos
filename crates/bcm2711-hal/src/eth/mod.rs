//! BCM54213PE Gigabit Ethernet driver
//!
//! This implementation was based on:
//! https://github.com/u-boot/u-boot/blob/master/drivers/net/bcmgenet.c

use crate::hal::blocking::delay::DelayUs;
use bcm2711_regs::genet::*;

pub use crate::eth::address::EthernetAddress;
pub use crate::eth::descriptor::Descriptor;
pub use crate::eth::phy::Status as PhyStatus;
pub use crate::eth::rx::RxPacket;
use core::marker::PhantomData;
use log::{debug, trace};

mod address;
mod descriptor;
mod dma;
mod mdio;
mod mii;
mod netif;
mod phy;
mod rx;
mod umac;

const GENET_V5: u8 = 5;

// Hw adds 2 bytes for IP alignment
const LEADING_PAD: usize = 2;

// Body(1500) + EH_SIZE(14) + VLANTAG(4) + BRCMTAG(6) + FCS(4) = 1528.
// 1536 is multiple of 256 bytes
pub const MAX_MTU_SIZE: usize = 1536;
pub const MIN_MTU_SIZE: usize = 60;
pub const RX_BUF_LENGTH: usize = 2048;
pub const TX_BUF_LENGTH: usize = 2048;

pub type Descriptors = [Descriptor; NUM_DMA_DESC];

pub struct Devices {
    pub sys: SYS,
    pub ext: EXT,
    pub intrl2_0: INTRL2_0,
    pub intrl2_1: INTRL2_1,
    pub rbuf: RBUF,
    pub umac: UMAC,
    pub hfb: HFB,
    pub hfb_regs: HFBREGS,
    pub mdio: MDIO,
    pub rdma: RXDMA,
    pub tdma: TXDMA,
}

impl Devices {
    pub fn new() -> Self {
        Devices {
            sys: SYS::new(),
            ext: EXT::new(),
            intrl2_0: INTRL2_0::new(),
            intrl2_1: INTRL2_1::new(),
            rbuf: RBUF::new(),
            umac: UMAC::new(),
            hfb: HFB::new(),
            hfb_regs: HFBREGS::new(),
            mdio: MDIO::new(),
            rdma: RXDMA::new(),
            tdma: TXDMA::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Error {
    WouldBlock,
    HwVersionNotSupported,
    HwError,
    HwDescError,
    Fragmented,
    Malformed,
    Exhausted,
    Dropped,
    TimedOut,
}

pub struct Eth<'rx, 'tx, A: Bcm54213peHal> {
    c_index: usize,
    rx_index: usize,
    tx_index: usize,
    dev: Devices,
    rx_mem: &'rx mut [Descriptor],
    tx_mem: &'tx mut [Descriptor],
    phantom: PhantomData<A>,
}

impl<'rx, 'tx, A: Bcm54213peHal> Eth<'rx, 'tx, A> {
    pub fn new<D: DelayUs<u32>>(
        devices: Devices,
        delay: &mut D,
        mac_address: EthernetAddress,
        rx_mem: &'rx mut [Descriptor],
        tx_mem: &'tx mut [Descriptor],
    ) -> Result<Self, Error> {
        trace!("CC");
        assert_eq!(rx_mem.len(), NUM_DMA_DESC);
        assert_eq!(tx_mem.len(), NUM_DMA_DESC);

        trace!("A: {:?}", devices.sys.as_ptr());
        // TODO https://github.com/u-boot/u-boot/blob/master/drivers/net/bcmgenet.c#L626
        let version_major = match devices
            .sys
            .rev_ctrl
            .get_field(sys::RevCtrl::Major::Read)
            .unwrap()
            .val()
        {
            6 => 5,
            _ => 0,
        };
        trace!("DD");
        let version_minor: u8 = devices
            .sys
            .rev_ctrl
            .get_field(sys::RevCtrl::Minor::Read)
            .unwrap()
            .val() as _;
        let version_phy: u8 = devices
            .sys
            .rev_ctrl
            .get_field(sys::RevCtrl::EPhy::Read)
            .unwrap()
            .val() as _;

        trace!("DD");
        if (version_major != GENET_V5) || (version_minor != 0) || (version_phy != 0) {
            return Err(Error::HwVersionNotSupported);
        }

        trace!("finish pre check");
        let mut eth = Eth {
            c_index: 0,
            rx_index: 0,
            tx_index: 0,
            dev: devices,
            rx_mem,
            tx_mem,
            phantom: PhantomData,
        };

        trace!("A");
        eth.mii_config();
        trace!("B");
        eth.umac_reset(delay);
        trace!("C");
        eth.mdio_reset();

        trace!("D");
        eth.umac_reset2(delay);
        trace!("E");
        eth.umac_reset(delay);
        trace!("F");
        eth.umac_init(delay);
        trace!("G");

        eth.umac_set_hw_addr(&mac_address);
        trace!("H");
        eth.umac_set_rx_mode(&mac_address);
        trace!("I");

        // Disable RX/TX DMA and flush TX queues
        eth.dma_disable(delay);
        trace!("J");

        eth.rx_ring_init();
        trace!("K");
        eth.rx_descs_init();
        trace!("L");
        eth.tx_ring_init();
        trace!("M");

        // Enable RX/TX DMA
        eth.dma_enable();
        trace!("N");

        let status = eth.phy_read_status()?;

        // Update MAC registers based on PHY property
        eth.mii_setup(&status);

        // Enable Rx/Tx
        eth.netif_start();

        Ok(eth)
    }

    pub fn status(&mut self) -> Result<PhyStatus, Error> {
        self.phy_read_status()
    }

    pub fn recv(&mut self) -> Result<RxPacket, Error> {
        self.dma_recv()
    }

    pub fn send<F: FnOnce(&mut [u8]) -> R, R>(&mut self, length: usize, f: F) -> Result<R, Error> {
        self.dma_send(length, f)
    }
}

pub trait Bcm54213peHal {
    fn ndelay(n_times: usize);
}
