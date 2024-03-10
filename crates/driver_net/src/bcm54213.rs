use crate::{NetBuf, NetBufPtr, NetDriverOps};
use core::marker::PhantomData;
pub use driver_bcm54213::{Bcm54213HalTraits, Bcm54213NicDevice, CNetDevice};
use driver_common::{BaseDriverOps, DevResult};

pub struct Bcm54213Nic<A>
where
    A: Bcm54213HalTraits,
{
    device: Bcm54213NicDevice<A>,
    phantom: PhantomData<A>,
}

impl<A> Bcm54213Nic<A>
where
    A: Bcm54213HalTraits,
{
    pub fn init(trait_impl: A) -> Self {
        let device = Bcm54213NicDevice::<A>::new(0); // TODO
        Self {
            device,
            phantom: PhantomData,
        }
    }
}

unsafe impl<A: Bcm54213HalTraits> Sync for Bcm54213Nic<A> {}
unsafe impl<A: Bcm54213HalTraits> Send for Bcm54213Nic<A> {}

impl<A> BaseDriverOps for Bcm54213Nic<A>
where
    A: Bcm54213HalTraits,
{
    fn device_name(&self) -> &str {
        "Bcm54213 Network Interface Card"
    }

    fn device_type(&self) -> driver_common::DeviceType {
        driver_common::DeviceType::Net
    }
}

impl<A> NetDriverOps for Bcm54213Nic<A>
where
    A: Bcm54213HalTraits,
{
    fn mac_address(&self) -> crate::EthernetAddress {
        crate::EthernetAddress(self.device.get_mac_address())
    }

    fn tx_queue_size(&self) -> usize {
        16
    }

    fn rx_queue_size(&self) -> usize {
        16
    }

    fn can_receive(&self) -> bool {
        true
    }

    fn can_transmit(&self) -> bool {
        true
    }

    fn alloc_tx_buffer(&mut self, size: usize) -> DevResult<NetBufPtr> {
        todo!()
    }

    fn recycle_rx_buffer(&mut self, rx_buf: NetBufPtr) -> DevResult {
        todo!()
    }

    fn recycle_tx_buffers(&mut self) -> DevResult {
        todo!()
    }

    fn transmit(&mut self, tx_buf: NetBufPtr) -> DevResult {
        todo!()
    }

    fn receive(&mut self) -> DevResult<NetBufPtr> {
        todo!()
    }
}
