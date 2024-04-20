extern crate bcm2711_hal as hal;

use crate::{EthernetAddress, NetBufPtr, NetDriverOps};
use arr_macro::arr;
use driver_common::{BaseDriverOps, DevError, DevResult, DeviceType};
use hal::bcm2711_regs::{mbox::MBOX, sys_timer::SysTimer};
use hal::eth::Eth as Bcm54213peDevice;
use hal::eth::{Descriptor, Descriptors, Devices};
use hal::timer::TimerExt;

pub struct Bcm54213peNic {
    device: Bcm54213peDevice,
}

unsafe impl Send for Bcm54213peNic {}
unsafe impl Sync for Bcm54213peNic {}

impl Bcm54213peNic {
    pub fn init() -> Self {
        let eth_devices = Devices::new();

        let rx_descriptors = unsafe {
            static mut RX_DESC: Descriptors = arr![Descriptor::zero(); 256];
            &mut RX_DESC[..]
        };

        let tx_descriptors = unsafe {
            static mut TX_DESC: Descriptors = arr![Descriptor::zero(); 256];
            &mut TX_DESC[..]
        };

        let sys_timer = SysTimer::new();
        let mut sys_counter = sys_timer.split().sys_counter;
        let mut mbox = Mailbox::new(MBOX::new());
        let mut eth = Bcm54213peDevice::new(
            eth_devices,
            &mut sys_counter,
            hal::eth::EthernetAddress::from(*get_mac_address(&mut mbox).mac_address()),
            rx_descriptors,
            tx_descriptors,
        )
        .unwrap();
        Self { device: eth }
    }
}

impl BaseDriverOps for Bcm54213peNic {
    fn device_name(&self) -> &str {
        "Bcm54213peNic"
    }
    fn device_type(&self) -> DeviceType {
        DeviceType::Net
    }
}

impl NetDriverOps for Bcm54213peNic {
    /// The ethernet address of the NIC.
    fn mac_address(&self) -> EthernetAddress {
        let mut mbox = Mailbox::new(MBOX::new());
        EthernetAddress::from(*get_mac_address(&mut mbox).mac_address())
    }

    /// Whether can transmit packets.
    fn can_transmit(&self) -> bool {
        true
    }

    /// Whether can receive packets.
    fn can_receive(&self) -> bool {
        true
    }

    /// Size of the receive queue.
    fn rx_queue_size(&self) -> usize {
        16
    }

    /// Size of the transmit queue.
    fn tx_queue_size(&self) -> usize {
        16
    }

    /// Gives back the `rx_buf` to the receive queue for later receiving.
    ///
    /// `rx_buf` should be the same as the one returned by
    /// [`NetDriverOps::receive`].
    fn recycle_rx_buffer(&mut self, rx_buf: NetBufPtr) -> DevResult {
        todo!()
    }

    /// Poll the transmit queue and gives back the buffers for previous transmiting.
    /// returns [`DevResult`].
    fn recycle_tx_buffers(&mut self) -> DevResult {
        Ok(())
    }

    /// Transmits a packet in the buffer to the network, without blocking,
    /// returns [`DevResult`].
    fn transmit(&mut self, tx_buf: NetBufPtr) -> DevResult {
        Ok(())
    }

    /// Receives a packet from the network and store it in the [`NetBuf`],
    /// returns the buffer.
    ///
    /// Before receiving, the driver should have already populated some buffers
    /// in the receive queue by [`NetDriverOps::recycle_rx_buffer`].
    ///
    /// If currently no incomming packets, returns an error with type
    /// [`DevError::Again`].
    fn receive(&mut self) -> DevResult<NetBufPtr> {
        Err(DevError::Unsupported)
    }

    /// Allocate a memory buffer of a specified size for network transmission,
    /// returns [`DevResult`]
    fn alloc_tx_buffer(&mut self, size: usize) -> DevResult<NetBufPtr> {
        Err(DevError::Unsupported)
    }
}

use hal::mailbox::{Channel, GetMacAddressRepr, Mailbox, RespMsg};
fn get_mac_address(mbox: &mut Mailbox) -> GetMacAddressRepr {
    let resp = mbox
        .call(Channel::Prop, &GetMacAddressRepr::default())
        .expect("MBox call()");

    if let RespMsg::GetMacAddress(repr) = resp {
        repr
    } else {
        panic!("Invalid response\n{:#?}", resp);
    }
}
