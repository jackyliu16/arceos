extern crate bcm2711_hal as hal;

use crate::{EthernetAddress, NetBufPtr, NetDriverOps};
use driver_common::{BaseDriverOps, DevError, DevResult, DeviceType};
use hal::eth::Eth as Bcm54213peDevice;

use core::marker::PhantomData;
pub struct Bcm54213peNic {
    inner: usize,
}

unsafe impl Send for Bcm54213peNic {}
unsafe impl Sync for Bcm54213peNic {}

impl Bcm54213peNic {
    fn init() -> Self {
        Self { inner: 0 }
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
        todo!()
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
