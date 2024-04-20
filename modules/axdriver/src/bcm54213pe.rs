
// use axalloc::global_allocator;
// use axhal::mem::{phys_to_virt, virt_to_phys};
use axhal::time::{busy_wait, Duration, current_time};

use driver_net::bcm54213pe::Bcm54213peHal;
pub struct Bcm54213peHalImpl;

impl Bcm54213peHal for Bcm54213peHalImpl {
    fn ndelay(n_times:usize)
    {
        busy_wait(Duration::from_nanos(n_times.try_into().unwrap()));
    }
}
